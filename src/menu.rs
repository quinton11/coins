use color_eyre::eyre::Result;
use ratatui::{ layout::{Constraint, Direction, Layout}, 
prelude::{Buffer, Rect}, style::{Color, Modifier, Style}, symbols::border, 
text::{Line, Span}, widgets::{Block, Borders, Padding, Paragraph, StatefulWidget, Widget}, 
DefaultTerminal, Frame,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::constant;


pub struct Menu {
    pub title: String,
    pub chest: String,
    pub page: MenuPage,
    pub mm_options: Vec<&'static str>,
    pub selected: usize
}

impl Default for Menu{
    fn default() -> Self {
        Menu { 
            title: " Coins ".to_string(), 
            chest: " 💰 ".to_string(), 
            page: MenuPage::Main,
            mm_options: vec!["Play - Human Mode", "Model", "Stats"],
            selected: 0 }
    }
}

#[derive(Clone)]
pub enum MenuPage{
    Main,
    Train,
    Stats
}

impl StatefulWidget for &mut Menu{
    type State = MenuPage;

    fn render(self, area: Rect, buf: &mut Buffer, _state: &mut Self::State) {
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


        // Coins ascii text
        let coins_text = Paragraph::new(constant::COINS)
        .centered()
        .block(Block::new().borders(Borders::ALL).padding(Padding::new(0, 0, 3, 0)));

        // menu drop down will be a list of the menus
        // we can move up and down which 
        // loop through options, if selected attach span to
        let menu_layout = layout[1];
        let y_offset = menu_layout.y + menu_layout.height/2 +2;
        let spaced: u16 = 3;
        for (i, option) in self.mm_options.iter().enumerate() {

            let content = if i == self.selected {
                Span::styled(format!("▶ {}", option),Style::default().fg(Color::Rgb(134, 47, 172)).add_modifier(Modifier::BOLD))

            } else { Span::raw(*option)};

            let text_width = content.width() as u16;

            let start_x = menu_layout.x + (menu_layout.width - text_width) / 2;


            buf.set_span(start_x, y_offset + i as u16 * spaced, &content, menu_layout.width-2);
        }

        chest_display.render(layout[0], buf);

        coins_text.render(layout[1], buf);


        block.render(area, buf);
    }
}

impl Menu {
    pub fn draw(&mut self, frame: &mut Frame) {
        let state_s = &mut self.page.clone();
        frame.render_stateful_widget(self, frame.area(), state_s);
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {

            terminal.draw(|frame| {
                self.draw(frame)
            })?;

            if let Ok(event) = event::read() {
                if let Event::Key(KeyEvent { code: KeyCode::Esc, .. }) = event {
                    break Ok(());
                }
                match event {
                    Event::Key(KeyEvent { code: KeyCode::Down, .. }) => {
                        self.down();
                    }
                    Event::Key(KeyEvent { code: KeyCode::Up, .. }) => {
                        self.up();
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn down(&mut self) {
        if self.selected != self.mm_options.len() -1 {
            self.selected +=1;
        }
    }

    pub fn up (&mut self) {
        if self.selected <= self.mm_options.len() -1{
            self.selected -=1;
        }
    }
}