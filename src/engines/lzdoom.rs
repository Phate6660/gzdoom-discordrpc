use discord_rpc_client::Client;
use std::{thread, time};
use window_titles::{Connection, ConnectionTrait};

pub fn lzdoom_info(connection: &Connection, game_input: String) -> (String, String) {
    // List of windows as vector with strings
    let windows: Vec<String> = connection.window_titles().unwrap();
    // Yes. I know. This code is bad. I don't care. It took so many hours of agony to get it to work.
    // Wait wait wait! I'm sorry, I didn't mean it! Don't go... ;-;
    match game_input.as_str() {
        "doom" => {
            let game = windows
                .iter()
                .find(|title| title.to_string().contains("DOOM"))
                .unwrap();
            let status_game = ["Game: ".to_string(), game.to_string()].concat();
            // Print the status to stdout
            println!("Engine: LZDoom\n{}\n--------------------", status_game);
            (game.to_string(), status_game)
        }
        "pb" => {
            let game = windows
                .iter()
                .find(|title| title.to_string().contains("Project Brutality"))
                .unwrap();
            let status_game = ["Game: ".to_string(), game.to_string()].concat();
            // Print the status to stdout
            println!("Engine: LZDoom\n{}\n--------------------", status_game);
            (game.to_string(), status_game)
        }
        _ => {
            // Generically search for LZDoom if wrong arg is supplied
            let game = windows
                .iter()
                .find(|title| title.to_string().contains("LZDoom"))
                .unwrap();
            let status_game = ["Game: ".to_string(), game.to_string()].concat();
            // Print the status to stdout
            println!("Engine: LZDoom\n{}\n--------------------", status_game);
            (game.to_string(), status_game)
        }
    }
}

pub fn lzdoom_run(mut drpc: Client, game: String, status_game: String) {
    // Set the icon
    let icon = "lz";

    // Set the engine
    let engine = "Engine: LZDoom";
    
    // Set the activity
    if let Err(why) = drpc.set_activity(|a| {
        a.details(engine)
            .state(status_game)
            .assets(|ass| ass.large_image(icon).large_text(game))
    }) {
        println!("Failed to set presence: {}", why);
    }

    // Loop every 15 seconds
    thread::sleep(time::Duration::from_secs(15));
    println!("program has looped\n-----------------");
}
