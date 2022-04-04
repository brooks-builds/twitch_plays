use std::{
    sync::mpsc::{channel, Sender},
    thread::{sleep, spawn, JoinHandle},
    vec,
};

use enigo::{Enigo, KeyboardControllable};

use crate::buttons::Button;

#[derive(Debug)]
pub struct ThreadPool {
    pub senders: Vec<Sender<Button>>,
    pub handles: Vec<JoinHandle<()>>,
    pub last_index: usize,
}

impl ThreadPool {
    pub fn new() -> Self {
        let mut senders = vec![];
        let mut handles = vec![];

        for _ in 0..5 {
            let (sender, receiver) = channel();

            senders.push(sender);

            let handle = spawn(move || {
                let mut enigo = Enigo::new();
                loop {
                    let button = if let Ok(button) = receiver.recv() {
                        button
                    } else {
                        continue;
                    };

                    if let Button::None = button {
                        continue;
                    }

                    enigo.key_down(button.into());
                    sleep(button.get_duration());
                    enigo.key_up(button.into());
                }
            });

            handles.push(handle);
        }

        let last_index = 0;

        Self {
            senders,
            handles,
            last_index,
        }
    }

    pub fn send(&mut self, button: Button) {
        self.senders[self.last_index].send(button).unwrap();
        self.last_index = if self.last_index >= self.senders.len() - 1 {
            0
        } else {
            self.last_index + 1
        };
    }
}
