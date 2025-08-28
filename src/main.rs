mod UI;
mod chat;
mod helper;
mod network;
mod plugins;
mod service;
mod types;

use crate::UI::app::App;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
