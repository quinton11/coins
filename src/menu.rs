use color_eyre::eyre::Result;
use game::{Game, ScoreType};
use player::Player;
use ratatui::{ layout::{Constraint, Direction, Layout}, 
prelude::{Buffer, Rect}, style::{Color, Modifier, Style}, symbols::border, 
text::{Line, Span}, widgets::{Block, Borders, Padding, Paragraph, StatefulWidget, Widget}, 
DefaultTerminal, Frame,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::constant;

mod game;
mod player;


pub struct Menu {
    pub title: String,
    pub chest: String,
    pub page: MenuPage,
    pub mm_options: Vec<&'static str>,
    pub selected: usize,
    pub highlighted: usize,
    pub game: Game,
    pub player: Player
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
            game : Game::default(),
            player: Player::default() }
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
    pub game: Game,
    pub player: Player
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
        let player = self.player.clone();
        let state = &mut GameState {
            page,
            game,
            player
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

        // TODO: Show steps taken and max steps count
        // Then add episode end screen or box or something
        // Then store records in a file
        // Then implement model menu, with training 


        // if started then user can hit chest numbers to get treasure - NEXT
        // check if chest has been selected, if yes then get score from probabilities
        // if chest has treasure, then update treasure chest with money emoji, else, with dust
        // then render
        // else if not selected, then treasure chests should have ? marks
        // then on selection, increase step count
        // if step count == max_steps then end of episode, store stats and display game over screen, asking to play again
        // if not move to main screen, if yes increment episode count and start again

        // then simulate a game
        // on no action taken, show an overlay text saying start, player selects a treasure box, then based on the probability of gaining a treasure of that box
        // a reward is spat out, then the user's score is updated and his action value estimate is updated
        // this goes on until the steps are exhausted, indicating the end of the episode
        // when done, or game over, the stats are stored in a file, for record keeping

        self.spawn_score_screen(left_bottom, buf);

        self.spawn_game_screen(left_top, buf);

        self.spawn_stat_screen(horizontal_right, buf);

    }

    pub fn spawn_stat_screen(&mut self, area: Rect, buf: &mut Buffer) {

        let right_text = Paragraph::new("Stats Screen")
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::new().border_type(ratatui::widgets::BorderType::Rounded).borders(Borders::ALL));

        let margin_x = 2;
        let margin_y = 5;
        let spaced = 2;
        
        for (i, value) in self.player.value_estimates.iter().enumerate() {
            let content = if i == self.game.highlighted_chest as usize {
                (
                    Span::styled(format!("[ ❓ ]"), Style::default().fg(Color::Green)), 
                    Span::styled(format!(" {}  →  {}", i, value), Style::default().fg(Color::Green))
                )
            } else {
                (
                    Span::styled(format!("[ ❓ ]"), Style::default().fg(Color::Rgb(134, 47, 172)).add_modifier(Modifier::BOLD)),
                    Span::styled(format!(" {}  →  {}", i, value), Style::default().fg(Color::Rgb(134, 47, 172)).add_modifier(Modifier::BOLD))
                )
            };
        
            let chest_content = content.0;
            let idx_content = content.1;
        
            let chest_width = chest_content.width();
            let chest_height = 1;
        
            let chest_x = area.x + margin_x;
        
            let chest_y = area.y + margin_y + i as u16 * (chest_height as u16 + spaced);
        
            buf.set_span(chest_x, chest_y, &chest_content, chest_width as u16);
        
            buf.set_span(chest_x + chest_width as u16 + 1, chest_y, &idx_content, area.width - 1);
        }

        right_text.render(area, buf);
    }

    pub fn spawn_score_counts(&mut self, area: Rect, buf: &mut Buffer, idx: usize, text: String) {
        // calculate center, then spawn similarly to chests
        let fields = 5;
        let spaced: u16 = 4;
        let text_width = text.len() as u16;

        let y_start = area.y + (area.height/2) +1;


        let total_width = (fields * text_width) + ((fields - 1) * spaced );
        let start_x = area.x + (area.width.saturating_sub(total_width as u16) / 2);

        let chest_x = start_x + idx as u16 * (text_width as u16 + spaced);

        let info_text = Span::styled(
            text,
            Style::default().fg(Color::Rgb(185, 164, 56)),
        );

        buf.set_span(chest_x, y_start, &info_text, area.width-1);
    }


    pub fn spawn_score_screen(&mut self, area: Rect, buf: &mut Buffer) {
        let bottom_left_text = Paragraph::new("Score Tab")
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::new().border_type(ratatui::widgets::BorderType::Rounded).borders(Borders::ALL));

        // render scores in bottom left
        let score = Paragraph::new(Line::from(vec![
            Span::styled(format!("Gold 💰: {}", self.player.score), Style::default().fg(Color::Green))
        ])).alignment(ratatui::layout::Alignment::Left)
        .block(Block::new().borders(Borders::ALL).padding(Padding::new(2, 0, 0, 0)));

        score.render(area, buf);

        // Jackpot
        self.spawn_score_counts(area, buf, 0, format!("{} {}",ScoreType::Jackpot(0).to_string(), self.player.score_count.jackpot));

        // Treasure
        self.spawn_score_counts(area, buf, 1, format!("{} {}",ScoreType::Treasure(0).to_string(), self.player.score_count.treasure));

        // Bust
        self.spawn_score_counts(area, buf, 2, format!("{} {}",ScoreType::Bust(0).to_string(), self.player.score_count.bust));

        // Loss
        self.spawn_score_counts(area, buf, 3, format!("{} {}",ScoreType::Loss(0).to_string(), self.player.score_count.loss));

        // Robbed
        self.spawn_score_counts(area, buf, 4, format!("{} {}",ScoreType::Robbed(0).to_string(), self.player.score_count.robbed));

        bottom_left_text.render(area, buf);

    }

    pub fn spawn_game_screen(&mut self, area: Rect, buf: &mut Buffer) {

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

            let start_x = area.x + (area.width - text_width) /2;
            let start_y = area.y + (area.height/2);

            buf.set_span(start_x, start_y, &start_game_text, area.width -2);
        }

        else {
            let y_start = area.y + (area.height/2) -2;
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
                let start_x = area.x + (area.width.saturating_sub(total_width as u16) / 2);

                let chest_x = start_x + i as u16 * (chest_width as u16 + spaced);

                // content
                buf.set_span(chest_x, y_start, &chest_content, area.width-1);
                
                // content index
                buf.set_span(chest_x + chest_width as u16/2 - 1, y_start+ 3, &idx_content,area.width-1);
            }


            let text_content = "\u{2B05} Use direction keys to navigate \u{2B95}  ...  Hit Enter( \u{21B5} ) to Select Chest";
            let text_width = text_content.len() as u16;

            let info_text = Span::styled(
                text_content,
                Style::default().fg(Color::Rgb(185, 164, 56)),
            );

            // Position at bottom and center horizontally
            let start_x = area.x + (area.width.saturating_sub(text_width) / 2);
            let start_y = area.y + area.height.saturating_sub(2);

            buf.set_span(start_x, start_y, &info_text, text_width);


        }

        top_left_text.render(area, buf);

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
                        match self.page{
                            MenuPage::Main => self.select(),
                            MenuPage::Play => {
                                let highlighted_chest = self.game.highlighted_chest;
                                let score = self.game.get_chest_score();

                                self.player.update_estimate(highlighted_chest as usize, score);
                            },
                            _ => ()
                        }
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