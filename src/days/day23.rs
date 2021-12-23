use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day23.input");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tiles {
    Void,
    Empty,
    Forbidden,
    Room,
    Wall,
    Amphi(char),
}

use Tiles::*;

type Coordinates = (usize, usize);

type Map = HashMap<Coordinates, Tiles>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Amphipod {
    sub_type: char,
    coordinates: Coordinates,
    targets: [Coordinates; 2],
    tile_type: Tiles,
    energy: usize,
}

static AMBER: Amphipod = Amphipod {
    sub_type: 'A',
    coordinates: (0, 0),
    targets: [(3, 2), (3, 3)],
    tile_type: Room,
    energy: 1,
};

static BRONZE: Amphipod = Amphipod {
    sub_type: 'B',
    coordinates: (0, 0),
    targets: [(5, 2), (5, 3)],
    tile_type: Room,
    energy: 10,
};

static COPPER: Amphipod = Amphipod {
    sub_type: 'C',
    coordinates: (0, 0),
    targets: [(7, 2), (7, 3)],
    tile_type: Room,
    energy: 100,
};

static DESERT: Amphipod = Amphipod {
    sub_type: 'D',
    coordinates: (0, 0),
    targets: [(9, 2), (9, 3)],
    tile_type: Room,
    energy: 1000,
};

impl Amphipod {
    fn new(sub_type: char, coordinates: Coordinates) -> Self {
        let mut amphi = match sub_type {
            'A' => AMBER.clone(),
            'B' => BRONZE.clone(),
            'C' => COPPER.clone(),
            'D' => DESERT.clone(),
            _ => unreachable!(),
        };
        amphi.coordinates = coordinates;
        amphi
    }

    fn get_move(&self, map: &Map) -> bool {
        let mut need = true;
        if self.targets[1] == self.coordinates {
            need = false;
        }
        if self.targets[0] == self.coordinates {
            if let Amphi(x) = map.get(&self.targets[1]).unwrap() {
                if *x == self.sub_type {
                    need = false;
                }
            }
        }
        need
    }

    fn try_move_out(&self, map: &Map) -> Option<Vec<(Coordinates, usize)>> {
        let mut visited: HashSet<Coordinates> = vec![self.coordinates].into_iter().collect();
        let neighbors = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut current_coordinates = vec![(self.coordinates, 0)];
        let mut candidates = vec![];
        while !current_coordinates.is_empty() {
            let (coordinates, steps) = current_coordinates.pop().unwrap();
            for neighbor in neighbors.iter() {
                let next_steps = steps + 1;
                let next_coordinates = (
                    (coordinates.0 as isize + neighbor.0) as usize,
                    (coordinates.1 as isize + neighbor.1) as usize,
                );
                if visited.contains(&next_coordinates) {
                    continue;
                }
                visited.insert(next_coordinates);
                let energy = next_steps * self.energy;
                match map.get(&next_coordinates).unwrap() {
                    Empty => {
                        candidates.push((next_coordinates, energy));
                        current_coordinates.push((next_coordinates, next_steps));
                    }
                    Forbidden => {
                        current_coordinates.push((next_coordinates, next_steps));
                    }
                    Room => {
                        if self.targets[1] == next_coordinates {
                            return Some(vec![(next_coordinates, energy)]);
                        } else if self.targets[0] == next_coordinates {
                            let next_tile = map.get(&self.targets[1]).unwrap();
                            if let Amphi(x) = next_tile {
                                if x == &self.sub_type {
                                    return Some(vec![(next_coordinates, energy)]);
                                }
                            } else {
                                current_coordinates.push((next_coordinates, next_steps));
                            }
                        } else {
                            current_coordinates.push((next_coordinates, next_steps));
                        }
                    }
                    _ => {}
                }
            }
        }
        if candidates.is_empty() {
            None
        } else {
            Some(candidates)
        }
    }

