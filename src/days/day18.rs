use std::error::Error;
use std::fmt;

const INPUT: &str = include_str!("../../data/day18.input");

impl fmt::Display for SnailPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_string())
    }
}

#[derive(Clone)]
enum SnailPair {
    Solo(usize),
    Pair(Box<SnailPair>, Box<SnailPair>),
}

use SnailPair::*;
impl SnailPair {
    fn add(&self, other: &Self) -> Self {
        Pair(Box::new(self.clone()), Box::new(other.clone()))
    }

    fn get_string(&self) -> String {
        match self {
            Solo(x) => x.to_string(),
            Pair(x, y) => {
                format!("[{}, {}]", x.get_string(), y.get_string())
            }
        }
    }

    fn check_explosion(self, deep: usize) -> (Self, Option<(usize, usize)>) {
        let next;
        match self {
            Solo(x) => {
                return (Solo(x), None);
            }
            Pair(x, y) => {
                let mut sub_l = *x;
                let mut sub_r = *y;

                if let Solo(left) = sub_l {
                    if let Solo(right) = sub_r {
                        if deep == 4 {
                            return (Solo(0), Some((left, right)));
                        }
                    }
                }
                let (pair_l, rest_l) = sub_l.check_explosion(deep + 1);
                sub_l = pair_l;
                if let Some((x, y)) = rest_l {
                    if y != 0 {
                        sub_r = sub_r.apply_overflow(y, true);
                    }
                    return (Pair(Box::new(sub_l), Box::new(sub_r)), Some((x, 0)));
                } else {
                    let (pair_r, rest_r) = sub_r.check_explosion(deep + 1);
                    sub_r = pair_r;
                    if let Some((x, y)) = rest_r {
                        if x != 0 {
                            sub_l = sub_l.apply_overflow(x, false);
                        }
                        return (Pair(Box::new(sub_l.clone()), Box::new(sub_r)), Some((0, y)));
                    }
                }

                next = Pair(Box::new(sub_l), Box::new(sub_r));
            }
        }

        (next, None)
    }

    fn check_split(self) -> (Self, bool) {
        match self {
            Solo(x) => {
                if x >= 10 {
                    let temp = x as f32;
                    let left = (temp / 2.0).floor() as usize;
                    let right = (temp / 2.0).ceil() as usize;

                    return (Pair(Box::new(Solo(left)), Box::new(Solo(right))), true);
                }
                return (Solo(x), false);
            }
            Pair(x, y) => {
                let mut sub_l = *x;
                let mut sub_r = *y;

                let (pair_l, found_l) = sub_l.check_split();
                sub_l = pair_l;
                if found_l {
                    return (Pair(Box::new(sub_l), Box::new(sub_r)), true);
                }
                let (pair_r, found_r) = sub_r.check_split();
                sub_r = pair_r;
                if found_r {
                    return (Pair(Box::new(sub_l), Box::new(sub_r)), true);
                }
                return (Pair(Box::new(sub_l), Box::new(sub_r)), false);
            }
        }
    }

    fn apply_overflow(self, value: usize, left: bool) -> Self {
        match self {
            Solo(x) => Solo(x + value),
            Pair(x, y) => {
                if left {
                    let temp = *x;
                    Pair(Box::new(temp.apply_overflow(value, left)), y)
                } else {
                    let temp = *y;
                    Pair(x, Box::new(temp.apply_overflow(value, left)))
                }
            }
        }
    }

    fn reduce(self) -> Self {
        let mut next = self;
        loop {
            let (step, exploded) = next.check_explosion(0);
            next = step;
            if exploded.is_none() {
                let (step, splitted) = next.check_split();
                next = step;
                if !splitted {
                    break;
                }
            }
        }
        next
    }

    fn get_magnitude(&self) -> usize {
        match self {
            Solo(x) => *x,
            Pair(x, y) => 3 * x.get_magnitude() + 2 * y.get_magnitude(),
        }
    }
}

fn get_pair(input: &[char], index: &mut usize) -> SnailPair {
    let mut l_pair = Solo(0);
    let mut r_pair = Solo(0);

    while input[*index] != ',' {
        match input[*index] {
            '[' => {
                *index += 1;
                l_pair = get_pair(input, index);
            }
            _ => {
                l_pair = Solo(input[*index].to_string().parse::<usize>().unwrap());
            }
        }
        *index += 1;
        if *index >= input.len() {
            return l_pair;
        }
    }

    *index += 1;

    while input[*index] != ']' {
        match input[*index] {
            '[' => {
                *index += 1;
                r_pair = get_pair(input, index);
            }
            _ => {
                r_pair = Solo(input[*index].to_string().parse::<usize>().unwrap());
            }
        }
        *index += 1;
    }
    Pair(Box::new(l_pair), Box::new(r_pair))
}

fn get_input() -> Vec<SnailPair> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            let chara = line.trim().chars().collect::<Vec<_>>();
            let mut count = 0;
            get_pair(&chara, &mut count)
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let operations = get_input();
    let mut total = operations[0].clone();

    for op in operations.into_iter().skip(1) {
        total = total.add(&op);
        total = total.reduce();
    }

    println!("Final operation: {}", total);
    println!("Total magnitude is: {}", total.get_magnitude());

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let operations = get_input();
    let mut max_magni = 0;

    for i in 0..operations.len() {
        for j in 0..operations.len() {
            if i == j {
                continue;
            }
            let a = operations[i].clone();
            let b = operations[j].clone();

            max_magni = max_magni.max(a.add(&b).reduce().get_magnitude());
        }
    }

    println!("Max magni: {}", max_magni);

    Ok(())
}
