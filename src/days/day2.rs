use std::error::Error;
use std::fs;
use std::path::Path;

enum Direction {
    Up,
    Down,
    Forward,
}

fn get_input() -> Vec<(i32, Direction)> {
    fs::read_to_string(Path::new("./data/day2.input"))
        .expect("Something wrong with input")
        .trim()
        .lines()
        .map(|instruction| {
            let mut splitted = instruction.trim().split(' ');
            let direction = splitted.next().unwrap();
            let amount = splitted.next().unwrap().parse::<i32>().unwrap_or(0);
            match direction {
                "forward" => (amount, Direction::Forward),
                "up" => (amount, Direction::Up),
                "down" => (amount, Direction::Down),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut forward, mut depth) = (0, 0);
    let instructions = get_input();

    for (amount, direction) in instructions {
        match direction {
            Direction::Up => {
                depth -= amount;
            }
            Direction::Down => {
                depth += amount;
            }
            Direction::Forward => {
                forward += amount;
            }
        };
    }

    println!("Final position code is: {}", forward * depth);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut forward, mut depth, mut aim) = (0, 0, 0);
    let instructions = get_input();

    for (amount, direction) in instructions {
        match direction {
            Direction::Up => {
                aim -= amount;
            }
            Direction::Down => {
                aim += amount;
            }
            Direction::Forward => {
                forward += amount;
                depth += amount * aim;
            }
        };
    }

    println!("Final position code is: {}", forward * depth);

    Ok(())
}
