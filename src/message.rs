use crate::Message;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use slint::{Color, SharedString, VecModel};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageData {
    id: String,
    name: String,
    message: String,
    color: String,
    #[serde(default)]
    timestamp: String,
}

#[derive(Debug)]
pub struct ChatView {
    // messages: Vec<Message>,
}

impl ChatView {
    // pub fn new(messages: Vec<Message>) -> Self {
    //     Self { messages }
    // }

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
