use std::error::Error;

static INPUT: &str = include_str!("../../data/day10.input");

#[derive(Clone, Copy, Debug, PartialEq)]
enum CHUNKVALUES {
    Paro = 0,
    Brao = 1,
    Curo = 2,
    Cheo = 4,
    Parc = 3,
    Brac = 57,
    Curc = 1197,
    Chec = 25137,
}

use CHUNKVALUES::*;

fn get_input() -> Vec<Vec<CHUNKVALUES>> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|cha| match cha {
                    '{' => Curo,
                    '(' => Paro,
                    '[' => Brao,
                    '<' => Cheo,
                    '}' => Curc,
                    ')' => Parc,
                    ']' => Brac,
                    '>' => Chec,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut score = 0;
    let input = get_input();
    for line in input {
        let mut stored_close = vec![];
        let mut expected_close = None;
        for symbol in line {
            match symbol {
                Paro => {
                    if let Some(exp) = expected_close {
                        stored_close.push(exp)
                    }
                    expected_close = Some(Parc)
                }
                Parc => {
                    if let Some(close) = expected_close {
                        if close != Parc {
                            score += symbol as usize;
                            break;
                        } else if let Some(previous) = stored_close.pop() {
                            expected_close = Some(previous);
                        }
                    }
                }
                Brao => {
                    if let Some(exp) = expected_close {
                        stored_close.push(exp)
                    }
                    expected_close = Some(Brac)
                }
                Brac => {
                    if let Some(close) = expected_close {
                        if close != Brac {
                            score += symbol as usize;
                            break;
                        } else if let Some(previous) = stored_close.pop() {
                            expected_close = Some(previous);
                        }
                    }
                }
                Curo => {
                    if let Some(exp) = expected_close {
                        stored_close.push(exp)
                    }
                    expected_close = Some(Curc)
                }
                Curc => {
                    if let Some(close) = expected_close {
                        if close != Curc {
                            score += symbol as usize;
                            break;
                        } else if let Some(previous) = stored_close.pop() {
                            expected_close = Some(previous);
                        }
                    }
                }
                Cheo => {
                    if let Some(exp) = expected_close {
                        stored_close.push(exp)
                    }
                    expected_close = Some(Chec)
                }
                Chec => {
                    if let Some(close) = expected_close {
                        if close != Chec {
                            score += symbol as usize;
                            break;
                        } else if let Some(previous) = stored_close.pop() {
                            expected_close = Some(previous);
                        }
                    }
                }
            }
        }
    }

    println!("New High Score! {}", score);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
