use std::{sync::mpsc::channel, thread};

use enigo::{Enigo, Key, KeyboardControllable};
use eyre::Result;
use twitch_plays::MainState;

fn main() -> Result<()> {
    let (send_to_twitch, receive_from_app) = channel();
    let (send_to_app, receive_from_twitch) = channel();

    let twitchchat = thread::spawn(move || {
        twitch_chat_wrapper::run(receive_from_app, send_to_app).unwrap();
    });

    send_to_twitch.send("You can now play!".into())?;

    let mut main_state = MainState::new(receive_from_twitch);
    main_state.run();

    twitchchat.join().unwrap();

    Ok(())
}
