use std::{env, io};
use std::{sync::mpsc::channel, thread};

use eyre::{bail, Result};
use twitch_plays::game_profiles::Games;
use twitch_plays::MainState;

fn main() -> Result<()> {
    dotenv::dotenv()?;
    print_instructions();
    let game = get_game()?;
    let (send_to_twitch, receive_from_app) = channel();
    let (send_to_app, receive_from_twitch) = channel();

    let twitchchat = thread::spawn(move || {
        twitch_chat_wrapper::run(receive_from_app, send_to_app).unwrap();
    });

    let owner = env::var("OWNER")?;
    let mut main_state = MainState::new(receive_from_twitch, game, owner, send_to_twitch);
    main_state.run();

    twitchchat.join().unwrap();

    Ok(())
}

fn print_instructions() {
    println!("What game are we playing?");
    println!("1 - Vampire Survivors");
}

fn get_game() -> Result<Games> {
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    Ok(match response.trim() {
        "1" => Games::VampireSurvivors,
        _ => bail!("Error, you must give a number that matches above"),
    })
}
