use bincode::config;
use chrono::Local;
use env_logger::{Builder, Env, Target};
use locusts::introduce_locusts;
use std::fs;
use std::io::Write;
use std::process::exit;
use std::sync::{Arc, Mutex};
use wwb::*;

static BINCODE_CONFIG: config::Configuration = config::standard();

/// Roll a d6 and return the result.
fn roll_d6() -> u8 {
    fastrand::u8(1..=6)
}

/// Load the game state from a file.
/// Note: If there is no path, the game will not be loaded.
fn load_game(path: &str) -> Game {
    if path.is_empty() {
        return Game::default();
    }
    match fs::read(path) {
        Ok(serialized_game) => {
            let (game, _): (Game, usize) =
                bincode::decode_from_slice(&serialized_game, BINCODE_CONFIG).unwrap();
            game
        }
        Err(_) => Game::default(),
    }
}

/// Save the game state to a file.
///
/// Note: If there is no path, the game will not be saved.
fn save_game(game: &Game, path: &str) {
    if path.is_empty() {
        return;
    }
    let serialized_game = bincode::encode_to_vec(game, BINCODE_CONFIG).unwrap();
    fs::write(path, serialized_game).unwrap();
}

fn main() {
    introduce_locusts();

    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        args[1].clone()
    } else {
        "".to_string()
    };

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

    let game = Arc::new(Mutex::new(load_game(&path)));

    // Set up Ctrl-C handler to print the game state before exiting.
    let game_clone = Arc::clone(&game);
    let path_clone = path.clone();
    ctrlc::set_handler(move || {
        let game = game_clone.lock().unwrap();
        println!("Game state at exit:");
        log::error!("{:#?}", *game);
        save_game(&game, &path_clone);
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    game_loop(&game, &path);
}

/// ### Game loop:
/// 1. Player rolls d6
/// 2. If not 5, next player's turn
/// 3. If 5, advance one space
/// 4. If players are on the same space that is NOT 0, move BOTH back to space 0
/// 5. If there was no collision, roll d6 again
/// 6. If not 5, go back to space 0
/// 7. If 5 again, nothing happens
/// 8. Next player's turn
///
fn game_loop(game: &Arc<Mutex<Game>>, path: &str) {
    loop {
        let mut game = game.lock().unwrap();
        game.turn_count += 1;

        // This will probably not work properly when the turn count goes above 2^32 on a 32-bit machine and 3^64 on a 64-bit machine.
        let current_player_number = game.turn_count as usize % PLAYER_COUNT;
        let mut current_space: u16 = game.players[current_player_number].current_space;

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
            save_game(&game, &path);
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
