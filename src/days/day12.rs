use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

const INPUT: &str = include_str!("../../data/day12.input");

fn get_input() -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for line in INPUT.trim().lines() {
        let parts = line.split('-').map(String::from).collect::<Vec<_>>();
        let destination = map.entry(parts[0].clone()).or_insert_with(Vec::new);
        destination.push(parts[1].clone());
        let destination = map.entry(parts[1].clone()).or_insert_with(Vec::new);
        destination.push(parts[0].clone());
    }

    map
}

fn bfs(
    current_node: String,
    map: &HashMap<String, Vec<String>>,
    visited_small: &mut HashSet<String>,
    mut path_count: usize,
) -> usize {
    if &current_node == "end" {
        return path_count + 1;
    }

    let mut was_inserted = false;

    if current_node.chars().next().unwrap().is_ascii_lowercase() {
        if visited_small.get(&current_node).is_some() {
            return path_count;
        } else {
            visited_small.insert(current_node.clone());
            was_inserted = true;
        }
    }

    let next_targets = map.get(&current_node).unwrap();

    for next in next_targets {
        path_count = bfs(next.to_string(), map, visited_small, path_count);
    }

    if was_inserted {
        visited_small.remove(&current_node);
    }

    path_count
}

fn bfs_special(
    current_node: String,
    map: &HashMap<String, Vec<String>>,
    visited_small: &mut HashSet<String>,
    visited_twice: bool,
    mut path_count: usize,
) -> usize {
    if &current_node == "end" {
        return path_count + 1;
    }
    let mut twice = visited_twice;
    let mut was_inserted = false;

    if current_node.chars().next().unwrap().is_ascii_lowercase() {
        if visited_small.get(&current_node).is_some() {
            if twice || &current_node == "start" {
                return path_count;
            }
            twice = true;
        } else {
            visited_small.insert(current_node.clone());
            was_inserted = true;
        }
    }

    let next_targets = map.get(&current_node).unwrap();

    for next in next_targets {
        path_count = bfs_special(next.to_string(), map, visited_small, twice, path_count);
    }

    if was_inserted {
        visited_small.remove(&current_node);
    }

    path_count
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_input();

    let count = bfs(String::from("start"), &map, &mut HashSet::new(), 0);

    println!("Number of path is: {}", count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_input();

    let count = bfs_special(String::from("start"), &map, &mut HashSet::new(), false, 0);

    println!("Number of path is: {}", count);

    Ok(())
}
