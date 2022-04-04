mod buttons;
mod command;
pub mod game_profiles;
mod thread_pool;

use game_profiles::Games;
use std::{
    process::exit,
    sync::mpsc::{Receiver, Sender},
};
use thread_pool::ThreadPool;
use twitch_chat_wrapper::ChatMessage;

use crate::command::Command;

#[derive(Debug)]
pub struct MainState {
    from_chat: Receiver<ChatMessage>,
    thread_pool: ThreadPool,
    game: Games,
    owner: String,
    to_chat: Sender<String>,
}

impl MainState {
    pub fn new(
        from_chat: Receiver<ChatMessage>,
        game: Games,
        owner: String,
        to_chat: Sender<String>,
    ) -> Self {
        let thread_pool = ThreadPool::new();

        Self {
            from_chat,
            game,
            owner,
            to_chat,
            thread_pool,
        }
    }

    pub fn run(&mut self) {
        self.to_chat
            .send(format!("We are playing {}", self.game))
            .unwrap();

        self.to_chat.send(self.game.help()).unwrap();

        loop {
            if let Ok(chat_message) = self.from_chat.recv() {
                let command = Command::from(chat_message.message);
                let sender = chat_message.name.trim().to_lowercase();
                self.handle_owner_command(&command, &sender);
                if let Some(button) = self.game.get_button(command) {
                    self.thread_pool.send(button);
                }
            }
        }
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
