# W.W.B.

A Rust simulation of the World's Worst Boardgame (W.W.B.) by Rich Hutnik.

## Disclaimer

I am not affiliated with Rich Hutnik or the World's Worst Boardgame in any way. This project is a fan-made simulation of the game and is not endorsed by the creator. I also do not verify the simulation is correct, as I have not played the game myself. I have read the rules and adapted the game to my understanding of the rules.

## What is W.W.B.?

[W.W.B.](https://boardgamegeek.com/boardgame/99918/wwb) is a "game" designed in 2011 by Rich Hutnik. It is a two-player game that is meant to be immensely frustrating and impossible to actually finish. The "board" is 102 cards, "Start", "Finish", and the numbers 1-100.

The game loop is as follows:

1. Player rolls d6
2. If not 5, next player's turn
3. If 5, advance one space
4. If players are on the same space that is NOT Start, move BOTH back to Start
5. If there was no collision, roll d6 again
6. If not 5, go back to Start
7. If 5 again, nothing happens
8. Next player's turn

## Why?

I wanted to learn Rust and this seemed like a fun project to work on.
A simulation of WWB has never been done either, the longest known one crashed at about 50 trillion turns with a high score of 17.

### My current record

Using this program, a computer managed to go at least 15 trillion turns overnight with a high score of 17. The game runs about 150-200 million turns a second on my Ryzen 9 5900X running Arch Linux on WSL. Running it on Windows on the same machine was about the same.

#### Exact info

The exact information of the game was on turn 15,071,820,303,760 Player 0 had a top score of 17 while Player 1 had a top score of 16.

## Usage

### Building

Building this project requires Rust and Cargo. You can install them using [rustup](https://rustup.rs/) or through your package manager.

To build the project, run:

```sh
cargo build --release
```

### Running

To run the project, run:

```sh
./target/release/wwb [savefile]
```

where `savefile` is an optional argument that specifies the file to save the game state to. If no savefile is provided, the game will start from the beginning and **NOT SAVE**.

## License

This project is licensed under the 0BSD license. See the [LICENSE](LICENSE) file for details.