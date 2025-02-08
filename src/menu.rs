use color_eyre::eyre::Result;
use game::Game;
use ratatui::{ layout::{Constraint, Direction, Layout}, 
prelude::{Buffer, Rect}, style::{Color, Modifier, Style}, symbols::border, 
text::{Line, Span}, widgets::{Block, Borders, Padding, Paragraph, StatefulWidget, Widget}, 
DefaultTerminal, Frame,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::constant;

mod game;


pub struct Menu {
    pub title: String,
    pub chest: String,
    pub page: MenuPage,
    pub mm_options: Vec<&'static str>,
    pub selected: usize,
    pub highlighted: usize,
    pub game: Game
}

impl Default for Menu{
    fn default() -> Self {
        Menu { 
            title: " Coins ".to_string(), 
            chest: " 💰 ".to_string(), 
            page: MenuPage::Main,
            mm_options: vec!["Play - Human Mode", "Model", "Stats"],
            selected: 0,
            highlighted: 0,
            game : Game::default() }
    }
}

#[derive(Clone)]
pub enum MenuPage{
    Main,
    Play,
    Model,
    Stats
}

pub struct GameState {
    pub page: MenuPage,
    pub game: Game
}

impl StatefulWidget for &mut Menu{
    type State = GameState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        // draw block
        let title = Line::from(self.title.clone());


        // layout
        let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![
            Constraint::Percentage(12),
            Constraint::Percentage(88)
        ]).split(area);

        // for now just a simple block with a title
        let block = Block::bordered()
        .title(title.centered())
        .border_set(border::DOUBLE);


        let chest_text = Line::from(vec![
            Span::raw(" Max out your "),
            Span::styled(
                self.chest.clone(),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " treasure! ",
                Style::default()
                    .fg(Color::Rgb(185, 164, 56))
                    .add_modifier(Modifier::BOLD),
            )
        ]);

        let chest_display = Paragraph::new(chest_text)
        .centered()
        .block(Block::new().borders(Borders::TOP).padding(Padding::new(0, 0, 1, 0)));

        chest_display.render(layout[0], buf);


        block.render(area, buf);

        match state.page {
            MenuPage::Main => self.spawn_main_menu(layout[1], buf),
            MenuPage::Play => self.spawn_play_screen(layout[1], buf),
            _ => ()
        }
    }

}

impl Menu {
    pub fn draw(&mut self, frame: &mut Frame) {
        let page = self.page.clone();
        let game = self.game.clone();
        let state = &mut GameState {
            page,
            game
        };
        frame.render_stateful_widget(self, frame.area(), state);
    }

    pub fn spawn_main_menu(&mut self, area: Rect, buf: &mut Buffer) {

        // Coins ascii text
        let coins_text = Paragraph::new(constant::COINS)
        .centered()
        .block(Block::new().borders(Borders::ALL).padding(Padding::new(0, 0, 3, 0)));


        let menu_layout = area;
        let y_offset = menu_layout.y + menu_layout.height/2 +2;
        let spaced: u16 = 3;
        for (i, option) in self.mm_options.iter().enumerate() {

            let content = if i == self.highlighted {
                Span::styled(format!("▶ {}", option),Style::default().fg(Color::Rgb(134, 47, 172)).add_modifier(Modifier::BOLD))

            }
            else { Span::raw(*option)};

            let text_width = content.width() as u16;

            let start_x = menu_layout.x + (menu_layout.width - text_width) / 2;


            buf.set_span(start_x, y_offset + i as u16 * spaced, &content, menu_layout.width-2);
        }

        coins_text.render(area, buf);
    }

