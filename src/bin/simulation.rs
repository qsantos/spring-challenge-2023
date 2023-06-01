use std::process::{Command, Stdio};

use spring_challenge_2023::Game;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} bot1 bot2", args[0]);
        return;
    }
    let bot1 = Command::new(&args[1])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start first bot");

    let bot2 = Command::new(&args[2])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start first bot");

    let mut bot1_input = &mut bot1.stdin.unwrap();
    let mut bot2_input = &mut bot2.stdin.unwrap();

    let game = Game::parse().unwrap();
    game.write(&mut bot1_input);
    game.write(&mut bot2_input);
}
