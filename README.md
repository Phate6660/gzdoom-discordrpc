## gzdoom-discordrpc

![license](https://img.shields.io/badge/license-public%20domain-green)

A discordrpc client for ZDoom written in Rust.

WIP(?) cross-platform support. It works on Linux and Windows, still need to test on MacOS. Use at own risk and all that yada yada.

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
6. Program loops every 15 secs, looping every second would be overkill

## Running

1. First make sure that GZDoom or LZDoom is running, if using GZDoom make sure to be inside of a level.
2. Download a prebuilt binary from the releases section.
3. Open a terminal (on Windows, CMD or PowerShell should work just fine) then...

ENGINE = `gzdoom` or `lzdoom`

GAME = `doom` or `pb`

Linux: `chmod +x gzdoom-discordrpc`, then `./gzdoom-discordrpc ENGINE GAME`

Windows: `.\gzdoom-discordrpc.exe ENGINE GAME`

From source:
1. `git clone https://github.com/Phate6660/gzdoom-discordrpc`
2. `cd gzdoom-discordrpc`
3. `cargo run -- ENGINE GAME`

## Screenshots

DOOM:

![DOOM](images/doom.png?raw=true "DOOM")

DOOM (with LZDoom):

![DOOM](images/doom_lz.png?raw=true "DOOM (with LZDoom)")

DOOM II:

![DOOM II](images/doom_ii.png?raw=true "DOOM II")

Project Brutality:

![Project Brutality](images/pb.png?raw=true "Project Brutality")
