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

fn get_values_by_len(packet: &mut Vec<char>) -> (usize, Vec<u64>) {
    let mut packet_version = 0;
    let mut values = vec![];
    let mut iter_packet = packet.clone().into_iter();

    let bin_len = iter_packet.by_ref().take(15).collect::<String>();
    let len = usize::from_str_radix(&bin_len, 2).unwrap();

    let mut sub_packets: Vec<char> = iter_packet.by_ref().take(len).collect();
    while !sub_packets.is_empty() {
        let (version, mut sub_values) = get_total_version(&mut sub_packets);
        packet_version += version;
        values.append(&mut sub_values);
    }
    *packet = iter_packet.collect();

    (packet_version, values)
}

fn get_values_by_count(packet: &mut Vec<char>) -> (usize, Vec<u64>) {
    let mut packet_version = 0;
    let mut values = vec![];
    let mut iter_packet = packet.clone().into_iter();

    let count =
        usize::from_str_radix(&iter_packet.by_ref().take(11).collect::<String>(), 2).unwrap();

    *packet = iter_packet.collect();
    for _ in 0..count {
        let (version, mut sub_values) = get_total_version(packet);
        packet_version += version;
        values.append(&mut sub_values);
    }

    (packet_version, values)
}

fn get_op_values(packet: &mut Vec<char>) -> (usize, Vec<u64>) {
    let mut packet_version = 0;
    let mut current_values = vec![];

    let mut iter_packet = packet.clone().into_iter();

    let len_type_id = iter_packet.next().unwrap_or('0');
    *packet = iter_packet.collect();
    if len_type_id == '0' {
        let (version, mut values) = get_values_by_len(packet);
        packet_version += version;
        current_values.append(&mut values);
    } else {
        let (version, mut values) = get_values_by_count(packet);
        packet_version += version;
        current_values.append(&mut values);
    }
    (packet_version, current_values)
}

fn get_total_version(packet: &mut Vec<char>) -> (usize, Vec<u64>) {
    let mut iter_packet = packet.clone().into_iter();
    let mut current_values = vec![];
    let mut packet_version =
        usize::from_str_radix(&iter_packet.by_ref().take(3).collect::<String>(), 2).unwrap();
    let id = usize::from_str_radix(&iter_packet.by_ref().take(3).collect::<String>(), 2).unwrap();

    *packet = iter_packet.collect();

    match id {
        0 => {
            let (version, mut values) = get_op_values(packet);
            packet_version += version;
            current_values.append(&mut values);
            let sum: u64 = current_values.iter().sum();
            current_values = vec![sum];
        }
        1 => {
            let (version, mut values) = get_op_values(packet);
            packet_version += version;
            current_values.append(&mut values);
            let product: u64 = current_values.iter().product();
            current_values = vec![product];
        }
        2 => {
            let (version, mut values) = get_op_values(packet);
            packet_version += version;
            current_values.append(&mut values);
            let min: u64 = *current_values.iter().min().unwrap_or(&0);
            current_values = vec![min];
        }
        3 => {
            let (version, mut values) = get_op_values(packet);
            packet_version += version;
            current_values.append(&mut values);
            let max: u64 = *current_values.iter().max().unwrap_or(&u64::MAX);
            current_values = vec![max];
        }
        4 => {
            let mut total = vec![];
            iter_packet = packet.clone().into_iter();
            loop {
                let x: Vec<char> = iter_packet.by_ref().take(5).collect();
                let lead = x[0];
                let mut value: Vec<char> = x[1..].iter().copied().collect();
                total.append(&mut value);
                if lead == '0' {
                    current_values.push(
                        u64::from_str_radix(&total.into_iter().collect::<String>(), 2).unwrap(),
                    );
                    *packet = iter_packet.collect();
                    break;
                }
            }
        }
        5 => {
            let (version, mut values) = get_op_values(packet);
            packet_version += version;
            current_values.append(&mut values);
            current_values = if current_values[0] > current_values[1] {
                vec![1]
            } else {
                vec![0]
            }
        }
        6 => {
            let (version, mut values) = get_op_values(packet);
            packet_version += version;
            current_values.append(&mut values);
            current_values = if current_values[0] < current_values[1] {
                vec![1]
            } else {
                vec![0]
            }
        }
        7 => {
            let (version, mut values) = get_op_values(packet);
            packet_version += version;
            current_values.append(&mut values);
            current_values = if current_values[0] == current_values[1] {
                vec![1]
            } else {
                vec![0]
            }
        }
        _ => unreachable!(),
    };

    (packet_version, current_values)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut packet = get_input();
    let (total_version, _) = get_total_version(&mut packet);

    println!("Version sum is: {}", total_version);
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut packet = get_input();
    let (_, total_sum) = get_total_version(&mut packet);

    println!("Version sum is: {}", total_sum[0]);
    Ok(())
}
