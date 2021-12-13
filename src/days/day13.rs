use std::collections::HashSet;
use std::error::Error;

const INPUT: &str = include_str!("../../data/day13.input");

type Map = HashSet<(usize, usize)>;
type Fold = (bool, usize);

fn get_input() -> (Map, Vec<Fold>) {
    let mut map = HashSet::new();
    let mut folds = vec![];
    let lines: Vec<_> = INPUT.trim().lines().collect();
    for line in lines.iter().take_while(|x| !x.is_empty()) {
        let coordinates = line
            .trim()
            .split(',')
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        map.insert((coordinates[0], coordinates[1]));
    }
    for line in lines.iter().skip_while(|x| !x.is_empty()).skip(1) {
        let parts = line.trim().split('=').collect::<Vec<_>>();
        let x_axis = parts[0].contains('x');
        let step = parts[1].parse::<usize>().unwrap();
        folds.push((x_axis, step));
    }

    (map, folds)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (map, folds) = get_input();

    let mut folds = folds.into_iter().rev().collect::<Vec<_>>();

    let (x_axe, value) = folds.pop().unwrap();

    let (fixed, moved): (Vec<_>, Vec<_>) =
        map.into_iter()
            .partition(|(x, y)| if x_axe { *x < value } else { *y < value });

    let map = fixed.into_iter().collect::<HashSet<(usize, usize)>>();
    let moved = moved
        .into_iter()
        .map(|(x, y)| {
            if x_axe {
                (value - (x - value), y)
            } else {
                (x, value - (y - value))
            }
        })
        .collect::<HashSet<(usize, usize)>>();
    let map: HashSet<(usize, usize)> = map.union(&moved).cloned().collect();

    println!("{}", map.len());
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
