use std::collections::HashMap;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day25.input");

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    East,
    South,
}

use Tile::*;

type Coordinate = (usize, usize);

fn get_input() -> (HashMap<Coordinate, Tile>, Vec<Coordinate>, Vec<Coordinate>) {
    let mut map = HashMap::new();
    let mut east_facing = vec![];
    let mut south_facing = vec![];

    for (y, line) in INPUT.trim().lines().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            match tile {
                '.' => {
                    map.insert((x, y), Empty);
                }
                '>' => {
                    map.insert((x, y), East);
                    east_facing.push((x, y));
                }
                'v' => {
                    map.insert((x, y), South);
                    south_facing.push((x, y));
                }
                _ => unreachable!(),
            }
        }
    }

    (map, east_facing, south_facing)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut map, mut east_facing, mut south_facing) = get_input();
    let mut count = 0;
    let mut moved = true;

    while moved {
        moved = false;
        east_facing.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        south_facing.sort_unstable_by(|(_, a), (_, b)| a.cmp(b));

        let mut next_east = vec![];
        let mut next_south = vec![];

        for &pos in east_facing.iter() {
            let mut next_pos = (pos.0 + 1, pos.1);
            if let Some(tile) = map.get(&next_pos) {
                if *tile == Empty {
                    moved = true;
                    next_east.push(next_pos);
                } else {
                    next_east.push(pos);
                }
            } else {
                next_pos = (0, next_pos.1);
                if let Empty = map.get(&next_pos).unwrap() {
                    moved = true;
                    next_east.push(next_pos);
                } else {
                    next_east.push(pos);
                }
            }
        }

        for e in east_facing {
            map.insert(e, Empty);
        }
        for e in next_east.iter() {
            map.insert(*e, East);
        }

        east_facing = next_east;

        for &south in south_facing.iter() {
            let mut next_pos = (south.0, south.1 + 1);
            if let Some(tile) = map.get(&next_pos) {
                if *tile == Empty {
                    moved = true;
                    next_south.push(next_pos);
                } else {
                    next_south.push(south);
                }
            } else {
                next_pos = (next_pos.0, 0);
                if let Empty = map.get(&next_pos).unwrap() {
                    moved = true;
                    next_south.push(next_pos);
                } else {
                    next_south.push(south);
                }
            }
        }

        for s in south_facing {
            map.insert(s, Empty);
        }
        for s in next_south.iter() {
            map.insert(*s, South);
        }

        south_facing = next_south;

        count += 1;
    }
    println!(
        "At the {}th days o' christmas the cucumber stopped moving!",
        count
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    println!("All star aquired, thanks for playing :)");
    Ok(())
}