    pub fn spawn_play_screen(&mut self, area: Rect, buf: &mut Buffer) {
        // split screen horizontally into 2 with  the left one having 70% of the screen
        let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

        let horizontal_left = horizontal_layout[0];
        let horizontal_right = horizontal_layout[1];

        // the left portion would be  where the game happens, right is where the action value estimates are recorded
        // split left portion into two vertically, the top to hold the actual game and the bottom to hold the score, the steps left and steps taken and the game mode
        let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(horizontal_left);

        let left_top = vertical_layout[0];
        let left_bottom = vertical_layout[1];

        let start_game_color = if self.game.start {
            Color::Green
        } else {
            Color::Rgb(134, 47, 172)
        };

        let top_left_text = Paragraph::new(
            Line::from(vec![Span::styled("Game ", Style::default().fg(start_game_color))]))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::new()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL).border_style(Style::default().fg(start_game_color)));

        
        // check if game has started, if not display start text over game area
        if !self.game.start {
            let start_game_text = Span::styled("Hit 'S' to Start!",Style::default().fg(Color::Rgb(134, 47, 172)).add_modifier(Modifier::BOLD));

            let text_width = start_game_text.width() as u16;

            let start_x = left_top.x + (left_top.width - text_width) /2;
            let start_y = left_top.y + (left_top.height/2);

            buf.set_span(start_x, start_y, &start_game_text, left_top.width -2);
        }

        else {
            // spawn treasure chests
            // should be y centered, but slightly to the top
            // each treasure box would have its index + 1 under it indicating the number corresponding to it

            let y_start = left_top.y + (left_top.height/2) -2;
            let total_chests = self.game.chests.len();
            let spaced: u16 = 8;

            for (i, _) in self.game.chests.iter().enumerate() {
                let content = if i == self.game.highlighted_chest as usize {
                    (
                        Span::styled(format!("[ ❓ ]",),Style::default().fg(Color::Green)), 
                        Span::styled(format!(" {} ", i), Style::default().fg(Color::Green))
                    )
                } else {
                    (
                        Span::styled(format!("[ ❓ ]",),Style::default().fg(Color::Rgb(134, 47, 172)).add_modifier(Modifier::BOLD)),
                        Span::styled(format!(" {} ", i), Style::default().fg(Color::Rgb(134, 47, 172)).add_modifier(Modifier::BOLD))
                    )
                };

                let chest_content = content.0;
                let idx_content = content.1;

                let chest_width = chest_content.width();
                let total_width = (total_chests * chest_width) + ((total_chests - 1) * spaced as usize);
                let start_x = left_top.x + (left_top.width.saturating_sub(total_width as u16) / 2);

                let chest_x = start_x + i as u16 * (chest_width as u16 + spaced);

                // content
                buf.set_span(chest_x, y_start, &chest_content, left_top.width-1);
                
                // content index
                buf.set_span(chest_x + chest_width as u16/2 - 1, y_start+ 3, &idx_content,left_top.width-1);
            }


            let text_content = "\u{2B05} Use direction keys to navigate \u{2B95}  ...  Hit Enter( \u{21B5} ) to Select Chest";
            let text_width = text_content.len() as u16;

            let info_text = Span::styled(
                text_content,
                Style::default().fg(Color::Rgb(185, 164, 56)),
            );

            // Position at bottom and center horizontally
            let start_x = left_top.x + (left_top.width.saturating_sub(text_width) / 2);
            let start_y = left_top.y + left_top.height.saturating_sub(2);

            buf.set_span(start_x, start_y, &info_text, text_width);
        }

        // if started then user can hit chest numbers to get treasure - NEXT
        // check if chest has been selected, if yes then get score from probabilities
        // if chest has treasure, then update treasure chest with money emoji, else, with dust
        // then render
        // else if not selected, then treasure chests should have ? marks
        // then on selection, increase step count
        // if step count == max_steps then end of episode, store stats and display game over screen, asking to play again
        // if not move to main screen, if yes increment episode count and start again

        let bottom_left_text = Paragraph::new("Score Tab")
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::new().border_type(ratatui::widgets::BorderType::Rounded).borders(Borders::ALL));

        let right_text = Paragraph::new("Stats Screen")
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::new().border_type(ratatui::widgets::BorderType::Rounded).borders(Borders::ALL));

        // then simulate a game
        // on no action taken, show an overlay text saying start, player selects a treasure box, then based on the probability of gaining a treasure of that box
        // a reward is spat out, then the user's score is updated and his action value estimate is updated
        // this goes on until the steps are exhausted, indicating the end of the episode
        // when done, or game over, the stats are stored in a file, for record keeping
        // draw 8 treasure boxes, with question marks in them at the beginning
        // then if 

        top_left_text.render(left_top, buf);
        bottom_left_text.render(left_bottom, buf);
        right_text.render(horizontal_right, buf);

        // loop through
    }


    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {

            terminal.draw(|frame| {
                self.draw(frame)
            })?;

            if let Ok(event) = event::read() {

                match event {
                    Event::Key(KeyEvent { code: KeyCode::Down, .. }) => {
                        self.down();
                    }
                    Event::Key(KeyEvent { code: KeyCode::Up, .. }) => {
                        self.up();
                    }
                    Event::Key(KeyEvent { code: KeyCode::Enter, .. }) => {
                        self.select();
                    }
                    Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
                        match self.page {
                            MenuPage::Main => break Ok(()),
                            MenuPage::Play => {
                                self.page = MenuPage::Main;
                                self.game.end();
                            },
                            _ => ()
                        }
                    }
                    Event::Key(KeyEvent { code: KeyCode::Char('s'), .. }) => {
                        match self.page {
                            MenuPage::Play => self.game.start(),
                            _ => ()
                        };
                    }
                    Event::Key(KeyEvent { code: KeyCode::Left, .. }) => {
                        match self.page {
                            MenuPage::Play => {
                                self.game.move_left();
                            },
                            _ => ()
                        }
                    }
                    Event::Key(KeyEvent { code: KeyCode::Right, .. }) => {
                        match self.page {
                            MenuPage::Play => {
                                self.game.move_right();
                            },
                            _ => ()
                        }
                    }
                    _ => {
                        println!("Unhandled Key Event: {:?}", event);
                    }
                }
            }
        }
    }

    pub fn down(&mut self) {
        if self.highlighted != self.mm_options.len() -1 {
            self.highlighted +=1;
        }
    }

    pub fn up (&mut self) {
        if self.highlighted > 0{
            self.highlighted -=1;
        }
    }

    pub fn select(&mut self) {
        self.selected = self.highlighted;

        // transition menus
        self.next_menu()

        // So if PLay is selected, we transition straight to game screen
        // if model is selected, we transition to model menu screen, where you can play using model or train using model
        // if stats screen is selected, then we can view model metrics, progression of weights over episodes or steps of 20 intervals, compared to real values of actions
    }

    pub fn next_menu(&mut self) {
        self.page = match self.selected {
            0 => MenuPage::Play,
            1 => MenuPage::Model,
            2 => MenuPage::Stats,
            _ => MenuPage::Main
        }
    }
}