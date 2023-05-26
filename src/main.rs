use std::{
    fmt::Display,
    io::{self, BufRead, Lines},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
enum ParsingError {
    WrongNumberOfElements(String, usize, usize),
    NotAnInteger(String, ParseIntError),
    InvalidCellKind(i32),
    EarlyEndOfFile,
    IoError(io::Error),
}

fn parse_i32(s: &str) -> Result<i32, ParsingError> {
    s.trim()
        .parse::<i32>()
        .map_err(|e| ParsingError::NotAnInteger(s.to_string(), e))
}

fn parse_usize(s: &str) -> Result<usize, ParsingError> {
    s.trim()
        .parse::<usize>()
        .map_err(|e| ParsingError::NotAnInteger(s.to_string(), e))
}

fn next_line<T: BufRead>(lines: &mut Lines<T>) -> Result<String, ParsingError> {
    match lines.next() {
        None => Err(ParsingError::EarlyEndOfFile),
        Some(Err(e)) => Err(ParsingError::IoError(e)),
        Some(Ok(v)) => Ok(v),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CellKind {
    Empty = 0,
    Eggs = 1,
    Crystals = 2,
}

impl TryFrom<i32> for CellKind {
    type Error = ParsingError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            x if x == CellKind::Empty as i32 => Ok(CellKind::Empty),
            x if x == CellKind::Eggs as i32 => Ok(CellKind::Eggs),
            x if x == CellKind::Crystals as i32 => Ok(CellKind::Crystals),
            _ => Err(ParsingError::InvalidCellKind(value)),
        }
    }
}

#[derive(Clone, Debug)]
struct Cell {
    kind: CellKind,
    resources: i32,
    _neigh_1: i32,
    _neigh_2: i32,
    _neigh_3: i32,
    _neigh_4: i32,
    _neigh_5: i32,
    _neigh_6: i32,
    allied_ants: i32,
    ennemy_ants: i32,
}

impl FromStr for Cell {
    type Err = ParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs: Vec<i32> = s
            .split(" ")
            .map(|s| parse_i32(s))
            .collect::<Result<Vec<i32>, _>>()?;
        if inputs.len() != 8 {
            return Err(ParsingError::WrongNumberOfElements(
                s.to_string(),
                inputs.len(),
                8,
            ));
        }
        let base_cell = Cell {
            kind: inputs[0].try_into()?,
            resources: inputs[1],
            _neigh_1: inputs[2],
            _neigh_2: inputs[3],
            _neigh_3: inputs[4],
            _neigh_4: inputs[5],
            _neigh_5: inputs[6],
            _neigh_6: inputs[7],
            allied_ants: 0,
            ennemy_ants: 0,
        };
        Ok(base_cell)
    }
}

#[derive(Clone, Debug)]
struct Game {
    cells: Vec<Cell>,
    allied_bases: Vec<usize>,
    ennemy_bases: Vec<usize>,
}

impl Game {
    fn parse_bases(line: &str, count: usize) -> Result<Vec<usize>, ParsingError> {
        let ret = line
            .split(' ')
            .map(|s| parse_usize(s))
            .collect::<Result<Vec<usize>, _>>()?;
        if ret.len() != count {
            return Err(ParsingError::WrongNumberOfElements(
                line.to_string(),
                ret.len(),
                count,
            ));
        }
        Ok(ret)
    }

    fn parse<T: BufRead>(lines: &mut Lines<T>) -> Result<Game, ParsingError> {
        let number_of_cells = parse_usize(&next_line(lines)?)?;
        let mut cells = Vec::new();
        for _ in 0..number_of_cells {
            cells.push(next_line(lines)?.parse()?);
        }

        let number_of_bases = parse_usize(&next_line(lines)?)?;
        let allied_bases = Game::parse_bases(&next_line(lines)?, number_of_bases)?;
        let ennemy_bases = Game::parse_bases(&next_line(lines)?, number_of_bases)?;

        Ok(Game {
            cells,
            allied_bases,
            ennemy_bases,
        })
    }

    fn update<T: BufRead>(mut self, lines: &mut Lines<T>) -> Result<Game, ParsingError> {
        for cell in self.cells.iter_mut() {
            let line = next_line(lines)?;
            let inputs = line.split(" ").collect::<Vec<_>>();
            cell.resources = parse_i32(inputs[0])?;
            cell.allied_ants = parse_i32(inputs[1])?;
            cell.ennemy_ants = parse_i32(inputs[2])?;
        }
        Ok(self)
    }
}

struct ActionLine {
    source: usize,
    destination: usize,
    strength: i32,
}

struct ActionBeacon {
    location: usize,
    strength: i32,
}

struct ActionMessage {
    message: String,
}

enum Action {
    Wait,
    Line(ActionLine),
    Beacon(ActionBeacon),
    ActionMessage(ActionMessage),
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Wait => write!(f, "WAIT"),
            Action::Line(line) => write!(
                f,
                "LINE {} {} {}",
                line.source, line.destination, line.strength
            ),
            Action::Beacon(beacon) => write!(f, "BEACON {} {}", beacon.location, beacon.strength),
            Action::ActionMessage(message) => write!(f, "MESSAGE {}", message.message),
        }
    }
}

fn main() {
    let mut lines = io::stdin().lines();
    let mut game = Game::parse(&mut lines).unwrap();

    loop {
        game = game.update(&mut lines).unwrap();

        let mut action = Action::Wait;

        let allied_base = game.allied_bases[0];
        for (i, cell) in game.cells.iter().enumerate() {
            if cell.resources != 0 {
                action = Action::Line(ActionLine {
                    source: allied_base,
                    destination: i,
                    strength: 100,
                });
                break;
            }
        }

        println!("{}", action);
    }
}
