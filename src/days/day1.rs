use std::error::Error;
use std::fs;
use std::path::Path;

fn get_input() -> Vec<usize> {
    fs::read_to_string(Path::new("./data/day1.input"))
        .unwrap_or(String::from(""))
        .trim()
        .split('\n')
        .map(|depth| (depth.trim().parse::<usize>().unwrap_or(0)))
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let depths = get_input();

    let mut count = 0;

    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count += 1;
        }
    }

    println!("Number of depth increase: {}", count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let depths = get_input();
    let mut count = 0;
    let mut previous = 0;

    for i in 0..depths.len() - 2 {
        let sum = depths[i] + depths[i + 1] + depths[i + 2];
        if previous != 0 {
            if previous < sum {
                count += 1;
            }
        }
        previous = sum;
    }

    println!("Number of depth increase by group of three is: {}", count);

    Ok(())
}
