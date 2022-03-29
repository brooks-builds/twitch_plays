mod command;
pub mod game_profiles;

use enigo::{Enigo, Key, KeyboardControllable};
use game_profiles::Games;
use std::{fmt::Result, process::exit, sync::mpsc::Receiver, thread, time::Duration};
use twitch_chat_wrapper::ChatMessage;

use crate::command::Command;

#[derive(Debug)]
pub struct MainState {
    from_chat: Receiver<ChatMessage>,
    input_controller: Enigo,
    normal_wait: Duration,
    game: Games,
    owner: String,
}

impl MainState {
    pub fn new(from_chat: Receiver<ChatMessage>, game: Games, owner: String) -> Self {
        Self {
            from_chat,
            input_controller: Enigo::new(),
            normal_wait: Duration::from_millis(2000),
            game,
            owner,
        }
    }

    pub fn run(&mut self) {
        println!("We are playing {}", &self.game);

        loop {
            if let Ok(chat_message) = self.from_chat.recv() {
                let command = Command::from(chat_message.message);
                let sender = chat_message.name.trim().to_lowercase();
                self.handle_owner_command(&command, &sender);
            }
        }
    }

    fn press_key(&mut self, key: Key) {
        dbg!("pressing", &key);
        self.input_controller.key_down(key);
        thread::sleep(self.normal_wait);
        self.input_controller.key_up(key);
    }

    fn handle_owner_command(&self, command: &Command, sender: &str) {
        if sender != self.owner {
            return;
        }

        match command {
            Command::Quit => exit(0),
            _ => (),
        }
    }
}
