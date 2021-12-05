use std::error::Error;
use std::fs;
use std::path::Path;

type Point = (u32, u32);
type Segment = Vec<Point>;

fn get_input() -> Vec<Segment> {
    let input = fs::read_to_string(Path::new("./data/day5.input"))
        .expect("Something went wrong when reading input");
    let mut vents = vec![];
    for line in input.trim().lines() {
        let mut segment = vec![];

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
                segment.push((start.0, i));
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
                segment.push((i, start.1));
            }
        }

        if !segment.is_empty() {
            vents.push(segment);
        }
    }
    vents
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut segments = get_input();
    let mut intersections: Vec<Point> = vec![];

    while segments.len() > 1 {
        let current = segments.pop().unwrap();

        for segment in segments.iter() {
            let intersection = segment.iter().filter(|p| current.contains(p));
            for inter in intersection {
                if !intersections.contains(inter) {
                    intersections.push(*inter);
                }
            }
        }
    }

    println!("Number of overlapping points: {}", intersections.len());
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
