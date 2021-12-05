use std::error::Error;
use std::fs;
use std::path::Path;

use std::collections::HashMap;

type Point = (u32, u32);

fn get_input2(with_diag: bool) -> HashMap<Point, u8> {
    let mut map = HashMap::new();

    let input = fs::read_to_string(Path::new("./data/day5.input"))
        .expect("Something went wrong when reading input");

    for line in input.trim().lines() {
        let coordinates: Vec<&str> = line.trim().split_ascii_whitespace().collect();

        let inter1: Vec<u32> = coordinates[0]
            .split(',')
            .map(|x| x.parse::<u32>())
            .collect::<Result<_, _>>()
            .unwrap();
        let inter2: Vec<u32> = coordinates[2]
            .split(',')
            .map(|x| x.parse::<u32>())
            .collect::<Result<_, _>>()
            .unwrap();

        let p1 = (inter1[0], inter1[1]);
        let p2 = (inter2[0], inter2[1]);

        let start;
        let end;

        if p1.0 == p2.0 {
            if p1.1 < p2.1 {
                start = p1;
                end = p2;
            } else {
                start = p2;
                end = p1;
            }
            for i in start.1..=end.1 {
                let position = (start.0, i);
                let count = map.entry(position).or_insert(0);
                *count += 1;
            }
        } else if p1.1 == p2.1 {
            if p1.0 < p2.0 {
                start = p1;
                end = p2;
            } else {
                start = p2;
                end = p1;
            }

            for i in start.0..=end.0 {
                let position = (i, start.1);
                let count = map.entry(position).or_insert(0);
                *count += 1;
            }
        } else if with_diag {
            if p1.0 < p2.0 {
                start = p1;
                end = p2;
            } else {
                start = p2;
                end = p1;
            }

            for (step, i) in (start.0..=end.0).into_iter().enumerate() {
                let position = if start.1 < end.1 {
                    (i, start.1 + step as u32)
                } else {
                    (i, start.1 - step as u32)
                };
                let count = map.entry(position).or_insert(0);
                *count += 1;
            }
        }
    }

    map
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_input2(false);

    let count = map.values().filter(|i| **i > 1).count();

    println!("Number of overlapping points: {}", count);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let map = get_input2(true);

    let count = map.values().filter(|i| **i > 1).count();

    println!("Number of overlapping points (with diag): {}", count);
    Ok(())
}
