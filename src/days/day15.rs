use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::error::Error;

const INPUT: &str = include_str!("../../data/day15.input");

fn get_input() -> Vec<Vec<u8>> {
    INPUT
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|risk| risk.to_string().parse())
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect()
}

// Algorithm fetched from https://doc.rust-lang.org/std/collections/binary_heap/index.html

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(risk_level: &[Vec<u8>]) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..(risk_level.len() * risk_level[0].len()))
        .map(|_| usize::MAX)
        .collect();
    let lines_num = risk_level.len();
    let start = (0, 0);
    let goal = (risk_level.len() - 1, risk_level[0].len() - 1);

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start.0 * lines_num + start.1] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position.0 * lines_num + position.1] {
            continue;
        }

        let neighbors: Vec<_> = vec![
            (position.0.saturating_sub(1), position.1),
            (position.0 + 1, position.1),
            (position.0, position.1.saturating_sub(1)),
            (position.0, position.1 + 1),
        ]
        .into_iter()
        .filter(|&x| x != position)
        .collect();

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neighbor in neighbors {
            let edge_cost = if let Some(line) = risk_level.get(neighbor.0) {
                if let Some(value) = line.get(neighbor.1) {
                    *value
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let next = State {
                cost: cost + edge_cost as usize,
                position: neighbor,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[neighbor.0 * lines_num + neighbor.1] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[neighbor.0 * lines_num + neighbor.1] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

/*fn a_star(risk_level: &[Vec<u8>]) -> usize {
    let mut open_set = vec![((0, 0), 0)];

    let len = risk_level.len();

    let mut g_score: Vec<usize> = (0..(risk_level.len() * risk_level[0].len()))
        .map(|_| usize::MAX)
        .collect();
    g_score[0] = 0;

    while !open_set.is_empty() {
        let (current, score) = open_set.pop().unwrap();
        let current_score = g_score[current.0 * len + current.1];
        if score > current_score {
            continue;
        }

        if current == (risk_level.len() - 1, risk_level[0].len() - 1) {
            return current_score;
        }

        let neighbors: Vec<_> = vec![
            (current.0.saturating_sub(1), current.1),
            (current.0 + 1, current.1),
            (current.0, current.1.saturating_sub(1)),
            (current.0, current.1 + 1),
        ]
        .into_iter()
        .filter(|&x| x != current)
        .collect();

        for neighbor in neighbors {
            let d = if let Some(line) = risk_level.get(neighbor.0) {
                if let Some(value) = line.get(neighbor.1) {
                    *value
                } else {
                    continue;
                }
            } else {
                continue;
            };

            let tentative_score = current_score + d as usize;
            if tentative_score < g_score[&neighbor.0 * len + &neighbor.1] {
                g_score[&neighbor.0 * len + &neighbor.1] = tentative_score;
                open_set.push((neighbor, tentative_score));
                open_set.sort_unstable_by(|(a, _), (b, _)| {
                    g_score[b.0 * len + b.1].cmp(&g_score[a.0 * len + a.1])
                });
            }
        }
    }

    0
}*/

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let risk_level = get_input();
    println!(
        "Lowest total risk: {}",
        shortest_path(&risk_level).unwrap_or(0)
    );
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let risk_level = get_input();
    let mut big_risk = vec![];

    for risk_line in risk_level.iter() {
        let mut line = vec![];
        for j in 0..5 {
            let mut sub = risk_line
                .iter()
                .cloned()
                .map(|x| if x + j > 9 { x + j - 9 } else { x + j })
                .collect();
            line.append(&mut sub);
        }
        big_risk.push(line);
    }

    for i in 1..5 {
        for j in 0..risk_level.len() {
            let line = big_risk[j]
                .iter()
                .cloned()
                .map(|x| {
                    if x + i as u8 > 9 {
                        x + i - 9
                    } else {
                        x + i as u8
                    }
                })
                .collect();
            big_risk.push(line);
        }
    }
    println!(
        "Lowest total risk in big map: {}",
        shortest_path(&big_risk).unwrap_or(0)
    );
    Ok(())
}
