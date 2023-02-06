# pong-core

## NOTE: this is the legacy version. the new version will be less focused on minimalism. this version is kept because I want to have a minimal example anyway.

A very lightweight implementation of the game Pong. I got this idea while working on [Nullrefino](https://github.com/duckfromdiscord/nullrefino); at the time I only wanted to write a frontend, and not waste time recreating all of the logic of Tetris. The point of pong-core is to provide the game logic for Pong, without any extra unneeded dependencies. The overarching idea for the entire pong-core project is to be able to port it to many different devices, and so size and dependencies are crucial.

The only dependency pong-core has as of now is the `rand` crate, used to decide direction for the Pong ball.