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

use std::collections::HashMap;
fn get_universes_per_dice_sum() -> Vec<(u8, u8)> {
    let mut proba = HashMap::new();
    for x in 1..=3 {
        for y in 1..=3 {
            for z in 1..=3 {
                let count = proba.entry((x + y + z) as u8).or_insert(0);
                *count += 1;
            }
        }
    }
    proba.into_iter().collect()
}

fn recurse_proba(
    score: [usize; 2],
    position: [usize; 2],
    universes_per_dice_sum: &[(u8, u8)],
    player: usize,
    cache: &mut HashMap<(usize, usize, usize, usize, usize), [u64; 2]>,
) -> [u64; 2] {
    if let Some(universes) = cache.get(&(score[0], score[1], position[0], position[1], player)) {
        return *universes;
    }

    let mut total_winning_universes = [0; 2];

    for &(dice_result, universes_generated) in universes_per_dice_sum {
        let mut sub_score = score;
        let universes_generated = universes_generated as u64;

        let mut sub_position = position;
        sub_position[player] = position[player] + dice_result as usize;

        sub_position[player] = if sub_position[player] > 10 {
            sub_position[player] - 10
        } else {
            sub_position[player]
        };

        sub_score[player] += sub_position[player];

        if sub_score[player] >= 21 {
            total_winning_universes[player] += universes_generated;
        } else {
            let sub_total = recurse_proba(
                sub_score,
                sub_position,
                universes_per_dice_sum,
                (player + 1) % 2,
                cache,
            );
            total_winning_universes[0] += sub_total[0] * universes_generated;
            total_winning_universes[1] += sub_total[1] * universes_generated;
        }
    }

    cache.insert(
        (score[0], score[1], position[0], position[1], player),
        total_winning_universes,
    );

    total_winning_universes
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let position = get_input();
    let proba = get_universes_per_dice_sum();
    let score = [0; 2];

    let mut cache: HashMap<(usize, usize, usize, usize, usize), [u64; 2]> = HashMap::new();

    let total_universes = recurse_proba(score, position, &proba, 0, &mut cache);

    println!(
        "Best player wins in {} universes",
        total_universes.iter().max().unwrap()
    );

    Ok(())
}
