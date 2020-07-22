use discord_rpc_client::Client;
use regex::Regex;
use std::{env, thread, time};
use window_titles::{Connection, ConnectionTrait};

fn info(connection: Connection) -> Result<String, Box<dyn std::error::Error>> {
    // List of windows as vector with strings
    let windows: Vec<String> = connection.window_titles().unwrap();
    let r = Regex::new("(Project Brutality)|(DOOM)")?;
    let window: String = windows.into_iter().filter(|s| r.is_match(s)).collect();
    Ok(window)
}

fn main() {
    let engine = env::args()
        .nth(1)
        .expect("Requires an engine to be input (gzdoom/lzdoom)");
    // Create the client
    let mut drpc = Client::new(729549945408979065);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Create connection so that window list may be obtained
    let connection = Connection::new().unwrap();
    let window = info(connection).unwrap();
    let window_clone = &window.clone();
    let window_second_clone = &window.clone();
    loop {
        if engine == "gzdoom" {
            let game_vec: Vec<&str> = window_clone.split(" - ").collect();
            let level = game_vec[0];
            let game = game_vec[1];
            let status_game = ["Game: ".to_string(), game.to_string()].concat();
            println!(
                "Engine: GZDoom\n{}\nLevel: {}\n--------------------",
                status_game, level
            );

            // Get the icon
            let icon = match game {
                "DOOM Registered" => "doom",
                "DOOM 2: Hell on Earth" => "doom_ii",
                "DOOM 2: Unity Edition" => "doom_ii",
                "The Ultimate DOOM" => "ultimate_doom",
                "Brutal Doom: Project Brutality" => "pb_icon",
                "Project Brutality 3.0" => "pb_icon",
                _ => "gz",
            };

            // Set the engine
            let engine = "Engine: GZDoom";

            // Set the activity
            if let Err(why) = drpc.set_activity(|a| {
                a.details(engine)
                    .state(status_game)
                    .assets(|ass| ass.large_image(icon).large_text(level))
            }) {
                println!("Failed to set presence: {}", why);
            }
        } else if engine == "lzdoom" {
            let status_game = ["Game: ".to_string(), window_second_clone.to_string()].concat();
            println!("Engine: LZDoom\n{}\n--------------------", status_game);

            // Set the icon
            let icon = "lz";

            // Set the engine
            let engine = "Engine: LZDoom";

            // Set the activity
            if let Err(why) = drpc.set_activity(|a| {
                a.details(engine)
                    .state(status_game)
                    .assets(|ass| ass.large_image(icon).large_text(&window))
            }) {
                println!("Failed to set presence: {}", why);
            }
        } else {
            println!("Invalid engine has been inputted.");
        }
        // Loop every 15 seconds
        thread::sleep(time::Duration::from_secs(15));
        println!("program has looped\n------------------");
    }
}
