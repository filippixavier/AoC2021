use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day23.input");

type Coordinates = (usize, usize);
type Map = HashMap<Coordinates, Tile>;

enum Tile {
    Void,
    Wall,
    Empty,
    Forbidden,
    Room(char),
}

use Tile::*;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Amphipod {
    position: Coordinates,
    energy: usize,
    species: char,
    id: usize,
}

impl Amphipod {
    fn new(position: Coordinates, species: char, id: usize) -> Self {
        let energy = match species {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        };
        Amphipod {
            position,
            energy,
            species,
            id,
        }
    }

    fn need_move(&self, map: &Map, amphibs: &[Amphipod]) -> bool {
        if let Room(t) = map.get(&self.position).unwrap() {
            if *t == self.species {
                Amphipod::have_misplaced(self.position, self.species, map, amphibs)
            } else {
                true
            }
        } else {
            true
        }
    }

    fn have_misplaced(
        coordinate: Coordinates,
        species: char,
        map: &Map,
        amphibs: &[Amphipod],
    ) -> bool {
        let mut y = coordinate.1 + 1;
        let mut misplaced = false;
        'outer: loop {
            if let Wall = map.get(&(coordinate.0, y)).unwrap() {
                break;
            }
            for amphi in amphibs {
                if amphi.position == (coordinate.0, y) && amphi.species != species {
                    misplaced = true;
                    break 'outer;
                }
            }
            y += 1;
        }
        misplaced
    }

    fn new_coordinates(
        &self,
        map: &Map,
        amphibs: &[Amphipod],
        placed: &[Amphipod],
    ) -> Vec<(Coordinates, usize)> {
        let mut other_positions: HashSet<Coordinates> =
            amphibs.iter().map(|x| x.position).collect();
        for place in placed {
            other_positions.insert(place.position);
        }
        let start_from_room = matches!(map.get(&self.position).unwrap(), Room(_));
        let mut visited: HashSet<Coordinates> = HashSet::new();
        let mut positions: Vec<(Coordinates, usize)> = vec![(self.position, 0)];
        let neighbors = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut candidates: Vec<(Coordinates, usize)> = vec![];

        let mut found_room = false;

        while !positions.is_empty() {
            let (current_pos, steps) = positions.pop().unwrap();
            let next_step = steps + 1;

            if !visited.insert(current_pos) {
                continue;
            }

            for neighbor in neighbors.iter() {
                let next_position = (
                    (current_pos.0 as isize + neighbor.0) as usize,
                    (current_pos.1 as isize + neighbor.1) as usize,
                );
                if other_positions.contains(&next_position) || visited.contains(&next_position) {
                    continue;
                }
                let tile = map.get(&next_position).unwrap();
                match tile {
                    Empty => {
                        if !found_room && start_from_room {
                            candidates.push((next_position, next_step * self.energy));
                        }
                        positions.push((next_position, next_step));
                    }
                    Forbidden => {
                        positions.push((next_position, next_step));
                    }
                    Room(x) => {
                        positions.push((next_position, next_step));
                        if *x == self.species
                            && !Amphipod::have_misplaced(next_position, self.species, map, amphibs)
                        {
                            found_room = true;
                            candidates = vec![(next_position, next_step * self.energy)];
                        }
                    }
                    _ => {}
                }
            }
        }
        candidates
    }
}

fn get_input(insert: Option<&str>) -> (Vec<Amphipod>, Map) {
    let mut amphibs: Vec<Amphipod> = vec![];
    let input = if let Some(additional_part) = insert {
        let mut lines = INPUT.trim().lines();
        let first_part: String = lines.by_ref().take(3).collect::<Vec<_>>().join("\n");
        let second_part: String = lines.collect::<Vec<_>>().join("\n");
        format!("{}\n{}\n{}", first_part, additional_part, second_part)
    } else {
        String::from(INPUT)
    };

    let mut map: Map = HashMap::new();
    let mut id = 0;
    for (y, line) in input.trim().lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            let tile = match tile {
                '#' => Wall,
                ' ' => Void,
                '.' => Empty,
                _ => {
                    amphibs.push(Amphipod::new((x, y), tile, id));
                    id += 1;
                    Room(match x {
                        3 => 'A',
                        5 => 'B',
                        7 => 'C',
                        9 => 'D',
                        _ => unreachable!(),
                    })
                }
            };
            map.insert((x, y), tile);
        }
    }
    map.insert((3, 1), Forbidden);
    map.insert((5, 1), Forbidden);
    map.insert((7, 1), Forbidden);
    map.insert((9, 1), Forbidden);
    (amphibs, map)
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    energy_spent: usize,
    unplaced: Vec<Amphipod>,
    placed: Vec<Amphipod>,
    signature: String,
}

impl State {
    fn new(
        ampibs: Vec<Amphipod>,
        mut already_placed: Vec<Amphipod>,
        energy_spent: usize,
        map: &Map,
    ) -> State {
        let (mut unplaced, mut placed): (Vec<Amphipod>, Vec<Amphipod>) = ampibs
            .iter()
            .cloned()
            .partition(|x| x.need_move(map, &ampibs));
        placed.append(&mut already_placed);

        unplaced.sort_unstable_by(|x, y| {
            if x.position.0 == y.position.0 {
                x.position.1.cmp(&y.position.1)
            } else {
                x.position.0.cmp(&y.position.0)
            }
        });

        let signature = unplaced
            .iter()
            .map(|x| format!("{}:{:?}", x.species, x.position))
            .collect();

        State {
            unplaced,
            placed,
            energy_spent,
            signature,
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.energy_spent.partial_cmp(&self.energy_spent)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.energy_spent.cmp(&self.energy_spent)
    }
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (amphibs, map) = get_input(None);
    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    let mut min_erergy = 0;

    let mut visited_states: HashSet<String> = HashSet::new();

    heap.push(State::new(amphibs, vec![], 0, &map));

    while !heap.is_empty() {
        let state = heap.pop().unwrap();
        if state.unplaced.is_empty() {
            min_erergy = state.energy_spent;
            break;
        }

        if !visited_states.insert(state.signature) {
            continue;
        }

        for (i, amphib) in state.unplaced.iter().enumerate() {
            let movements = amphib.new_coordinates(&map, &state.unplaced, &state.placed);
            for (coordinates, energy_spent) in movements {
                let mut next_ampbibs = state.unplaced.clone();
                next_ampbibs[i].position = coordinates;
                let next_state = State::new(
                    next_ampbibs,
                    state.placed.clone(),
                    state.energy_spent + energy_spent,
                    &map,
                );
                heap.push(next_state);
            }
        }
    }

    println!("{}", min_erergy);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let add_on = r"  #D#C#B#A#
  #D#B#A#C#";
    get_input(Some(add_on));
    Ok(())
}
