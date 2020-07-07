use discord_rpc_client::Client;
use std::{env, thread, time};
use wmctrl;

fn main() {
    // Input game (doom or pb)
    let game_input = env::args().nth(1).expect("Requires at least one argument");

    // Create the client
    let mut drpc = Client::new(729549945408979065);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();
    loop {
        // Get list of windows
        let windows = wmctrl::get_windows();
        // if game_input = doom; then
        //     search for DOOM process
        // else if game_input = pb; then
        //     search for Project Brutality process
        // else
        //     search for gzdoom process
        // fi
        // TODO: Somehow filter out non-gzdoom windows like browsers or terminals that happen to have doom in the title.
        let game_process = if game_input == "doom" {
            wmctrl::utils::find_window_by_title(&windows, "DOOM").unwrap()
        } else if game_input == "pb" {
            wmctrl::utils::find_window_by_title(&windows, "Project Brutality").unwrap()
        } else {
            wmctrl::utils::find_window_by_title(&windows, "gzdoom").unwrap()
        };
        // Obtain window title
        let window_title = game_process.title();
        // Create vector out of title, using " - " as the delimiter
        let game_vec: Vec<&str> = window_title.split(" - ").collect();
        let level = game_vec[0];
        let game = game_vec[1];
        let status_game = ["Game: ".to_string(), game.to_string()].concat();
        let status_level = ["Level: ".to_string(), level.to_string()].concat();
        // Print the status to stdout
        println!("{}\n{}\n--------------------", status_game, status_level);

        // Get the icon
        let icon = match game {
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
