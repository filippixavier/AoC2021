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
    let ((x_min, x_max), (y_min, y_max)) = get_input();

    let mut all_x = vec![];
    let mut all_y = vec![];

    let count = (1..=x_max).collect::<Vec<isize>>();

    for i in count.iter().rev() {
        let mut vel_x = *i;
        let mut pos = 0;
        while vel_x > 0 {
            pos += vel_x;
            if pos > x_max {
                break;
            }
            if pos >= x_min && pos <= x_max {
                all_x.push(i);
                break;
            }
            vel_x -= 1;
        }
    }

    for star_vel in y_min..7_000 {
        let mut pos = 0;
        let mut vel = star_vel;

        while pos > y_min {
            pos += vel;
            vel -= 1;
            if pos <= y_max && pos >= y_min {
                all_y.push(star_vel);
                break;
            }
        }
    }

    let mut count_unique = 0;
    let mut all_cut = vec![];

    for &vel_x in all_x {
        for &vel_y in &all_y {
            let mut vel = (vel_x, vel_y);
            let mut pos = (0, 0);

            while pos.0 <= x_max && pos.1 >= y_min {
                pos.0 += vel.0;
                pos.1 += vel.1;
                vel.0 -= if vel.0 > 0 { 1 } else { 0 };
                vel.1 -= 1;
                if pos.0 >= x_min && pos.0 <= x_max && pos.1 >= y_min && pos.1 <= y_max {
                    count_unique += 1;
                    all_cut.push((vel_x, vel_y));
                    break;
                }
            }
        }
    }

    println!("Unique valid velocity: {}", count_unique);

    Ok(())
}
