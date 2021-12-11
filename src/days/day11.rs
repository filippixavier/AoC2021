use std::error::Error;

const INPUT: &str = include_str!("../../data/day11.input");
const MAX_LEN: usize = 10;

fn get_input() -> [[usize; MAX_LEN]; MAX_LEN] {
    INPUT
        .trim()
        .lines()
        .enumerate()
        .fold([[0; MAX_LEN]; MAX_LEN], |mut acc, (index, line)| {
            acc[index] =
                line.chars()
                    .enumerate()
                    .fold([0; MAX_LEN], |mut sub_acc, (col_no, cha)| {
                        sub_acc[col_no] = cha.to_string().parse().unwrap_or(0);
                        sub_acc
                    });
            acc
        })
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut octopuses = get_input();
    let mut shine_spark_count = 0;
    for _ in 0..100 {
        let mut shiner = vec![];
        for (x, line) in octopuses.iter_mut().enumerate() {
            for (y, col) in line.iter_mut().enumerate() {
                *col += 1;
                if *col == 10 {
                    shiner.push((x, y));
                }
            }
        }

        while !shiner.is_empty() {
            shine_spark_count += 1;
            let (line, col) = shiner.pop().unwrap();
            if line > 0 {
                let x = line - 1;
                if col > 0 {
                    let y = col - 1;
                    octopuses[x][y] += 1;
                    if octopuses[x][y] == 10 {
                        shiner.push((x, y));
                    }
                }
                octopuses[x][col] += 1;
                if octopuses[x][col] == 10 {
                    shiner.push((x, col));
                }
                if col < MAX_LEN - 1 {
                    let y = col + 1;
                    octopuses[x][y] += 1;
                    if octopuses[x][y] == 10 {
                        shiner.push((x, y));
                    }
                }
            }
            if line < MAX_LEN - 1 {
                let x = line + 1;
                if col > 0 {
                    let y = col - 1;
                    octopuses[x][y] += 1;
                    if octopuses[x][y] == 10 {
                        shiner.push((x, y));
                    }
                }
                octopuses[x][col] += 1;
                if octopuses[x][col] == 10 {
                    shiner.push((x, col));
                }
                if col < MAX_LEN - 1 {
                    let y = col + 1;
                    octopuses[x][y] += 1;
                    if octopuses[x][y] == 10 {
                        shiner.push((x, y));
                    }
                }
            }
            if col > 0 {
                let y = col - 1;
                octopuses[line][y] += 1;
                if octopuses[line][y] == 10 {
                    shiner.push((line, y));
                }
            }
            if col < MAX_LEN - 1 {
                let y = col + 1;
                octopuses[line][y] += 1;
                if octopuses[line][y] == 10 {
                    shiner.push((line, y));
                }
            }
        }

        for line in octopuses.iter_mut() {
            for col in line.iter_mut() {
                if *col >= 10 {
                    *col = 0;
                }
            }
        }
    }

    println!("Sparky count: {}", shine_spark_count);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
