use spring_challenge_2023::{Action, ActionLine, CellKind, Game};

fn main() {
    let mut game = Game::parse().unwrap();

    loop {
        game = game.read_update().unwrap();

        let mut action = Action::Wait;

        let allied_base = game.allied_bases[0];
        let closest_eggs = game.closest_cell(allied_base, CellKind::Eggs);
        let closest_crystals = game.closest_cell(allied_base, CellKind::Crystals);

        if let Some((distance, index)) = closest_eggs {
            if distance < 5 {
                action = Action::Line(ActionLine {
                    source: allied_base,
                    destination: index,
                    strength: 100,
                });
            }
        }

        if action == Action::Wait {
            if let Some((distance, index)) = closest_crystals {
                action = Action::Line(ActionLine {
                    source: allied_base,
                    destination: index,
                    strength: 100,
                });
            }
        }

        println!("{}", action);
    }
}
