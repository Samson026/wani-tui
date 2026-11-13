use std::{env, error::Error, fmt::format};

use reqwest::{Client, header::AUTHORIZATION};

use crate::tui::App;
mod tui;
mod wani;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut app = App::new().await;
    ratatui::run(|terminal| app.run(terminal))?;
    Ok(())
}
