use std::{
    collections::{HashMap, HashSet, VecDeque},
    convert::{TryFrom, TryInto},
    fmt::Display,
    io::{self, Write},
    num::ParseIntError,
    str::FromStr,
};

#[derive(Debug)]
pub enum ParsingError {
    WrongNumberOfElements(String, usize, usize),
    NotAnInteger(String, ParseIntError),
    InvalidCellKind(i32),
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

fn next_line() -> Result<String, ParsingError> {
    let mut line = String::new();
    match io::stdin().read_line(&mut line) {
        Err(e) => Err(ParsingError::IoError(e)),
        Ok(_) => Ok(line),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CellKind {
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

impl ToString for CellKind {
    fn to_string(&self) -> String {
        (*self as i32).to_string()
    }
}

#[derive(Clone, Debug)]
pub struct Cell {
    kind: CellKind,
    resources: i32,
    neighbors: Vec<usize>,
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
            neighbors: inputs[2..=7]
                .into_iter()
                .copied()
                .filter(|&v| v >= 0)
                .map(|v| usize::try_from(v).unwrap())
                .collect(),
            allied_ants: 0,
            ennemy_ants: 0,
        };
        Ok(base_cell)
    }
}

impl ToString for Cell {
    fn to_string(&self) -> String {
        // TODO: there must be a better way
        let mut neighbors = [-1i32; 6];
        for (source, destination) in self.neighbors.iter().zip(neighbors.iter_mut()) {
            *destination = *source as i32;
        }
        let neighbors: Vec<String> = neighbors.iter().map(|x| x.to_string()).collect();
        format!(
            "{} {} {}",
            self.kind.to_string(),
            self.resources,
            neighbors.join(" "),
        )
    }
}

#[derive(Clone, Debug)]
pub struct Game {
    pub cells: Vec<Cell>,
    pub allied_bases: Vec<usize>,
    pub ennemy_bases: Vec<usize>,
}

struct MoveAssignment {
    source: usize,
    destination: usize,
    amount: i32,
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

    pub fn parse() -> Result<Game, ParsingError> {
        let number_of_cells = parse_usize(&next_line()?)?;
        let mut cells = Vec::new();
        for _ in 0..number_of_cells {
            cells.push(next_line()?.parse()?);
        }

        let number_of_bases = parse_usize(&next_line()?)?;
        let allied_bases = Game::parse_bases(&next_line()?, number_of_bases)?;
        let ennemy_bases = Game::parse_bases(&next_line()?, number_of_bases)?;

        Ok(Game {
            cells,
            allied_bases,
            ennemy_bases,
        })
    }

    pub fn write_bases<T: Write>(writer: &mut T, bases: &Vec<usize>) {
        write!(
            writer,
            "{}\n",
            bases
                .iter()
                .map(|base| base.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
        .unwrap();
    }

    pub fn write<T: Write>(&self, writer: &mut T) {
        write!(writer, "{}\n", self.cells.len()).unwrap();
        for cell in &self.cells {
            write!(writer, "{}\n", cell.to_string()).unwrap();
        }
        write!(writer, "{}\n", self.allied_bases.len()).unwrap();
        Self::write_bases(writer, &self.allied_bases);
        Self::write_bases(writer, &self.ennemy_bases);
    }

    pub fn read_update(mut self) -> Result<Game, ParsingError> {
        for cell in self.cells.iter_mut() {
            let line = next_line()?;
            let inputs = line.split(" ").collect::<Vec<_>>();
            cell.resources = parse_i32(inputs[0])?;
            cell.allied_ants = parse_i32(inputs[1])?;
            cell.ennemy_ants = parse_i32(inputs[2])?;
        }
        Ok(self)
    }

    pub fn write_update<T: Write>(&self, writer: &mut T) {
        for cell in self.cells.iter() {
            write!(
                writer,
                "{} {} {}",
                cell.resources, cell.allied_ants, cell.ennemy_ants
            )
            .unwrap();
        }
    }

    fn path(&self, source: usize, destination: usize) -> Vec<usize> {
        let mut previous = HashMap::new();
        let mut q = VecDeque::new();
        q.push_back((0, source));
        while let Some((distance, state)) = q.pop_front() {
            if state == destination {
                let mut path = Vec::new();
                let mut current = state;
                path.push(current);
                while let Some(&previous) = previous.get(&current) {
                    current = previous;
                    path.push(current);
                }
                path.reverse();
                return path;
            }

            let cell = &self.cells[state];
            for &neighbor in &cell.neighbors {
                if previous.contains_key(&neighbor) {
                    continue;
                }
                previous.insert(neighbor, state);

                q.push_back((distance + 1, neighbor));
            }
        }
        unreachable!();
    }

    fn distance(&self, source: usize, destination: usize) -> usize {
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0, source));
        while let Some((distance, state)) = q.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);

            if state == destination {
                return distance;
            }

            let cell = &self.cells[state];
            for &neighbor in &cell.neighbors {
                q.push_back((distance + 1, neighbor));
            }
        }
        unreachable!();
    }

