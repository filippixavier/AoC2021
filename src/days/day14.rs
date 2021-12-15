use std::collections::HashMap;
use std::error::Error;

const INPUT: &str = include_str!("../../data/day14.input");

type Pair = (char, char);
type PairInsertion = HashMap<Pair, char>;

fn get_input() -> (Vec<char>, PairInsertion) {
    let mut lines = INPUT.trim().lines();
    let mut pair_insertion = PairInsertion::new();

    let template = lines.next().unwrap().chars().collect();

    for line in lines.skip(1) {
        let parts: Vec<_> = line.split(" -> ").collect();
        let input: Vec<_> = parts[0].chars().collect();
        pair_insertion.insert((input[0], input[1]), parts[1].chars().next().unwrap());
    }

    (template, pair_insertion)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut template, pair_insertion) = get_input();
    let mut counters: HashMap<char, usize> = HashMap::new();

    for _ in 0..10 {
        let mut next_phase = vec![];

        for i in 0..template.len() - 1 {
            let current = (template[i], template[i + 1]);
            next_phase.push(template[i]);
            if let Some(insert) = pair_insertion.get(&current) {
                next_phase.push(*insert);
            }
        }

        next_phase.push(*template.last().unwrap());

        template = next_phase;
    }

    for i in template {
        let count = counters.entry(i).or_insert(0);
        *count += 1;
    }

    let min = counters.values().min().unwrap();
    let max = counters.values().max().unwrap();

    println!("Element score is: {}", max - min);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (template, pair_insertion) = get_input();
    let mut counters: HashMap<char, u64> = HashMap::new();

    let mut pair_count: HashMap<Pair, u64> = pair_insertion.keys().map(|&key| (key, 0)).collect();

    for i in 0..template.len() {
        let elem = template[i];
        let count = counters.entry(elem).or_insert(0);
        *count += 1;

        if i < template.len() - 1 {
            let pair = (elem, template[i + 1]);
            if pair_insertion.contains_key(&pair) {
                let count = pair_count.entry(pair).or_insert(0);
                *count += 1;
            }
        }
    }

    for _ in 0..40 {
        let mut next_pair = HashMap::new();

        for (pair, amount) in pair_count {
            let insert = *pair_insertion.get(&pair).unwrap();
            let count = counters.entry(insert).or_insert(0);
            *count += amount;

            let byproducts = [(pair.0, insert), (insert, pair.1)];

            for byproduct in byproducts {
                if pair_insertion.contains_key(&byproduct) {
                    let count = next_pair.entry(byproduct).or_insert(0);
                    *count += amount;
                }
            }
        }

        pair_count = next_pair;
    }

    let min = counters.values().min().unwrap();
    let max = counters.values().max().unwrap();

    println!("Big Element score is: {}", max - min);

    Ok(())
}
