use discord_rpc_client::Client;
use regex::Regex;
use std::{env, thread, time};
use window_titles::{Connection, ConnectionTrait};

// Returns the window title; for gzdoom this is "level - game", for lzdoom this is just "game"
fn info(connection: &Connection) -> Result<String, Box<dyn std::error::Error>> {
    // List of windows as vector with strings
    let windows: Vec<String> = connection.window_titles().unwrap();
    let r = Regex::new("(Project Brutality)|(DOOM)")?;
    let window: String = windows.into_iter().filter(|s| r.is_match(s)).collect();
    Ok(window)
}

fn main() {
    // Obtain engine from first arg since it would be near impossible to find correct one from the code
    let engine = env::args()
        .nth(1)
        .expect("Requires an engine to be input (gzdoom/lzdoom)");
    // Create the client
    let mut drpc = Client::new(729549945408979065);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Create connection so that window list may be obtained
    let connection = Connection::new().unwrap();

    loop {
        if engine == "gzdoom" {
            let window = info(&connection).unwrap();
            // "level - game" => [level, game]
            let game_vec: Vec<&str> = window.split(" - ").collect();
            let level = game_vec[0];
            let game = game_vec[1];
            let status_game = ["Game: ".to_string(), game.to_string()].concat();
            println!("Engine: GZDoom\n{}\nLevel: {}\n--------------------", status_game, level);

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
            let window = info(&connection).unwrap();
            let status_game = ["Game: ".to_string(), window.to_string()].concat();
            println!("Engine: LZDoom\n{}\n--------------------", status_game);

            let icon = "lz";

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
