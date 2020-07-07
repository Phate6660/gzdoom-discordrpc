## gzdoom-discordrpc

![license](https://img.shields.io/badge/license-public%20domain-green)

A discordrpc client for GZDoom written in Rust.

I'm making this because:

1. I could not find a pre-existing DiscordRPC client for it
2. This is something I would actively use, which would make developing it that much more fun
3. I can actually visualize how I would go about it, unlike most other projects where I feel blind as a bat

## How it works

1. Program reads first argument
2. Program connects to Discord via RPC
3. It then looks for a doom process based on supplied argument, and parses it's window title ("level - game/mod")
4. It is separated into an &str vector
5. The icon will be the logo for the game/mod, the status will be the level, and the hover text will be the game/mod
6. Program loops every 15 secs or so, looping every second would be overkill

## Running

1. Ensure that the `wmctrl` package for your distro is installed, it is needed searching for the game window
2. Make sure gzdoom is already running, preferably in a level since some wads don't name the title menu
3. `cargo run -- doom` (vanilla DOOM, should work for all versions) or `cargo run -- pb` (Project Brutality)

## Screenshots

DOOM:

![DOOM](images/doom.png?raw=true "DOOM")

DOOM II:

![DOOM II](images/doom_ii.png?raw=true "DOOM II")

Project Brutality:

![Project Brutality](images/pb.png?raw=true "Project Brutality")
