use std::error::Error;

const INPUT: &str = include_str!("../../data/day16.input");

fn get_input() -> Vec<char> {
    INPUT
        .trim()
        .chars()
        .map(|x| {
            format!("{:04b}", usize::from_str_radix(&x.to_string(), 16).unwrap())
                .chars()
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect()
}

fn test<I>(iter: &mut I) -> (usize, Vec<u64>)
where
    I: Iterator<Item = char>,
{
    let mut version_sum = usize::from_str_radix(&iter.take(3).collect::<String>(), 2).unwrap();
    let mut current_values: Vec<u64> = vec![];
    let op = usize::from_str_radix(&iter.take(3).collect::<String>(), 2).unwrap();

    match op {
        4 => {
            let mut result = vec![];
            loop {
                let flag = iter.next().unwrap();
                let mut pack = iter.take(4).collect::<Vec<_>>();
                result.append(&mut pack);
                if flag == '0' {
                    current_values
                        .push(u64::from_str_radix(&result.iter().collect::<String>(), 2).unwrap());
                    break;
                }
            }
        }
        _ => {
            let is_len = iter.next().unwrap() == '0';
            let count = if is_len {
                usize::from_str_radix(&iter.take(15).collect::<String>(), 2).unwrap()
            } else {
                usize::from_str_radix(&iter.take(11).collect::<String>(), 2).unwrap()
            };
            if is_len {
                // Produce an error[E0275] compilation error if we attempt to send the take() directly to the function (too much recursion during compilation)
                let mut temp = iter.take(count).collect::<Vec<_>>().into_iter().peekable();
                while temp.peek().is_some() {
                    let (version, mut values) = test(&mut temp);
                    version_sum += version;
                    current_values.append(&mut values);
                }
            } else {
                for _ in 0..count {
                    let (version, mut values) = test(iter);
                    version_sum += version;
                    current_values.append(&mut values);
                }
            }

            match op {
                0 => {
                    let sum: u64 = current_values.iter().sum();
                    current_values = vec![sum];
                }
                1 => {
                    let product: u64 = current_values.iter().product();
                    current_values = vec![product];
                }
                2 => {
                    let min: u64 = *current_values.iter().min().unwrap_or(&0);
                    current_values = vec![min];
                }
                3 => {
                    let max: u64 = *current_values.iter().max().unwrap_or(&u64::MAX);
                    current_values = vec![max];
                }
                5 => {
                    current_values = if current_values[0] > current_values[1] {
                        vec![1]
                    } else {
                        vec![0]
                    }
                }
                6 => {
                    current_values = if current_values[0] < current_values[1] {
                        vec![1]
                    } else {
                        vec![0]
                    }
                }
                7 => {
                    current_values = if current_values[0] == current_values[1] {
                        vec![1]
                    } else {
                        vec![0]
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    (version_sum, current_values)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let packet = get_input();
    let (v, _) = test(&mut packet.into_iter());

    println!("Version sum is: {}", v);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let packet = get_input();
    let (_, sum) = test(&mut packet.into_iter());

    println!("Package sum is: {}", sum[0]);
    Ok(())
}
