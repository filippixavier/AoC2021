use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day22.input");

type Range = (isize, isize);

fn get_input() -> Vec<(bool, Range, Range, Range)> {
    let reg = Regex::new(r"(on|off).*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)")
        .unwrap();
    let mut operations: Vec<(bool, Range, Range, Range)> = vec![];
    for cap in reg.captures_iter(INPUT) {
        let op = &cap[1] == "on";
        let range_x = (
            cap[2].parse::<isize>().unwrap(),
            cap[3].parse::<isize>().unwrap(),
        );
        let range_y = (
            cap[4].parse::<isize>().unwrap(),
            cap[5].parse::<isize>().unwrap(),
        );
        let range_z = (
            cap[6].parse::<isize>().unwrap(),
            cap[7].parse::<isize>().unwrap(),
        );

        operations.push((op, range_x, range_y, range_z));
    }

    operations
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let operations = get_input();
    let mut cube: HashMap<(isize, isize, isize), bool> = HashMap::new();

    for (status, range_x, range_y, range_z) in operations {
        for x in range_x.0.max(-50)..=range_x.1.min(50) {
            for y in range_y.0.max(-50)..=range_y.1.min(50) {
                for z in range_z.0.max(-50)..=range_z.1.min(50) {
                    let light = cube.entry((x, y, z)).or_insert(false);
                    *light = status;
                }
            }
        }
    }

    let cube_on = cube.values().filter(|&&x| x).count();

    println!(
        "There are {} cubes on in the initialization region",
        cube_on
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
