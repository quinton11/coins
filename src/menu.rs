use std::{fs::{metadata, File, OpenOptions}, io::{BufRead, BufReader}, time::Duration};

use color_eyre::eyre::Result;
use game::{Game, GameMode, ScoreType};
use player::{Player, ScoreCount};
use ratatui::{ layout::{ Constraint, Direction, Layout}, 
prelude::{Buffer, Rect}, style::{Color, Modifier, Style}, symbols::border, 
text::{Line, Span}, widgets::{Axis,BarChart, Block, Borders, Chart, Padding, Paragraph, StatefulWidget, Widget}, 
DefaultTerminal, Frame,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use csv::{self, WriterBuilder};
use stats::{EpisodeEntry, StatRecords};

use crate::constant;

mod game;
mod player;
mod stats;


pub struct Menu {
    pub title: String,
    pub chest: String,
    pub page: MenuPage,
    pub mm_options: Vec<&'static str>,
    pub gm_over_options: Vec<&'static str>,
    pub selected: usize,
    pub highlighted: usize,
    pub game: Game,
    pub player: Player,
    pub saved: bool,
    pub records: StatRecords
}

impl Default for Menu{
    fn default() -> Self {
        Menu { 
            title: " Coins ".to_string(), 
            chest: " 💰 ".to_string(), 
            page: MenuPage::Main,
            mm_options: vec!["Play - Human Mode", "Model", "Stats"],
            gm_over_options: vec!["Continue", "Back to Menu"],
            selected: 0,
            highlighted: 0,
            game : Game::default(),
            player: Player::default(),
            saved: false,
            records: StatRecords::default() }
    }
}

#[derive(Clone)]
pub enum MenuPage{
    Main,
    Play,
    GameOver,
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
            MenuPage::Model => self.spawn_model_screen(layout[1], buf),
            MenuPage::Stats => self.spawn_stats_screen(layout[1], buf),
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


        if self.game.game_over() {
            self.spawn_game_over_screen(left_top, buf);
        }
        else {
            self.spawn_game_screen(left_top, buf);
        }


        self.spawn_score_screen(left_bottom, buf);


        self.spawn_stat_screen(horizontal_right, buf);
    }


    pub fn spawn_model_screen(&mut self, area: Rect, buf: &mut Buffer) {
        // model screen similar to play screen but mode is different
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


        if self.game.game_over() && self.game.train_run_done() {
            self.spawn_game_over_screen(left_top, buf);
        }
        else {
            self.spawn_game_screen(left_top, buf);
        }


        self.spawn_score_screen(left_bottom, buf);


        self.spawn_stat_screen(horizontal_right, buf);
    }


    pub fn spawn_stats_screen(&mut self, area: Rect, buf: &mut Buffer) {

        if !self.records.has_records {
            let no_records_text = Line::from(vec![
                Span::raw(" No Model Records To Display Yet 😞.. Visit the "),
                Span::styled(
                    "Train",
                    Style::default()
                        .fg(Color::Red)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(
                    " menu to generate some! "
                )
            ]);

            let no_records_display = Paragraph::new(no_records_text)
            .centered()
            .block(Block::new().borders(Borders::TOP).padding(Padding::new(0, 0, area.height/2, 0)));
    
            no_records_display.render(area, buf);
            return
        }

        let layouts = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

        let right = layouts[1];

        let right_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(right);

        let left = layouts[0];
        let right_top = right_layout[0];
        let right_bottom = right_layout[1];


        // Top Right - Show Outcome Breakdown
        let total_outcomes = 100;
        let outcome_data = self.records.normalized_outcome_breakdown();

        let outcome_bar_chart = BarChart::default()
            .block(Block::default().borders(Borders::ALL).title("Score Outcome Breakdown (%)"))
            .bar_width(13) 
            .data(&outcome_data) 
            .max(total_outcomes) 
            .style(Style::default().fg(Color::Green));


        // Informatics: Value Estimates Over Time
        let middle_text = Paragraph::new("Informatics")
        .centered()
        .block(Block::new().borders(Borders::ALL));

        // Left Section: Line Chart: Value Estimates Over Time
        let chart_data = self.records.prepare_value_estimates_chart();
        let x_bounds = chart_data.1;
        let y_bounds = chart_data.2;
        let line_chart = Chart::new(chart_data.0)
            .block(Block::default().borders(Borders::ALL).title("Value Estimates Over Time"))
            .x_axis(Axis::default()
            .title("Episode")
            .bounds(x_bounds)
            .labels(vec![
                Span::raw(format!("{}", x_bounds[0] as i32)), 
                Span::raw(format!("{}", ((x_bounds[1] - x_bounds[0]) * 0.25 + x_bounds[0]) as i32)),  // 25% mark
                Span::raw(format!("{}", ((x_bounds[1] - x_bounds[0]) * 0.50 + x_bounds[0]) as i32)),  // 50% (Mid)
                Span::raw(format!("{}", ((x_bounds[1] - x_bounds[0]) * 0.75 + x_bounds[0]) as i32)),  // 75% mark
                Span::raw(format!("{}", x_bounds[1] as i32))
            ])
            )
            .y_axis(Axis::default()
                .title("Estimate")
                .bounds(y_bounds)
                .labels(vec![
                    Span::raw(format!("{:.1}", y_bounds[0])),  // -2.5
                    Span::raw(format!("{:.1}", y_bounds[0] + (y_bounds[1] - y_bounds[0]) / 4.0)),  // -1.25
                    Span::raw(format!("{:.1}", y_bounds[0] + (y_bounds[1] - y_bounds[0]) / 2.0)),  // 0.0
                    Span::raw(format!("{:.1}", y_bounds[0] + 3.0 * (y_bounds[1] - y_bounds[0]) / 4.0)),  // 1.25
                    Span::raw(format!("{:.1}", y_bounds[1])),  // 2.5
                ])
            );


        // Bottom Right: - Bar Chart: Score Progress Over Time
        let bar_data = self.records.recent_score_progress();

    
        let bar_chart = ratatui::widgets::BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Score Progress (Recent Multiples of 5 Episodes)"))
        .data(&bar_data.iter().map(|(s, v)| (s.as_str(), *v)).collect::<Vec<_>>())
        .bar_width(13)  
        .max(50)
        .style(Style::default().fg(Color::Yellow));


        middle_text.render(left, buf);

        line_chart.render(left, buf);
        outcome_bar_chart.render(right_top, buf);
        bar_chart.render(right_bottom, buf);
    }

    pub fn spawn_game_over_screen(&mut self, area: Rect, buf: &mut Buffer) {
        // render game over text in center slightly to the top
        let game_over_text =  Paragraph::new(constant::GAME_OVER)
        .centered()
        .block(Block::new().borders(Borders::ALL).padding(Padding::new(0, 0, 3, 0)));

        // Then in actual center, render two menu selections
        // - Continue
        // - Back To Menu

        let continue_text = self.gm_over_options[0];
        let main_menu_text = self.gm_over_options[1];

        // draw continue text
        self.create_menu_span_in_layout(format!("👉🏾 'C' to {}", continue_text), 0, area, buf);

        // draw main menu text
        self.create_menu_span_in_layout(format!("❌ 'q' to {}", main_menu_text), 1, area, buf);


        game_over_text.render(area, buf);
    }

    pub fn create_menu_span_in_layout(&self, text: String, index: u16, area: Rect, buf: &mut Buffer) {

        let y_offset = area.y + area.height/2 +2;
        let spaced: u16 = 3;

        let content = Span:: styled(text,Style::default().fg(Color::Rgb(134, 47, 172)).add_modifier(Modifier::BOLD));

        let text_width = content.width() as u16;

        let start_x = area.x + (area.width - text_width) /2;

        buf.set_span(start_x, y_offset + index  * spaced, &content, area.width -2);
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

        let episodes_run = Paragraph::new(Line::from(vec![
            Span::styled(format!("Episodes Run 🏃🏽‍♂️: {}", self.game.episodes_run + self.game.episode), Style::default().fg(Color::Green))
        ])).alignment(ratatui::layout::Alignment::Right)
        .block(Block::new().borders(Borders::ALL).padding(Padding::new(0, 2, 0, 0)));

        score.render(area, buf);
        episodes_run.render(area, buf);

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

        let step_text = Paragraph::new(
            Line::from(vec![Span::styled(format!("Steps: {}", self.game.steps), Style::default().fg(start_game_color))]))
        .alignment(ratatui::layout::Alignment::Left)
        .block(Block::new()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL).border_style(Style::default().fg(start_game_color)));

        let episode_text = Paragraph::new(
            Line::from(vec![Span::styled(format!("Episode ⏳: {}", self.game.episode), Style::default().fg(start_game_color))]))
        .alignment(ratatui::layout::Alignment::Left)
        .block(Block::new()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL).padding(Padding::new(20, 0, 0, 0)).border_style(Style::default().fg(start_game_color)));

        let max_steps_text = Paragraph::new(
            Line::from(vec![Span::styled(format!("Max Steps: {}", self.game.max_steps), Style::default().fg(start_game_color))]))
        .alignment(ratatui::layout::Alignment::Right)
        .block(Block::new()
        .border_type(ratatui::widgets::BorderType::Rounded)
        .borders(Borders::ALL).border_style(Style::default().fg(start_game_color)));

        // show episodes in between steps and game text
        // show episodes run in score tab on the right

        
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
        step_text.render(area, buf);
        max_steps_text.render(area, buf);
        episode_text.render(area, buf);

    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {

            terminal.draw(|frame| {
                self.draw(frame)
            })?;

            self.handle_event()?;

            // if mode is model then do things differently
            if matches!(self.page, MenuPage::Model) {
                // if game has started, then run
                if !self.game.start {
                    continue;
                }

                if self.game.train_run_done(){
                    self.game.start = false;
                    self.game.reset_episode();
                    continue;
                }

                // check for action selections
                let selection = self.player.model_step(self.game.episodes_run as f32);
                self.game.highlighted_chest = selection;
                let score = self.game.get_chest_score();

                // step in environment
                self.player.update_estimate(selection as usize, score, self.game.episodes_run as f32);
                self.player.set_action_selection(selection, self.game.steps as usize);
                self.game.step();

                // check if game over
                if self.game.game_over() && !self.saved {
                    if let Err(e) = self.save_episode() {
                        println!("Error: {}", e);
                    } else {
                        self.saved = false;
                        self.game.step_episode();
                        self.game.reset_steps();
                        self.player.zero_out();
                    }
                }
            }
        }
    }

    pub fn handle_event(&mut self) -> Result<()> {

        if event::poll(Duration::from_millis(100))? { 
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Down => self.down(),
                    KeyCode::Up => self.up(),
                    KeyCode::Enter => self.handle_enter(),
                    KeyCode::Esc => self.handle_escape(),
                    KeyCode::Char('s') => if matches!(self.page, MenuPage::Play) || matches!(self.page, MenuPage::Model) { self.game.start(); },
                    KeyCode::Left => if matches!(self.page, MenuPage::Play) { self.game.move_left(); },
                    KeyCode::Right => if matches!(self.page, MenuPage::Play) { self.game.move_right(); },
                    KeyCode::Char('c') => self.handle_reset(),
                    KeyCode::Char('q') => self.handle_quit(),
                    _ => (),
                }
            }
        }
    
        Ok(())
    }


    fn handle_enter(&mut self) {
        match self.page {
            MenuPage::Main => self.select(),
            MenuPage::Play => {
                if self.game.game_over() {
                    return;
                }

                let highlighted_chest = self.game.highlighted_chest;
                let score = self.game.get_chest_score();

                self.player.update_estimate(highlighted_chest as usize, score, self.game.episodes_run as f32);
                self.player.set_action_selection(highlighted_chest, self.game.steps as usize);
                self.game.step();

                if self.game.game_over() && !self.saved {
                    if let Err(e) = self.save_episode() {
                        println!("Error: {}", e);
                    } else {
                        self.saved = true;
                    }
                }
            }
            _ => (),
        }
    }

    fn handle_escape(&mut self)  {
        match self.page {
            MenuPage::Main => {
                ratatui::restore();
                std::process::exit(0)
            },
            MenuPage::Play => {
                self.page = MenuPage::Main;
                self.game.end();
                self.player.zero_out_all();
            },
            MenuPage::Model => {
                self.page = MenuPage::Main;
                self.game.end();
                self.player.zero_out_all();
            }
            MenuPage::Stats => {
                self.page = MenuPage::Main;
            }
            _ => (),
        }
    }

    fn handle_reset(&mut self) {
        if matches!(self.page, MenuPage::Play) {
            self.game.end();
            self.player.zero_out();
            self.saved = false;
        }
    }

    fn handle_quit(&mut self) {
        if matches!(self.page, MenuPage::Play) {
            if !self.game.game_over() {
                return;
            }

            self.page = MenuPage::Main;
            self.game.end();
            self.player.zero_out_all();
            self.saved = false;
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
        self.next_menu();
        self.saved = false

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
        };

        self.game.mode = match self.selected {
            0 => game::GameMode::Human,
            1 => game::GameMode::AgentTrain,
            _ => game::GameMode::Human
        };

        // Fetch the latest stat
        match self.page {
            MenuPage::Play => {
                self.game.mode = GameMode::Human;
                let result =  self.fetch_latest_stat();

                match result {
                    Ok((done, stat)) => {
                        if done {
                            self.player.value_estimates = stat.estimates;
                            self.game.episodes_run = stat.episode as u8
                        }
                    },
                    Err(_) => ()
                }
            }
            MenuPage::Model => {
                // fetch latest stat for model
                // set model estimates and episode count
                self.game.mode = GameMode::AgentTrain;
                let result =  self.fetch_latest_stat();

                match result {
                    Ok((done, stat)) => {
                        if done {
                            self.player.value_estimates = stat.estimates;
                            self.game.episodes_run = stat.episode as u8;
                        }
                    },
                    Err(_) => ()
                }
            }
            MenuPage::Stats => {
                // load entries list, if none, set has_records to false
                self.game.mode = GameMode::AgentTrain;
                let result = self.fetch_stat_records();

                match result {
                    Ok(()) => (),
                    Err(_) => ()
                }
            }
            _ => ()
        }
    }

    pub fn save_episode(&mut self)  -> std::io::Result<()> {


        let filename = format!("{}_stats.csv",self.game.mode.to_string());
        let file_exists = metadata(filename.clone()).is_ok();


        let mut episode_id = 1;

        
        if file_exists {
            let file = File::open(filename.clone())?;
            let reader = BufReader::new(file);
    
            if let Some(last_line) = reader.lines().filter_map(Result::ok).last() {
                let parts: Vec<&str> = last_line.split(',').map(|s| s.trim()).collect();
                if let Some(last_episode_id) = parts.first() {
                    if let Ok(last_id) = last_episode_id.parse::<u32>() {
                        episode_id = last_id + 1;
                    }
                }
            }
        }


        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)?;

        let mut wtr = WriterBuilder::new()
            .has_headers(!file_exists)
            .from_writer(file);

        if !file_exists {
            wtr.write_record(&["Episode", "Score", "Steps", "Estimates", "BreakDown", "Action Selections"])?;
        }

        let json_score = serde_json::to_string(&self.player.score_count)?;
        let estimates_json = serde_json::to_string(&self.player.value_estimates)?;
        let actions_json = serde_json::to_string(&self.player.action_selections)?;

        wtr.write_record(&[
        episode_id.to_string(),
        self.player.score.to_string(),
        self.game.steps.to_string(),
        estimates_json,
        json_score,
        actions_json
        ])?; 

        wtr.flush()?;

        Ok(())
    }

    pub fn fetch_latest_stat(&mut self) -> std::io::Result<(bool, EpisodeEntry)> {
        let filename = format!("{}_stats.csv",self.game.mode.to_string());
        let file_exists = metadata(filename.clone()).is_ok();

        if !file_exists {
            return Ok((false, EpisodeEntry::default()))
        }

        let file = File::open(filename.clone())?;
        let reader = BufReader::new(file);

        if let Some(last_line) = reader.lines().filter_map(Result::ok).last() {
            let mut csv_reader = csv::ReaderBuilder::new()
                .has_headers(false)
                .flexible(true)
                .from_reader(last_line.as_bytes());
    
            if let Some(result) = csv_reader.records().next() {
                if let Ok(record) = result {
                    let episode = record[0].parse::<u32>().unwrap_or(0);
                    let score = record[1].parse::<i32>().unwrap_or(0);
                    let steps = record[2].parse::<u32>().unwrap_or(0);
    
                    let estimates: Vec<f32> = serde_json::from_str(&record[3]).unwrap_or_else(|e| {
                        println!("Error parsing estimates: {}, content: {:?}", e, &record[3]);
                        vec![0.0;8]
                    });
    
                    let breakdown: ScoreCount = serde_json::from_str(&record[4]).unwrap_or_else(|e| {
                        println!("Error parsing breakdown: {}, content: {:?}", e, &record[4]);
                        ScoreCount {
                            jackpot: 0,
                            treasure: 0,
                            bust: 0,
                            loss: 0,
                            robbed: 0,
                        }
                    });

                    let action_selections: Vec<u8> = serde_json::from_str(&record[5]).unwrap_or_else(|e| {
                        println!("Error parsing action selections: {}, content: {:?}",e, &record[5]);
                        vec![0;20]
                    });
    
                    let entry = EpisodeEntry {
                        episode,
                        score,
                        steps,
                        estimates,
                        breakdown,
                        learning_rate: 0.1,
                        action_selections
                    };

                    return Ok((true, entry));
                }
            }
        }

        Ok((false, EpisodeEntry::default()))
    }


    pub fn fetch_stat_records(&mut self) -> std::io::Result<()> {
        let filename = format!("{}_stats.csv",self.game.mode.to_string());
        let file_exists = metadata(filename.clone()).is_ok();

        if !file_exists {
            return Ok(())
        }

        let mut episode_entries: Vec<EpisodeEntry> = Vec::new();

        let file = File::open(filename.clone())?;

        let mut csv_reader = csv::ReaderBuilder::new()
        .has_headers(true) 
        .flexible(true)
        .from_reader(file);

        let mut row_count = 0;

        for result in csv_reader.records() {
            row_count += 1;

            if let Ok(record) = result {

                let episode = record[0].parse::<u32>().unwrap_or(0);
                let score = record[1].parse::<i32>().unwrap_or(0);
                let steps = record[2].parse::<u32>().unwrap_or(0);

                let estimates: Vec<f32> = serde_json::from_str(&record[3]).unwrap_or_else(|e| {
                    println!("Error parsing estimates at row {}: {}, content: {:?}", row_count, e, &record[3]);
                    vec![0.0; 8]
                });

                let breakdown: ScoreCount = serde_json::from_str(&record[4]).unwrap_or_else(|e| {
                    println!("Error parsing breakdown at row {}: {}, content: {:?}", row_count, e, &record[4]);
                    ScoreCount { jackpot: 0, treasure: 0, bust: 0, loss: 0, robbed: 0 }
                });

                let action_selections: Vec<u8> = serde_json::from_str(&record[5]).unwrap_or_else(|e| {
                    println!("Error parsing action selections at row {}: {}, content: {:?}", row_count, e, &record[5]);
                    vec![0; 20]
                });

                let entry = EpisodeEntry {
                    episode,
                    score,
                    steps,
                    estimates,
                    breakdown,
                    learning_rate: 0.1,
                    action_selections,
                };

                episode_entries.push(entry);
            }
        }

        self.records.entries = episode_entries;
        self.records.has_records = true;

        return Ok(())
    }
}