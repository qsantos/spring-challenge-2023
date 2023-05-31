use std::process::Command;

use spring_challenge_2023::Game;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 3 {
        println!("!");
        return;
    }
    let bot1 = &args[1];
    let bot2 = &args[2];

    let game = Game::parse().unwrap();
    game.write();
}