    fn try_move_in(&self, map: &Map) -> Option<Vec<(Coordinates, usize)>> {
        if let Amphi(x) = map.get(&self.targets[1]).unwrap() {
            if x != &self.sub_type {
                return None;
            }
        }
        let neighbors = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        let mut visited: HashSet<Coordinates> = HashSet::new();
        let mut current_coordinates = vec![(self.coordinates, 0)];
        let mut targets_available = vec![];
        while !current_coordinates.is_empty() {
            let (coordinates, steps) = current_coordinates.pop().unwrap();
            if self.targets.contains(&coordinates) {
                targets_available.push(steps);
            }
            for neighbor in neighbors.iter() {
                let next_steps = steps + 1;
                let next_coordinates = (
                    (coordinates.0 as isize + neighbor.0) as usize,
                    (coordinates.1 as isize + neighbor.1) as usize,
                );
                if visited.contains(&next_coordinates) {
                    continue;
                }
                visited.insert(next_coordinates);
                match map.get(&next_coordinates).unwrap() {
                    Empty | Forbidden | Room => {
                        current_coordinates.push((next_coordinates, next_steps));
                    }
                    _ => {}
                }
            }
        }

        if targets_available.len() == 2 {
            let energy = targets_available[1] * self.energy;
            Some(vec![(self.targets[1], energy)])
        } else if targets_available.len() == 1 {
            let energy = targets_available[0] * self.energy;
            if let Amphi(x) = map.get(&self.targets[1]).unwrap() {
                if x == &self.sub_type {
                    Some(vec![(self.targets[0], energy)])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn move_amphi(&self, map: &Map) -> Option<Vec<(Coordinates, usize)>> {
        if self.tile_type == Room {
            self.try_move_out(map)
        } else {
            self.try_move_in(map)
        }
    }
}

fn get_input() -> (Map, Vec<Amphipod>) {
    let mut amphis = vec![];
    let mut map: Map = Map::new();
    for (y, lines) in INPUT.trim().lines().enumerate() {
        for (x, tile_char) in lines.chars().enumerate() {
            let tile = match tile_char {
                '#' => Wall,
                '.' => Empty,
                'A' | 'B' | 'C' | 'D' => {
                    amphis.push(Amphipod::new(tile_char, (x, y)));
                    Amphi(tile_char)
                }
                _ => Void,
            };
            map.insert((x, y), tile);
        }
    }
    map.insert((3, 1), Forbidden);
    map.insert((5, 1), Forbidden);
    map.insert((7, 1), Forbidden);
    map.insert((9, 1), Forbidden);

    (map, amphis)
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    energy_spent: usize,
    current_amphi: Vec<Amphipod>,
    current_map: Map,
    moves: usize,
    move_list: Vec<(Coordinates, Coordinates)>,
    energy_list: Vec<usize>,
    signature: String,
}

impl State {
    fn new(
        energy_spent: usize,
        current_amphi: Vec<Amphipod>,
        current_map: Map,
        moves: usize,
    ) -> Self {
        let signature = current_amphi.iter().fold(String::new(), |acc, amphi| {
            format!("{}{}:{:?}", acc, amphi.sub_type, amphi.coordinates)
        });
        State {
            energy_spent,
            current_amphi: current_amphi
                .into_iter()
                .filter(|x| x.get_move(&current_map))
                .collect(),
            current_map,
            moves,
            signature,
            energy_list: vec![],
            move_list: vec![],
        }
    }
}

use std::cmp::Ordering;

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.energy_spent == other.energy_spent {
            other.moves.partial_cmp(&self.moves)
        } else {
            other.energy_spent.partial_cmp(&self.energy_spent)
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.energy_spent == other.energy_spent {
            other.moves.cmp(&self.moves)
        } else {
            other.energy_spent.cmp(&self.energy_spent)
        }
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    use std::collections::BinaryHeap;
    let (map, amphipod) = get_input();
    let mut available_states: BinaryHeap<State> = BinaryHeap::new();
    let mut covered_steps: HashMap<String, usize> = HashMap::new();
    let mut total_energy = usize::MAX;

    available_states.push(State::new(0, amphipod, map, 0));

    while !available_states.is_empty() {
        let state = available_states.pop().unwrap();
        if state.current_amphi.is_empty() {
            total_energy = total_energy.min(state.energy_spent);
            break;
        }

        for (i, unplaced_amphi) in state.current_amphi.iter().enumerate() {
            if let Some(candidates_posi) = unplaced_amphi.move_amphi(&state.current_map) {
                for (next_coordinates, energy_spent) in candidates_posi {
                    let mut amphi = Amphipod::new(unplaced_amphi.sub_type, next_coordinates);
                    amphi.tile_type = *state.current_map.get(&amphi.coordinates).unwrap();
                    let mut next_amphis = state.current_amphi.clone();
                    let mut next_map = state.current_map.clone();
                    next_map.insert(unplaced_amphi.coordinates, unplaced_amphi.tile_type);
                    next_map.insert(amphi.coordinates, Amphi(amphi.sub_type));
                    next_amphis[i] = amphi;
                    let mut next_state = State::new(
                        state.energy_spent + energy_spent,
                        next_amphis,
                        next_map,
                        state.moves + 1,
                    );
                    next_state.energy_list = state.energy_list.clone();
                    next_state.move_list = state.move_list.clone();
                    next_state.energy_list.push(energy_spent);
                    next_state
                        .move_list
                        .push((unplaced_amphi.coordinates, next_coordinates));
                    if let Some(energy) = covered_steps.get(&next_state.signature) {
                        if *energy < next_state.energy_spent {
                            continue;
                        }
                    }
                    covered_steps.insert(next_state.signature.clone(), next_state.energy_spent);
                    available_states.push(next_state);
                }
            }
        }
    }
    println!("Minimum energy used: {}", total_energy);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
