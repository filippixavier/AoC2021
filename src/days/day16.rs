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

fn get_total_version(packet: &mut Vec<char>) -> Option<usize> {
    let mut iter_packet = packet.clone().into_iter();
    let mut total_skip = 0;
    let mut packet_version = if let Ok(x) =
        usize::from_str_radix(&iter_packet.by_ref().take(3).collect::<String>(), 2)
    {
        total_skip += 3;
        x
    } else {
        return None;
    };
    let id = if let Ok(x) =
        usize::from_str_radix(&iter_packet.by_ref().take(3).collect::<String>(), 2)
    {
        total_skip += 3;
        x
    } else {
        return None;
    };
    match id {
        4 => loop {
            let x: Vec<char> = iter_packet.by_ref().take(5).collect();
            total_skip += 5;
            if x[0] == '0' {
                break;
            }
        },
        _ => {
            let len_type_id = iter_packet.next().unwrap_or('0');
            total_skip += 1;
            if len_type_id == '0' {
                let len = if let Ok(x) =
                    usize::from_str_radix(&iter_packet.by_ref().take(15).collect::<String>(), 2)
                {
                    total_skip += 15;
                    x
                } else {
                    return None;
                };
                let mut sub_packets: Vec<char> = iter_packet.take(len).collect();
                total_skip += len;
                while !sub_packets.is_empty() {
                    if let Some(sub_ver) = get_total_version(&mut sub_packets) {
                        packet_version += sub_ver;
                    } else {
                        break;
                    }
                }
            } else {
                let count = if let Ok(x) =
                    usize::from_str_radix(&iter_packet.by_ref().take(11).collect::<String>(), 2)
                {
                    total_skip += 11;
                    x
                } else {
                    return None;
                };
                *packet = packet.iter().skip(total_skip).copied().collect();

                total_skip = 0;
                for _ in 0..count {
                    packet_version += get_total_version(packet).unwrap();
                }
            }
        }
    };

    *packet = packet.iter().skip(total_skip).copied().collect();

    Some(packet_version)
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut packet = get_input();
    let total_version = get_total_version(&mut packet);

    println!("Version sum is: {}", total_version.unwrap_or(0));
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
