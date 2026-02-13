use crate::{App, AppWindow, ChatState, Message};
use chrono::Utc;
use slint::{Color, ComponentHandle, SharedString, VecModel};
use std::rc::Rc;

pub struct MessageData {
    pub id: i8,
    pub message: String,
    pub time: String,
}

pub struct ChatView {
    app: App,
    messages: Rc<VecModel<Message>>,
}

impl ChatView {
    pub fn new(app: App, app_window: &AppWindow) -> Rc<Self> {
        let chatview_global = app_window.global::<ChatState>();
        let message_model = Rc::new(slint::VecModel::<Message>::default());

        chatview_global.set_messages(message_model.clone().into());

        let chat_view = Rc::new(Self {
            app,
            messages: message_model,
        });
        chat_view.init_callbacks(&app_window, Rc::clone(&chat_view));

        chat_view
    }

    fn init_callbacks(&self, app: &AppWindow, self_rc: Rc<Self>) {
        let chatvew_global = app.global::<ChatState>();
        let message_model = self.messages.clone();

        chatvew_global.on_send_message(move |text| {
            let new_msg = self_rc.new_message(&MessageData {
                id: 0,
                message: text.to_string(),
                time: ChatView::timestamp().into(),
            });
            if let Some(msg) = new_msg {
                message_model.push(msg.clone());
            }
        });
    }

    fn new_message(&self, message: &MessageData) -> Option<Message> {
        if let Some(terminal) = self.app.find_terminal(message.id as i8) {
            Some(Message {
                id: terminal.id as i32,
                message: SharedString::from(message.message.clone()),
                name: SharedString::from(&terminal.name),
                timestamp: SharedString::from(&message.time),
                color: ChatView::parse_color(&terminal.color),
            })
        } else {
            None
        }
    }

    fn parse_color(c: &str) -> Color {
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

    pub fn timestamp() -> String {
        Utc::now().format("%H:%M").to_string()
    }

    pub fn add_messages(&self, messages: Vec<MessageData>) {
        for message in messages {
            if let Some(message) = self.new_message(&message) {
                self.messages.push(message);
            }
        }
    }
}
