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
                let len = sig.len();
                len == 2 || len == 3 || len == 4 || len == 7
            })
            .count();
    }

    println!("Recognizable digits: {}", count);

    Ok(())
}

fn reorder(input: &[String]) -> Vec<String> {
    input
        .iter()
        .map(|l| {
            let mut x = l.chars().collect::<Vec<_>>();
            x.sort_unstable();
            x.into_iter().collect::<String>()
        })
        .collect::<Vec<_>>()
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    use std::collections::HashMap;

    let input = get_input();
    let mut count = 0;

    for (signals, display) in input {
        let mut coupling: HashMap<usize, String> = HashMap::new();
        let mut sig = reorder(&signals);
        let dis = reorder(&display);

        // Find 1, 4, 7, 8
        sig = sig
            .into_iter()
            .filter(|x| match x.len() {
                2 => {
                    coupling.insert(1, x.clone());
                    false
                }
                3 => {
                    coupling.insert(7, x.clone());
                    false
                }
                4 => {
                    coupling.insert(4, x.clone());
                    false
                }
                7 => {
                    coupling.insert(8, x.clone());
                    false
                }
                _ => true,
            })
            .collect();
        // Find 2, 3, 5
        sig = sig
            .into_iter()
            .filter(|x| match x.len() {
                5 => {
                    let one = coupling.get(&1).unwrap().chars();
                    let four = coupling.get(&4).unwrap().chars();
                    let count = one.filter(|y| !x.contains(*y)).count();
                    if count == 0 {
                        coupling.insert(3, x.clone());
                    } else if four.filter(|y| !x.contains(*y)).count() == 2 {
                        coupling.insert(2, x.clone());
                    } else {
                        coupling.insert(5, x.clone());
                    }
                    false
                }
                _ => true,
            })
            .collect();
        // find 0, 6, 9
        sig.into_iter().for_each(|x| {
            if x.len() == 6 {
                let one = coupling.get(&1).unwrap().chars();
                let three = coupling.get(&3).unwrap().chars();
                let count = three.filter(|y| !x.contains(*y)).count();
                if count == 0 {
                    coupling.insert(9, x);
                } else if one.filter(|y| !x.contains(*y)).count() == 0 {
                    coupling.insert(0, x);
                } else {
                    coupling.insert(6, x);
                }
            }
        });

        let coupling: HashMap<String, usize> = coupling.into_iter().map(|(k, v)| (v, k)).collect();

        count += dis.iter().fold(0, |acc, value| {
            let temp = coupling.get(value).unwrap_or(&0);
            acc * 10 + temp
        });
    }

    println!("Total output: {}", count);

    Ok(())
}
