use discord_rpc_client::Client;
use std::{env, thread, time};
use window_titles::{Connection, ConnectionTrait};

fn info(connection: &Connection, game_input: String) -> (String, String, String) {    
    // List of windows as vector with strings
    let windows: Vec<String> = connection.window_titles().unwrap();
    // Yes. I know. This code is bad. I don't care. It took so many hours of agony to get it to work.
    // Wait wait wait! I'm sorry, I didn't mean it! Don't go... ;-;
    if game_input == "doom" {
        let window = windows
            .iter()
            .find(|title| {
                title
                    .to_string()
                    .contains("- DOOM".to_string().as_str())
            })
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
    } else if game_input == "pb" {
        let window = windows
            .iter()
            .find(|title| {
                title
                    .to_string()
                    .contains("- Project Brutality".to_string().as_str())
            })
            .unwrap();
        let game_vec: Vec<&str> = window.split(" - ").collect();
        let level = game_vec[0];
        let game = game_vec[1];
        let status_game = ["Game: ".to_string(), game.to_string()].concat();
        let status_level = ["Level: ".to_string(), level.to_string()].concat();
        println!("{}\n{}\n--------------------", status_game, status_level);
        (game.to_string(), status_game, status_level)
    } else {
        // Generically search for gzdoom if wrong arg is supplied
        let window = windows
            .iter()
            .find(|title| {
                title
                    .to_string()
                    .contains("gzdoom".to_string().as_str())
            })
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

fn main() {
    // Input game (doom or pb)
    let game_input = env::args().nth(1).expect("Requires at least one argument");

    // Create the client
    let mut drpc = Client::new(729549945408979065);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Create connection so that window list may be obtained
    let connection = Connection::new().unwrap();
    loop {
        let (game, status_game, status_level) = info(&connection, game_input.clone());
        let game_match = game.as_str();

        // Get the icon
        let icon = match game_match {
            "DOOM Registered" => "doom",
            "DOOM 2: Hell on Earth" => "doom_ii",
            "The Ultimate DOOM" => "ultimate_doom",
            "Project Brutality" => "pb_icon",
            "Project Brutality 3.0" => "pb_icon",
            _ => "icon",
        };

        // Set the activity
        if let Err(why) = drpc.set_activity(|a| {
            a.details(status_game)
                .state(status_level)
                .assets(|ass| ass.large_image(icon).large_text(game))
        }) {
            println!("Failed to set presence: {}", why);
        }

        // Loop every 15 seconds
        thread::sleep(time::Duration::from_secs(15));
        println!("program has looped");
    }
}
