mod app;

use color_eyre::Result;
use tokio;
use crossterm::event::{self, Event};
use ratatui::{DefaultTerminal, Frame};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = app::App::default().run(terminal).await;
    ratatui::restore();
    result
}