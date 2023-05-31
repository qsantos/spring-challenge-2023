use std::process::Command;

use spring_challenge_2023::Game;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} bot1 bot2", args[0]);
        return;
    }
    let bot1 = Command::new(&args[1])
        .spawn()
        .expect("Failed to start first bot");

    let bot2 = Command::new(&args[2])
        .spawn()
        .expect("Failed to start first bot");

    let game = Game::parse().unwrap();
    game.write();
}
