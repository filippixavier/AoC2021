use std::error::Error;
use std::fs;
use std::path::Path;

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let depths: Vec<usize> = fs::read_to_string(Path::new("./data/day1.input"))?
        .trim()
        .split('\n')
        .map(|depth| (depth.trim().parse::<usize>().unwrap_or(0)))
        .collect();

    let mut count = 0;

    for i in 1..depths.len() {
        if depths[i] > depths[i - 1] {
            count += 1;
        }
    }

    println!("{}", count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
