use std::error::Error;

static INPUT: &str = include_str!("../../data/day21.input");

fn get_input() -> [usize; 2] {
    let lines = INPUT.trim().lines().collect::<Vec<_>>();

    let nums = lines
        .iter()
        .map(|line| {
            line.trim()
                .split(' ')
                .collect::<Vec<_>>()
                .last()
                .unwrap()
                .parse()
                .unwrap()
        })
        .collect::<Vec<usize>>();
    [nums[0], nums[1]]
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut position = get_input();
    let mut dice = vec![1, 2, 3];
    let mut score = [0; 2];
    let mut player = 0;
    let mut count = 0;

    loop {
        let sum = dice.iter().sum::<usize>() % 10;
        count += 3;
        position[player] += sum;
        if position[player] > 10 {
            position[player] -= 10;
        }
        score[player] += position[player];
        if score[player] >= 1000 {
            break;
        }
        dice = dice
            .into_iter()
            .map(|x| if x + 3 > 100 { x + 3 - 100 } else { x + 3 })
            .collect();
        player = (player + 1) % 2;
    }

    println!(
        "Player {} lose, score status: {}",
        ((player + 1) % 2) + 1,
        score[(player + 1) % 2] * count
    );

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
