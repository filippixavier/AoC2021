use std::collections::HashMap;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day9.input");

type Map = HashMap<(usize, usize), usize>;

fn get_heightmap() -> Map {
    let mut map = HashMap::new();

    for (line_no, line) in INPUT.trim().lines().enumerate() {
        for (col_no, val) in line.trim().chars().enumerate() {
            map.insert((line_no, col_no), val.to_string().parse().unwrap());
        }
    }

    map
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_heightmap();
    let mut risk_level = 0;
    for (coordinate, value) in map.iter() {
        if coordinate.0 > 0 {
            if let Some(neighbors) = map.get(&(coordinate.0 - 1, coordinate.1)) {
                if neighbors <= value {
                    continue;
                }
            }
        }
        if coordinate.1 > 0 {
            if let Some(neighbors) = map.get(&(coordinate.0, coordinate.1 - 1)) {
                if neighbors <= value {
                    continue;
                }
            }
        }
        if let Some(neighbors) = map.get(&(coordinate.0 + 1, coordinate.1)) {
            if neighbors <= value {
                continue;
            }
        }

        if let Some(neighbors) = map.get(&(coordinate.0, coordinate.1 + 1)) {
            if neighbors <= value {
                continue;
            }
        }

        risk_level += value + 1;
    }

    println!("Total risk value is: {}", risk_level);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
