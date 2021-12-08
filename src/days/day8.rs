use std::error::Error;

static INPUT: &str = include_str!("../../data/day8.input");

type Line = (Vec<String>, Vec<String>);

fn get_input() -> Vec<Line> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            let mut li = line
                .trim()
                .split('|')
                .map(|elem| {
                    elem.split_whitespace()
                        .map(String::from)
                        .collect::<Vec<String>>()
                })
                .collect::<Vec<_>>();
            let display = li.pop().unwrap();
            let signals = li.pop().unwrap();
            (signals, display)
        })
        .collect::<Vec<Line>>()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let input = get_input();
    let mut count = 0;

    for (_, display) in input {
        count += display
            .iter()
            .filter(|sig| {
                let len = sig.chars().count();
                len == 2 || len == 3 || len == 4 || len == 7
            })
            .count();
    }

    println!("Recognizable digits: {}", count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
