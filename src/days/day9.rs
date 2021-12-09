use std::collections::HashMap;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day9.input");

type Map = HashMap<(usize, usize), Option<usize>>;

fn get_heightmap() -> Map {
    let mut map = HashMap::new();

    for (line_no, line) in INPUT.trim().lines().enumerate() {
        for (col_no, val) in line.trim().chars().enumerate() {
            map.insert((line_no, col_no), Some(val.to_string().parse().unwrap()));
        }
    }

    map
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_heightmap();
    let mut risk_level = 0;
    for (coordinate, value) in map.iter() {
        let val = value.unwrap();
        if coordinate.0 > 0 {
            if let Some(neighbors) = map.get(&(coordinate.0 - 1, coordinate.1)) {
                let x = neighbors.unwrap();
                if x <= val {
                    continue;
                }
            }
        }
        if coordinate.1 > 0 {
            if let Some(neighbors) = map.get(&(coordinate.0, coordinate.1 - 1)) {
                let x = neighbors.unwrap();
                if x <= val {
                    continue;
                }
            }
        }
        if let Some(neighbors) = map.get(&(coordinate.0 + 1, coordinate.1)) {
            let x = neighbors.unwrap();
            if x <= val {
                continue;
            }
        }

        if let Some(neighbors) = map.get(&(coordinate.0, coordinate.1 + 1)) {
            let x = neighbors.unwrap();
            if x <= val {
                continue;
            }
        }

        risk_level += val + 1;
    }

    println!("Total risk value is: {}", risk_level);
    Ok(())
}

fn bfs(map: &mut Map, coordinate: (usize, usize)) -> usize {
    let mut size = 1;
    map.insert(coordinate, None);
    if coordinate.0 > 0 {
        if let Some(neighbors) = map.get(&(coordinate.0 - 1, coordinate.1)) {
            if neighbors.unwrap_or(9) != 9 {
                size += bfs(map, (coordinate.0 - 1, coordinate.1));
            }
        }
    }
    if coordinate.1 > 0 {
        if let Some(neighbors) = map.get(&(coordinate.0, coordinate.1 - 1)) {
            if neighbors.unwrap_or(9) != 9 {
                size += bfs(map, (coordinate.0, coordinate.1 - 1));
            }
        }
    }
    if let Some(neighbors) = map.get(&(coordinate.0 + 1, coordinate.1)) {
        if neighbors.unwrap_or(9) != 9 {
            size += bfs(map, (coordinate.0 + 1, coordinate.1));
        }
    }

    if let Some(neighbors) = map.get(&(coordinate.0, coordinate.1 + 1)) {
        if neighbors.unwrap_or(9) != 9 {
            size += bfs(map, (coordinate.0, coordinate.1 + 1));
        }
    }
    size
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut map = get_heightmap();
    let mut basin_sizes = vec![];

    let keys = map.keys().cloned().collect::<Vec<_>>();

    for coordinate in keys {
        if let Some(value) = map.get(&coordinate) {
            if value.unwrap_or(9) == 9 {
                continue;
            }
        }
        let size = bfs(&mut map, coordinate);
        basin_sizes.push(size);
    }

    basin_sizes.sort_unstable();
    println!(
        "Largest basin product is: {}",
        basin_sizes.into_iter().rev().take(3).product::<usize>()
    );
    Ok(())
}
