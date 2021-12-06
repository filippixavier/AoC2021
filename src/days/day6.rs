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

fn get_total_after_days(mut school: VecDeque<usize>, days: usize) -> usize {
    for _ in 0..days {
        let temp = school.pop_front().unwrap();
        let reset = school.get_mut(6).unwrap();
        *reset += temp;
        school.push_back(temp);
    }

    school.iter().sum::<usize>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let school: VecDeque<usize> = get_school().into_iter().collect();

    println!("{}", get_total_after_days(school, 80));

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let school: VecDeque<usize> = get_school().into_iter().collect();

    println!("{}", get_total_after_days(school, 256));

    Ok(())
}
