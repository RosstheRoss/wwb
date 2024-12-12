use chrono::Local;
use env_logger::{Builder, Env, Target};
use locusts::introduce_locusts;
use std::io::Write;
use std::process::exit;
use std::sync::{Arc, Mutex};
use wwb::*;

/// Roll a d6 and return the result.
fn roll_d6() -> u8 {
    fastrand::u8(1..=6)
}

fn main() {
    introduce_locusts();
    let env = Env::new().filter_or("RUST_LOG", "info");
    // Create logger
    Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "{} - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.args()
            )
        })
        .target(Target::Stdout)
        .init();

    game_loop();
}

fn game_loop() {
    let game = Arc::new(Mutex::new(Game::default()));

    // Set up Ctrl-C handler to print the game state before exiting.
    let game_clone = Arc::clone(&game);
    ctrlc::set_handler(move || {
        let game = game_clone.lock().unwrap();
        println!("Game state at exit:");
        log::error!("{:#?}", *game);
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    loop {
        let mut game = game.lock().unwrap();
        game.turn_count += 1;
        // Game loop:
        // Player rolls d6
        // If not 5, next player's turn
        // If 5, roll d6 again
        // If not 5, go back to space 0
        // If 5 again, go ahead a space
        // If players are on the same space that is NOT 0, move BOTH back to space 0
        // Next player's turn
        // Roll the dice

        // This will probably not work properly when the turn count goes above 2^32 on a 32-bit machine and 3^64 on a 64-bit machine.
        let current_player_number = game.turn_count as usize % PLAYER_COUNT;
        let mut current_space = game.players[current_player_number].current_space;

        let roll = roll_d6();
        log::debug!(
            "Player {} rolled a {} on turn {} from space {}",
            current_player_number,
            roll,
            game.turn_count,
            current_space
        );

        if roll != 5 {
            continue;
        }
        current_space += 1;
        // Check if the player has won by reaching the space after the last space.
        if current_space == BOARD_SIZE + 1 {
            log::error!(
                "Player {} has reached the Finish space and won on turn {}!",
                current_player_number,
                game.turn_count
            );
            break;
        }

        if current_space > game.players[current_player_number].high_score {
            log::info!(
                "Player {} has a new high score of {} on turn {}",
                current_player_number,
                current_space,
                game.turn_count
                    .to_string()
                    .as_bytes()
                    .rchunks(3)
                    .rev()
                    .map(std::str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap()
                    .join(",")
            );
            game.players[current_player_number].high_score = current_space;
        }

        let mut collision: bool = false;

        for player in game.players.iter_mut() {
            if player.current_space == current_space && player.current_space != 0 {
                log::debug!("Two players collided on space {}!", current_space);
                player.current_space = 0;
                current_space = 0;
                collision = true;
            }
        }

        // No need to run this logic if there was a collision and the current player is already at 0.
        if !collision {
            if roll_d6() == 5 {
                log::debug!(
                    "Player {} rolled a 5 again on turn {} gets to stay on space {}",
                    current_player_number,
                    game.turn_count,
                    current_space
                );
            } else {
                log::debug!(
                    "Player {} rolled a non-5 after rolling a 5 on turn {} and goes back to Start",
                    current_player_number,
                    game.turn_count
                );
                current_space = 0;
            }
        }

        game.players[current_player_number].current_space = current_space;
        log::debug!(
            "Player {} ends turn {} on space {}",
            current_player_number,
            game.turn_count,
            current_space
        );
    }
}
