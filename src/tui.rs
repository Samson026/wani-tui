use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, text::Text, widgets::Block};

#[derive(Default)]
pub struct App {
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let bound = Block::bordered().title("wani-tui");
        frame.render_widget(bound, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(event_key) if event_key.kind == KeyEventKind::Press => {
                match event_key.code {
                    KeyCode::Esc => self.exit = true,
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}
