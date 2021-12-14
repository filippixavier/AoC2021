use std::collections::HashMap;
use std::error::Error;

const INPUT: &str = include_str!("../../data/day14.input");

type PairInsertion = HashMap<String, String>;

fn get_input() -> (String, PairInsertion) {
    let mut lines = INPUT.trim().lines();
    let mut pair_insertion = PairInsertion::new();

    let template = lines.next().unwrap().to_string();

    for line in lines.skip(1) {
        let parts = line.split(" -> ").collect::<Vec<_>>();
        pair_insertion.insert(parts[0].to_string(), parts[1].to_string());
    }

    (template, pair_insertion)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut template, pair_insertion) = get_input();
    let mut counters: HashMap<char, usize> = HashMap::new();

    for _ in 0..10 {
        let mut next_phase = String::new();
        let chars = template.chars().collect::<Vec<_>>();

        for i in 0..chars.len() - 1 {
            let current = chars[i].to_string() + &chars[i + 1].to_string();
            next_phase += &chars[i].to_string();
            if let Some(insert) = pair_insertion.get(&current) {
                next_phase += &insert.to_string();
            }
        }

        next_phase += &chars.last().unwrap().to_string();

        template = next_phase;
    }

    for i in template.chars() {
        let count = counters.entry(i).or_insert(0);
        *count += 1;
    }

    let min = counters.values().min().unwrap();
    let max = counters.values().max().unwrap();

    println!("Element score is: {}", max - min);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
