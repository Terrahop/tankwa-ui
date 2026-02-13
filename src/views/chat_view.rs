use crate::{AppWindow, ChatState, Message};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use slint::{Color, ComponentHandle, Model, SharedString, VecModel};
use std::{fs, rc::Rc};

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageData {
    id: String,
    name: String,
    message: String,
    color: String,
    #[serde(default)]
    timestamp: String,
}

pub struct ChatView {
    messages: Rc<VecModel<Message>>,
}

impl ChatView {
    pub fn new(app: &AppWindow) -> Self {
        let chatvew_global = app.global::<ChatState>();
        let message_model = Rc::new(slint::VecModel::<Message>::default());

        let messages: VecModel<Message> =
            ChatView::read_messages("data/message.json".to_string()).unwrap();

        for msg in messages.iter() {
            message_model.push(msg);
        }

        chatvew_global.set_messages(message_model.clone().into());

        let controller = Self {
            messages: message_model,
        };
        controller.init_callbacks(&app);

        controller
    }

    fn init_callbacks(&self, app: &AppWindow) {
        let chatvew_global = app.global::<ChatState>();
        let message_model = self.messages.clone();

        chatvew_global.on_send_message(move |text| {
            let new_msg = Message {
                id: "id".into(),
                timestamp: "14:20".into(),
                name: "You".into(),
                message: text,
                color: Color::from_rgb_u8(100, 200, 255),
            };
            message_model.push(new_msg);
        });
    }

    pub fn timestamp() -> String {
        Utc::now().format("%d/%m/%Y %H:%M").to_string()
    }

    pub fn read_messages(file_path: String) -> Result<VecModel<Message>, std::io::Error> {
        println!("reading from file {}", file_path);
        let contents = fs::read_to_string(file_path)?;

        let mut messages: Vec<MessageData> = serde_json::from_str(&contents)?;
        let slint_messages: VecModel<Message> = VecModel::default();

        for msg in &mut messages {
            slint_messages.push(Message {
                id: SharedString::from(&msg.id),
                message: SharedString::from(&msg.message),
                name: SharedString::from(&msg.name),
                timestamp: SharedString::from(ChatView::timestamp()),
                color: ChatView::parse_color(&msg.color),
            });
        }

        Ok(slint_messages)
    }

    pub fn parse_color(c: &str) -> Color {
        if c.starts_with('#') {
            let hex = &c[1..];
            match hex.len() {
                6 => {
                    if let Ok(rgb) = u32::from_str_radix(hex, 16) {
                        return Color::from_rgb_u8(
                            ((rgb >> 16) & 0xFF) as u8,
                            ((rgb >> 8) & 0xFF) as u8,
                            (rgb & 0xFF) as u8,
                        );
                    }
                }
                8 => {
                    if let Ok(argb) = u32::from_str_radix(hex, 16) {
                        return Color::from_argb_u8(
                            ((argb >> 24) & 0xFF) as u8,
                            ((argb >> 16) & 0xFF) as u8,
                            ((argb >> 8) & 0xFF) as u8,
                            (argb & 0xFF) as u8,
                        );
                    }
                }
                _ => {}
            }
        }

        Color::from_rgb_u8(0, 0, 0)
    }
}
