use std::error::Error;

static INPUT: &str = include_str!("../../data/day7.input");

fn get_input() -> Vec<isize> {
    INPUT
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let crabs = get_input();
    let max = crabs.iter().max().unwrap();

    let mut min_fuel: Option<isize> = None;
    for i in 0..=*max {
        let fuel = crabs.iter().fold(0, |acc, x| acc + (x - i).abs());
        min_fuel = if let Some(current) = min_fuel {
            Some(std::cmp::min(current, fuel))
        } else {
            Some(fuel)
        }
    }

    println!("{}", min_fuel.unwrap());

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
