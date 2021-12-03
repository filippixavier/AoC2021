use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> (Vec<String>, usize) {
    let mut count = 0;
    let reading = fs::read_to_string(Path::new("./data/day3.input"))
        .expect("Something wrong with the input")
        .trim()
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            count = trimmed.chars().count();
            String::from(trimmed)
        })
        .collect();
    (reading, count)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (reading, line_len) = get_input();
    let count = reading.len();
    let mut one_count: Vec<usize> = vec![0; line_len];
    let (mut epsilon, mut gamma) = (0, 0);

    for line in reading {
        for (i, val) in line.chars().enumerate() {
            if val == '1' {
                one_count[i] += 1;
            }
        }
    }
    for (i, val) in one_count.into_iter().rev().enumerate() {
        if val >= count / 2 {
            gamma += usize::pow(2, i as u32);
        } else {
            epsilon += usize::pow(2, i as u32);
        }
    }

    println!(
        "Gamma rate: {}, Epsilon rate: {}, Power consumption: {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
