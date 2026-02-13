#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod views;

use crate::views::chat_view::{ChatView, MessageData};
use std::error::Error;

slint::include_modules!();

struct Terminal {
    id: i8,
    name: String,
    color: String,
}

struct App {
    terminals: Vec<Terminal>,
}

impl App {
    fn new(terminals: Vec<Terminal>) -> Self {
        Self { terminals }
    }

    fn find_terminal(&self, id: i8) -> Option<&Terminal> {
        self.terminals
            .iter()
            .find(|terminal| terminal.id == id as i8)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app_window = AppWindow::new()?;

    let app = App::new(vec![
        Terminal {
            id: 0,
            name: String::from("Camp1"),
            color: String::from("#25cc57"),
        },
        Terminal {
            id: 1,
            name: String::from("Camp2"),
            color: String::from("#7a3af2"),
        },
        Terminal {
            id: 2,
            name: String::from("Camp3"),
            color: String::from("#ed3b85"),
        },
        Terminal {
            id: 3,
            name: String::from("Camp4"),
            color: String::from("#dbed3b"),
        },
    ]);

    let chat_view = ChatView::new(app, &app_window);

    chat_view.add_messages(vec![
        MessageData {
            id: 0,
            message: String::from("oooooooooo"),
            time: String::from("14:01"),
        },
        MessageData {
            id: 1,
            message: String::from("ahhhhhhhhhhhhhhhhhhhhhhhhhhh"),
            time: String::from("14:01"),
        },
        MessageData {
            id: 2,
            message: String::from("eeeeehehehehe eeeeehehehehe eeeeehehehehe eeeeehehehehe"),
            time: String::from("14:01"),
        },
        MessageData {
            id: 3,
            message: String::from(
                "uuwuwuwuwuwuwuw  uuwuwuwuwuwuwuwuuwuwuwuwuwuwuw uuwuwuwuwuwuwuw uuwuwuwuwuwuwuw",
            ),
            time: String::from("14:01"),
        },
    ]);

    app_window.run()?;

    Ok(())
}
