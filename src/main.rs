use discord_rpc_client::Client;
use std::env;
use window_titles::{Connection, ConnectionTrait};

pub mod engines;
use engines::{
    gzdoom::{gzdoom_info, gzdoom_run},
    lzdoom::{lzdoom_info, lzdoom_run},
};

fn main() {
    // Input game (doom or pb)
    let engine = env::args()
        .nth(1)
        .expect("Requires an engine to be input (gzdoom/lzdoom)");
    let game_input = env::args()
        .nth(2)
        .expect("Requires a game to be input (doom/pb)");

    // Create the client
    let mut drpc = Client::new(729549945408979065);

    // Start up the client connection, so that we can actually send and receive stuff
    drpc.start();

    // Create connection so that window list may be obtained
    let connection = Connection::new().unwrap();
    loop {
        if engine == "gzdoom" {
            let (game, level, status_game) = gzdoom_info(&connection, game_input.clone());
            gzdoom_run(drpc.clone(), game, status_game, level);
        } else if engine == "lzdoom" {
            let (game, status_game) = lzdoom_info(&connection, game_input.clone());
            lzdoom_run(drpc.clone(), game, status_game);
        } else {
            println!("No valid engine was inputted.");
        }
    }
}
