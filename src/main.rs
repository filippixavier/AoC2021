use std::error::Error;
use std::io::{self};
use std::time::Instant;

mod days;

type Star = fn() -> Result<(), Box<dyn Error + 'static>>;

fn default_star() -> Result<(), Box<dyn Error + 'static>> {
    println!("Invalid day or not published yet");
    Ok(())
}

fn main() {
    let mut buffer = String::new();
    let mut day: usize = 1;

    println!("Welcome to Advent of Code, 2021 edition!");
    println!("Please enter the day number: ");

    match io::stdin().read_line(&mut buffer) {
        Ok(_) => {
            // Must use trim_end or the line ending might mess up with the parsing function
            day = buffer.trim_end().parse().unwrap_or(1);
            println!("Attempting to run day{}...", day);
        }
        Err(error) => println!("Error: {}, defaulting to day1", error),
    }

    let first_star: Star;
    let second_star: Star;
    match day {
        1 => {
            first_star = days::day1::first_star;
            second_star = days::day1::second_star;
        }
        2 => {
            first_star = days::day2::first_star;
            second_star = days::day2::second_star;
        }
        3 => {
            first_star = days::day3::first_star;
            second_star = days::day3::second_star;
        }
        4 => {
            first_star = days::day4::first_star;
            second_star = days::day4::second_star;
        }
        5 => {
            first_star = days::day5::first_star;
            second_star = days::day5::second_star;
        }
        6 => {
            first_star = days::day6::first_star;
            second_star = days::day6::second_star;
        }
        7 => {
            first_star = days::day7::first_star;
            second_star = days::day7::second_star;
        }
        8 => {
            first_star = days::day8::first_star;
            second_star = days::day8::second_star;
        }
        9 => {
            first_star = days::day9::first_star;
            second_star = days::day9::second_star;
        }
        10 => {
            first_star = days::day10::first_star;
            second_star = days::day10::second_star;
        }
        11 => {
            first_star = days::day11::first_star;
            second_star = days::day11::second_star;
        }
        12 => {
            first_star = days::day12::first_star;
            second_star = days::day12::second_star;
        }
        13 => {
            first_star = days::day13::first_star;
            second_star = days::day13::second_star;
        }
        14 => {
            first_star = days::day14::first_star;
            second_star = days::day14::second_star;
        }
        15 => {
            first_star = days::day15::first_star;
            second_star = days::day15::second_star;
        }
        16 => {
            first_star = days::day16::first_star;
            second_star = days::day16::second_star;
        }
        17 => {
            first_star = days::day17::first_star;
            second_star = days::day17::second_star;
        }
        18 => {
            first_star = days::day18::first_star;
            second_star = days::day18::second_star;
        }
        19 => {
            first_star = days::day19::first_star;
            second_star = days::day19::second_star;
        }
        20 => {
            first_star = days::day20::first_star;
            second_star = days::day20::second_star;
        }
        _ => {
            println!("Executing nothing");
            first_star = default_star;
            second_star = default_star;
        }
    }

    let now = Instant::now();
    match first_star() {
        Err(x) => {
            println!("Error: {:?}", x);
        }
        _ => {
            println!("First star: Success!");
        }
    }

    match second_star() {
        Err(x) => {
            println!("Error {:?}", x);
        }
        _ => {
            println!("Second star: Success!");
        }
    }

    let end = now.elapsed();
    println!(
        "Duration: {}sec {}millisec",
        end.as_secs(),
        end.subsec_millis()
    );
}
