use std::collections::HashMap;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day20.input");

type LightMap = HashMap<(isize, isize), bool>;

fn get_input() -> (LightMap, [bool; 512]) {
    let mut lines = INPUT.trim().lines();
    let mut map = HashMap::new();

    let enhancer = lines.next().unwrap();
    let enhancer = enhancer.trim().chars().map(|x| x == '#').enumerate().fold(
        [false; 512],
        |mut acc, (i, x)| {
            acc[i] = x;
            acc
        },
    );
    lines.next();
    for (y, line) in lines.enumerate() {
        for (x, val) in line.trim().chars().enumerate() {
            map.insert((x as isize, y as isize), val == '#');
        }
    }
    (map, enhancer)
}

fn get_new_point(
    coordinates: &[(isize, isize)],
    map: &LightMap,
    enhancer: &[bool; 512],
    default_value: bool,
) -> bool {
    let mut value = 0;
    for coordinate in coordinates {
        value <<= 1;
        let on = map.get(coordinate).unwrap_or(&default_value);
        if *on {
            value += 1;
        }
    }
    enhancer[value]
}

fn get_all_neighbors(coordinate: (isize, isize)) -> Vec<(isize, isize)> {
    let neighbors: [(isize, isize); 9] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (0, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    neighbors
        .iter()
        .map(|n| (coordinate.0 + n.0, coordinate.1 + n.1))
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut map, enhancer) = get_input();

    // Value of a random point at infinite distance
    let mut default_value = false;

    for _ in 0..2 {
        let mut next = HashMap::new();
        let mut lights = map.keys().collect::<Vec<_>>();
        lights.sort_unstable_by(|(x_a, y_a), (x_b, y_b)| {
            if x_a != x_b {
                x_a.cmp(x_b)
            } else {
                y_a.cmp(y_b)
            }
        });
        let (min, max) = (*lights[0], **lights.last().unwrap());
        for light in lights {
            let neighbors = get_all_neighbors(*light);
            next.insert(
                *light,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
        }

        for x in (min.0 - 1)..=(max.0 + 1) {
            let point_min = (x, min.1 - 1);
            let point_max = (x, max.1 + 1);

            let neighbors = get_all_neighbors(point_min);
            next.insert(
                point_min,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
            let neighbors = get_all_neighbors(point_max);
            next.insert(
                point_max,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
        }

        for y in (min.1 - 1)..(max.1 + 1) {
            let point_min = (min.0 - 1, y);
            let point_max = (max.0 + 1, y);

            let neighbors = get_all_neighbors(point_min);
            next.insert(
                point_min,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
            let neighbors = get_all_neighbors(point_max);
            next.insert(
                point_max,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
        }

        // compute new default value, since it may have changed thanks to the enhancer;
        let neighbors = get_all_neighbors((0, 0));
        default_value = get_new_point(&neighbors, &HashMap::new(), &enhancer, default_value);
        map = next;
    }
    let lights_on = map.values().filter(|x| **x).count();

    println!("Lights on: {}", lights_on);
    // 5171 too low

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let (mut map, enhancer) = get_input();

    // Value of a random point at infinite distance
    let mut default_value = false;

    for _ in 0..50 {
        let mut next = HashMap::new();
        let mut lights = map.keys().collect::<Vec<_>>();
        lights.sort_unstable_by(|(x_a, y_a), (x_b, y_b)| {
            if x_a != x_b {
                x_a.cmp(x_b)
            } else {
                y_a.cmp(y_b)
            }
        });
        let (min, max) = (*lights[0], **lights.last().unwrap());
        for light in lights {
            let neighbors = get_all_neighbors(*light);
            next.insert(
                *light,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
        }

        for x in (min.0 - 1)..=(max.0 + 1) {
            let point_min = (x, min.1 - 1);
            let point_max = (x, max.1 + 1);

            let neighbors = get_all_neighbors(point_min);
            next.insert(
                point_min,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
            let neighbors = get_all_neighbors(point_max);
            next.insert(
                point_max,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
        }

        for y in (min.1 - 1)..(max.1 + 1) {
            let point_min = (min.0 - 1, y);
            let point_max = (max.0 + 1, y);

            let neighbors = get_all_neighbors(point_min);
            next.insert(
                point_min,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
            let neighbors = get_all_neighbors(point_max);
            next.insert(
                point_max,
                get_new_point(&neighbors, &map, &enhancer, default_value),
            );
        }

        // compute new default value, since it may have changed thanks to the enhancer;
        let neighbors = get_all_neighbors((0, 0));
        default_value = get_new_point(&neighbors, &HashMap::new(), &enhancer, default_value);
        map = next;
    }
    let lights_on = map.values().filter(|x| **x).count();

    println!("Lights on: {}", lights_on);
    Ok(())
}
