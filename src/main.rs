#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod views;

use crate::views::chat_view::ChatView;
use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let app_window = AppWindow::new()?;

    let _ = ChatView::new(&app_window);

    app_window.run()?;

    Ok(())
}
