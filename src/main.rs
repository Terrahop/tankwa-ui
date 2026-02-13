#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod message;

use slint::{SharedString, VecModel};
use std::{error::Error, rc::Rc};

use crate::message::ChatView;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    use slint::Model;
    let app_window = AppWindow::new()?;
    let app_weak = app_window.as_weak();

    // let _ = ChatView::new(vec![]);

    let messages: VecModel<Message> = ChatView::read_messages("data/message.json".to_string()).unwrap();
    let model = Rc::new(slint::VecModel::<Message>::default());
    app_window.set_messages(model.clone().into());

    for msg in messages.iter() {
        model.push(msg);
    }

    app_window.on_send_message(move |text| {
        println!("sending message {}", text);
        let new_msg = Message {
            id: uuid::Uuid::new_v4().to_string().into(),
            timestamp: SharedString::from(ChatView::timestamp()),
            name: "You".into(),
            message: text.into(),
            color: slint::Color::from_rgb_u8(0, 150, 255),
        };
        model.push(new_msg);
    });

    // messages.extend(messages.clone());

    // app_window.get
    // let mut tiles: Vec<Message> = app_window.get_memory_message().iter().collect();

    // ui.on_request_increase_value({
    //     let ui_handle = ui.as_weak();
    //     move || {
    //         let ui = ui_handle.unwrap();
    //         ui.set_counter(ui.get_counter() + 1);
    //     }
    // });

    app_window.run()?;

    Ok(())
}
