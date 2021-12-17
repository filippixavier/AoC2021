use regex::Regex;
use std::error::Error;

const INPUT: &str = include_str!("../../data/day17.input");

fn get_input() -> ((isize, isize), (isize, isize)) {
    let re = Regex::new(r"x=(\d+).+?(\d+).+?(-?\d+).+?(-?\d+)").unwrap();
    let mut x_range = (0, 0);
    let mut y_range = (0, 0);
    for cap in re.captures_iter(INPUT) {
        x_range.0 = cap[1].parse().unwrap();
        x_range.1 = cap[2].parse().unwrap();
        y_range.0 = cap[3].parse().unwrap();
        y_range.1 = cap[4].parse().unwrap();
    }

    (x_range, y_range)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let ((x_min, x_max), (y_min, y_max)) = get_input();

    println!("({} {}), ({}, {})", x_min, x_max, y_min, y_max);

    let mut initial_x = 0;
    let mut initial_y = 0;
    let mut max_height = 0;

    for i in 0..x_max {
        if i * (i + 1) / 2 >= x_min {
            initial_x = i;
            break;
        }
    }

    // I know, it's cheating.
    for i in 0..7000 {
        let mut pos_y = i * (i + 1) / 2;
        let mut j = 0;
        let mut went_in = false;

        while pos_y >= y_min {
            pos_y -= j;
            j += 1;
            if pos_y <= y_max && pos_y >= y_min {
                went_in = true;
            }
        }

        if went_in {
            initial_y = i;
            max_height = i * (i + 1) / 2;
        }
    }

    println!(
        "With starting velocity of :({}, {}) it reaches max height of: {}",
        initial_x, initial_y, max_height
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
