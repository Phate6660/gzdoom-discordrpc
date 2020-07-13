use discord_rpc_client::Client;
use std::{thread, time};
use window_titles::{Connection, ConnectionTrait};

pub fn gzdoom_info(connection: &Connection, game_input: String) -> (String, String, String) {
    // List of windows as vector with strings
    let windows: Vec<String> = connection.window_titles().unwrap();
    // Yes. I know. This code is bad. I don't care. It took so many hours of agony to get it to work.
    // Wait wait wait! I'm sorry, I didn't mean it! Don't go... ;-;
    match game_input.as_str() {
        "doom" => {
            let window = windows
                .iter()
                .find(|title| title.to_string().contains("- DOOM"))
                .unwrap();
            // "level - game" => ["level", "game"]
            let game_vec: Vec<&str> = window.split(" - ").collect();
            let level = game_vec[0];
            let game = game_vec[1];
            let status_game = ["Game: ".to_string(), game.to_string()].concat();
            let status_level = ["Level: ".to_string(), level.to_string()].concat();
            // Print the status to stdout
            println!("{}\n{}\n--------------------", status_game, status_level);
            // Return the variables
            (game.to_string(), status_game, status_level)
        }
        "pb" => {
            let window = windows
                .iter()
                .find(|title| title.to_string().contains("- Project Brutality"))
                .unwrap();
            let game_vec: Vec<&str> = window.split(" - ").collect();
            let level = game_vec[0];
            let game = game_vec[1];
            let status_game = ["Game: ".to_string(), game.to_string()].concat();
            let status_level = ["Level: ".to_string(), level.to_string()].concat();
            println!("{}\n{}\n--------------------", status_game, status_level);
            (game.to_string(), status_game, status_level)
        }
        _ => {
            // Generically search for gzdoom if wrong arg is supplied
            let window = windows
                .iter()
                .find(|title| title.to_string().contains("GZDoom"))
                .unwrap();
            let game_vec: Vec<&str> = window.split(" - ").collect();
            let level = game_vec[0];
            let game = game_vec[1];
            let status_game = ["Game: ".to_string(), game.to_string()].concat();
            let status_level = ["Level: ".to_string(), level.to_string()].concat();
            // Print the status to stdout
            println!("{}\n{}\n--------------------", status_game, status_level);
            (game.to_string(), status_game, status_level)
        }
    }
}

pub fn gzdoom_run(mut drpc: Client, game: String, status_game: String, status_level: String) {
    // Get the icon
    let icon = match game.as_str() {
        "DOOM Registered" => "doom",
        "DOOM 2: Hell on Earth" => "doom_ii",
        "DOOM 2: Unity Edition" => "doom_ii",
        "The Ultimate DOOM" => "ultimate_doom",
        "Brutal Doom: Project Brutality" => "pb_icon",
        "Project Brutality 3.0" => "pb_icon",
        _ => "gz",
    };

    let engine = "GZDoom";

    // Set the activity
    if let Err(why) = drpc.set_activity(|a| {
        a.details(status_game)
            .state(status_level)
            .assets(|ass| ass.large_image(icon).large_text(engine))
    }) {
        println!("Failed to set presence: {}", why);
    }

    // Loop every 15 seconds
    thread::sleep(time::Duration::from_secs(15));
    println!("program has looped");
}
