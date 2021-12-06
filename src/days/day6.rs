use std::collections::VecDeque;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day6.input");

fn get_school() -> [usize; 9] {
    let mut school = [0; 9];

    let days = INPUT
        .trim()
        .split(',')
        .map(|num| num.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    for day in days {
        school[day] += 1;
    }

    school
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut school: VecDeque<usize> = get_school().into_iter().collect();

    for _ in 0..80 {
        let temp = school.pop_front().unwrap();
        let reset = school.get_mut(6).unwrap();
        *reset += temp;
        school.push_back(temp);
    }

    println!("{}", school.iter().sum::<usize>());

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
