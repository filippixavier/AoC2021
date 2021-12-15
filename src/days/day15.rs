use std::collections::HashMap;
use std::error::Error;

const INPUT: &str = include_str!("../../data/day15.input");

fn get_input() -> Vec<Vec<u8>> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|risk| risk.to_string().parse())
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect()
}

fn a_star(risk_level: &[Vec<u8>]) -> usize {
    let mut open_set = vec![(0, 0)];

    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    g_score.insert((0, 0), 0);
    // let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();

    while !open_set.is_empty() {
        open_set.sort_unstable_by(|a, b| g_score.get(b).unwrap().cmp(g_score.get(a).unwrap()));

        let current = open_set.pop().unwrap();
        let current_score = *g_score.get(&current).unwrap();
        if current == (risk_level.len() - 1, risk_level[0].len() - 1) {
            return current_score;
        }

        let neighbors: Vec<_> = vec![
            (current.0.saturating_sub(1), current.1),
            (current.0 + 1, current.1),
            (current.0, current.1.saturating_sub(1)),
            (current.0, current.1 + 1),
        ]
        .into_iter()
        .filter(|&x| x != current)
        .collect();

        for neighbor in neighbors {
            let d = if let Some(line) = risk_level.get(neighbor.0) {
                if let Some(value) = line.get(neighbor.1) {
                    *value
                } else {
                    continue;
                }
            } else {
                continue;
            };

            let tentative_score = current_score + d as usize;
            let mut is_smaller = false;
            if let Some(score) = g_score.get(&neighbor) {
                if tentative_score < *score {
                    is_smaller = true;
                }
            } else {
                is_smaller = true;
            }

            if is_smaller {
                g_score.insert(neighbor, tentative_score);

                if !open_set.contains(&neighbor) {
                    open_set.push(neighbor);
                }
            }
        }
    }

    0
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let risk_level = get_input();
    println!("Lowest total risk: {}", a_star(&risk_level));
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
