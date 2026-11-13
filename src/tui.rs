use std::io::Result;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph},
};

use crate::wani::WaniService;

pub struct App {
    pub exit: bool,
    wani: WaniService,
}

impl App {
    pub async fn new() -> Self {
        let mut wani = WaniService::new();
        println!("Updating reviews...");
        wani.update_reviews().await;

        Self { exit: false, wani }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let bound = Block::bordered().title("wani-tui");
        let inner = bound.inner(frame.area());
        frame.render_widget(bound, frame.area());
        self.draw_reviews(frame, inner);
    }

    fn draw_reviews(&self, frame: &mut Frame, area: Rect) {
        let constraints: Vec<Constraint> = self
            .wani
            .state
            .subjects
            .iter()
            .map(|_| Constraint::Length(1))
            .collect();

        let rows = Layout::vertical(constraints).split(area);

        for (sub, &row) in self.wani.state.subjects.iter().zip(rows.iter()) {
            let p = Paragraph::new(sub.data.characters.as_str());
            frame.render_widget(p, row);
        }
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
