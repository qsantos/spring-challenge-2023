use spring_challenge_2023::Game;

fn main() {
    let game = Game::parse().unwrap();
    game.write();
}
