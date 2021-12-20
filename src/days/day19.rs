use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::error::Error;

const INPUT: &str = include_str!("../../data/day19.input");

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Point(i32, i32, i32);

impl Point {
    fn rotate_x(&self) -> Self {
        Point(self.0, -self.2, self.1)
    }
    fn rotate_y(&self) -> Self {
        Point(self.2, self.1, -self.0)
    }
    fn rotate_z(&self) -> Self {
        Point(self.1, -self.0, self.2)
    }
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl std::ops::Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Point(-self.0, -self.1, -self.2)
    }
}

type Anchor = (Point, Point, Point);

type Coordinates = Vec<Point>;

#[derive(Debug)]
struct Probe {
    beacons: HashSet<Point>,
    anchors: Vec<Anchor>,
}

impl FromIterator<Point> for Probe {
    fn from_iter<I: IntoIterator<Item = Point>>(coordinates: I) -> Self {
        let mut beacons: HashSet<Point> = HashSet::new();
        let anchors;

        for coordinate in coordinates {
            beacons.insert(coordinate);
        }

        let coordinates: Coordinates = beacons.clone().into_iter().collect();

        anchors = Probe::get_sorted_anchors(coordinates);

        Probe { beacons, anchors }
    }
}

impl Probe {
    fn get_sorted_anchors(mut coordinates: Vec<Point>) -> Vec<Anchor> {
        let mut anchors = vec![];

        coordinates.sort_unstable_by(|x, y| {
            let a = x.0.abs() + x.1.abs() + x.2.abs();
            let b = y.0.abs() + y.1.abs() + y.2.abs();

            b.cmp(&a)
        });

        while coordinates.len() > 1 {
            let coordinate = coordinates.pop().unwrap();
            for other in &coordinates {
                let dist = (
                    other.0 - coordinate.0,
                    other.1 - coordinate.1,
                    other.2 - coordinate.2,
                );
                anchors.push((coordinate, *other, Point(dist.0, dist.1, dist.2)))
            }
        }

        anchors
    }

    /**
     * Check if two probes "collides" by checking the vectors created by each points. If two coefficient tuples are identical, it may indicate that the two corresponding points overlap after rotations
     */
    fn common_anchors(&self, other: &Self) -> Option<Vec<[Anchor; 2]>> {
        let mut anchors = vec![];

        for anchor in &self.anchors {
            // The abs basically convert to a rotation
            let mut dists_abs = [anchor.2 .0.abs(), anchor.2 .1.abs(), anchor.2 .2.abs()];
            // Ordering the coefficient is also equivalent to a rotation by doing that to both vector, we put them both in the same relative plane
            dists_abs.sort_unstable();
            for other_anchor in &other.anchors {
                let mut other_dists_abs = [
                    other_anchor.2 .0.abs(),
                    other_anchor.2 .1.abs(),
                    other_anchor.2 .2.abs(),
                ];
                other_dists_abs.sort_unstable();
                if dists_abs == other_dists_abs {
                    anchors.push([*anchor, *other_anchor]);
                }
            }
        }

        if anchors.is_empty() {
            None
        } else {
            Some(anchors)
        }
    }

    fn get_transforms(candidates: &[[Anchor; 2]]) -> HashMap<(usize, usize, usize), Point> {
        let mut rotations: HashSet<(usize, usize, usize)> = HashSet::new();
        let mut transforms: HashMap<(usize, usize, usize), Point> = HashMap::new();
        for [vec_start, vec_end] in candidates {
            let mut valid_rots = HashSet::new();
            let (mut p_a, mut p_b, _) = vec_end;
            let target_dist = vec_start.2;
            // Check with redundencies
            for x in 1..5 {
                p_a = p_a.rotate_x();
                p_b = p_b.rotate_x();
                for y in 1..5 {
                    p_a = p_a.rotate_y();
                    p_b = p_b.rotate_y();
                    for z in 1..5 {
                        let rot = (x % 4, y % 4, z % 4);
                        p_a = p_a.rotate_z();
                        p_b = p_b.rotate_z();
                        let dist = p_b - p_a;
                        if dist == target_dist {
                            valid_rots.insert(rot);
                            let coef = vec_start.0 - p_a;
                            transforms.insert(rot, coef);
                        } else if -dist == target_dist {
                            valid_rots.insert(rot);
                            let coef = vec_start.0 - p_b;
                            transforms.insert(rot, coef);
                        }
                    }
                }
            }

            if rotations.is_empty() {
                rotations = valid_rots;
            } else {
                rotations = rotations.intersection(&valid_rots).cloned().collect();
            }
        }
        transforms.retain(|k, _| rotations.contains(k));
        transforms
    }

    fn merge(&self, other: &Self, min_count: usize) -> Option<Self> {
        let mut merged = Probe {
            beacons: self.beacons.clone(),
            anchors: vec![],
        };

        if let Some(anchors) = self.common_anchors(other) {
            let transforms = Probe::get_transforms(&anchors);
            if transforms.is_empty() {
                return None;
            }
            let (rotation, translate) = transforms.iter().collect::<Vec<_>>()[0];
            for point in other.beacons.iter() {
                let mut translated = *point;
                for _ in 1..=rotation.0 {
                    translated = translated.rotate_x();
                }
                for _ in 1..=rotation.1 {
                    translated = translated.rotate_y();
                }
                for _ in 1..=rotation.2 {
                    translated = translated.rotate_z();
                }
                translated = translated + *translate;
                merged.beacons.insert(translated);
            }
            if merged.beacons.intersection(&self.beacons).count() < min_count {
                return None;
            }

            let coordinates: Coordinates = merged.beacons.clone().into_iter().collect();
            merged.anchors = Probe::get_sorted_anchors(coordinates);
        } else {
            return None;
        }

        Some(merged)
    }
}

fn get_input() -> VecDeque<Probe> {
    let mut iter_prob = INPUT.trim().lines().peekable();
    let mut probes: Vec<Vec<&str>> = vec![];

    loop {
        if iter_prob.peek().is_none() {
            break;
        }
        iter_prob.by_ref().next();
        probes.push(iter_prob.by_ref().take_while(|x| !x.is_empty()).collect());
    }

    probes
        .into_iter()
        .map(|probe| {
            probe
                .into_iter()
                .map(|line| {
                    let x = line
                        .trim()
                        .split(',')
                        .map(|num| num.parse::<i32>().unwrap())
                        .enumerate()
                        .fold([0i32; 3], |mut acc, (i, curr)| {
                            acc[i] = curr;
                            acc
                        });
                    Point(x[0], x[1], x[2])
                })
                .collect::<Probe>()
        })
        .collect::<VecDeque<Probe>>()
}
pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut probes = get_input();
    let mut placed_prob = probes.pop_front().unwrap();

    while !probes.is_empty() {
        let current = probes.pop_front().unwrap();
        if let Some(merged) = placed_prob.merge(&current, 12) {
            placed_prob = merged;
        } else {
            probes.push_back(current);
        }
    }
    println!("{}", placed_prob.beacons.len());
    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
