use std::{fmt::Result, sync::mpsc::Receiver, thread, time::Duration};

use enigo::{Enigo, Key, KeyboardControllable};
use twitch_chat_wrapper::ChatMessage;

pub struct MainState {
    from_chat: Receiver<ChatMessage>,
    input_controller: Enigo,
    normal_wait: Duration,
}

impl MainState {
    pub fn new(from_chat: Receiver<ChatMessage>) -> Self {
        Self {
            from_chat,
            input_controller: Enigo::new(),
            normal_wait: Duration::from_millis(500),
        }
    }

    pub fn run(&mut self) {
        loop {
            if let Ok(chat_message) = self.from_chat.recv() {
                dbg!(&chat_message);
                match chat_message.message.to_lowercase().as_str() {
                    "down" => self.press_key(Key::DownArrow),
                    "up" => self.press_key(Key::UpArrow),
                    "left" => self.press_key(Key::LeftArrow),
                    "right" => self.press_key(Key::RightArrow),
                    _ => (),
                }
            }
        }
    }

    fn press_key(&mut self, key: Key) {
        self.input_controller.key_down(key);
        thread::sleep(self.normal_wait);
        self.input_controller.key_up(key);
    }
}