    pub fn closest_cell(&self, source: usize, target_kind: CellKind) -> Option<(usize, usize)> {
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();
        q.push_back((0, source));
        while let Some((distance, state)) = q.pop_front() {
            if visited.contains(&state) {
                continue;
            }
            visited.insert(state);

            let cell = &self.cells[state];
            if cell.kind == target_kind && cell.resources != 0 {
                return Some((distance, state));
            }

            for &neighbor in &cell.neighbors {
                q.push_back((distance + 1, neighbor));
            }
        }
        None
    }

    fn beacons_of_line(&self, line: ActionLine) -> Vec<ActionBeacon> {
        let ActionLine {
            source,
            destination,
            strength,
        } = line;
        self.path(source, destination)
            .iter()
            .map(|&location| ActionBeacon { location, strength })
            .collect()
    }

    fn assign_moves(&self, beacons: Vec<ActionBeacon>) -> Vec<MoveAssignment> {
        // sources (current ant positions)
        struct Source {
            location: usize,
            ants: i32,
        }
        let mut sources = Vec::new();
        for (index, cell) in self.cells.iter().enumerate() {
            if cell.allied_ants != 0 {
                continue;
            }
            sources.push(Source {
                location: index,
                ants: cell.allied_ants,
            })
        }

        // sinks (beacons)
        struct Sink {
            location: usize,
            ants: i32,
            wiggle_room: i32,
        }
        let mut sinks = Vec::new();
        let scaling_factor = {
            let total_beacons: i32 = beacons.iter().map(|beacon| beacon.strength).sum();
            // TODO: abstract allied_ants/ennemy_ants
            let total_ants: i32 = self.cells.iter().map(|cell| cell.allied_ants).sum();
            assert!(total_ants != 0);
            f64::from(total_beacons) / f64::from(total_ants)
        };
        for beacon in &beacons {
            let scaled_strength = f64::from(beacon.strength) * scaling_factor;
            let high_beacon_value = scaled_strength.ceil() as i32;
            let low_beacon_value = scaled_strength.floor() as i32;
            let sink = Sink {
                location: beacon.location,
                ants: low_beacon_value.max(1),
                wiggle_room: high_beacon_value - beacon.strength,
            };
            sinks.push(sink);
        }

        // sorted list of source-sink pairs
        let mut pairs = Vec::new();
        for (source_index, source) in sources.iter().enumerate() {
            for (sink_index, sink) in sinks.iter().enumerate() {
                let d = self.distance(source.location, sink.location);
                pairs.push((d, source_index, sink_index));
            }
        }
        pairs.sort();

        let mut assignments = Vec::new();
        let mut stragglers = false;
        while !pairs.is_empty() {
            for &(_, source_index, sink_index) in &pairs {
                let source = &mut sources[source_index];
                let sink = &mut sinks[sink_index];

                let wiggle = if stragglers { sink.wiggle_room } else { 0 };
                let sink_size = sink.ants + wiggle;
                let assignment_size = sink_size.max(source.ants);
                if assignment_size == 0 {
                    continue;
                }
                assignments.push(MoveAssignment {
                    source: source_index,
                    destination: sink_index,
                    amount: assignment_size,
                });
                source.ants -= assignment_size;
                sink.ants -= assignment_size - wiggle;
                sink.wiggle_room -= wiggle;
            }
            pairs = pairs
                .into_iter()
                .filter(|&(_, source_index, _sink_index)| self.cells[source_index].allied_ants > 0)
                .collect();
            stragglers = true;
        }

        assignments
    }

    fn step(
        mut self,
        allied_beacons: Vec<ActionBeacon>,
        _ennemy_beacons: Vec<ActionBeacon>,
    ) -> Self {
        let move_assignments = self.assign_moves(allied_beacons);
        for move_assignment in move_assignments {
            let MoveAssignment {
                source,
                destination,
                amount,
            } = move_assignment;
            let path = self.path(source, destination);
            if path.len() > 1 {
                let source = &mut self.cells[source];
                // TODO: abstract allied_ants/ennemy_ants
                assert!(source.allied_ants >= amount);
                source.allied_ants -= amount;

                let next_step = path[1];
                let next_step = &mut self.cells[next_step];
                // TODO: abstract allied_ants/ennemy_ants
                next_step.allied_ants += amount;
            }
        }
        self
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ActionLine {
    pub source: usize,
    pub destination: usize,
    pub strength: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ActionBeacon {
    pub location: usize,
    pub strength: i32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ActionMessage {
    pub message: String,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Action {
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
