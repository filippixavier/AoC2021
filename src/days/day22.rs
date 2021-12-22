use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

static INPUT: &str = include_str!("../../data/day22.input");

type Range = (isize, isize);

fn get_input() -> Vec<(bool, Range, Range, Range)> {
    let reg = Regex::new(r"(on|off).*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)")
        .unwrap();
    let mut operations: Vec<(bool, Range, Range, Range)> = vec![];
    for cap in reg.captures_iter(INPUT) {
        let op = &cap[1] == "on";
        let range_x = (
            cap[2].parse::<isize>().unwrap(),
            cap[3].parse::<isize>().unwrap(),
        );
        let range_y = (
            cap[4].parse::<isize>().unwrap(),
            cap[5].parse::<isize>().unwrap(),
        );
        let range_z = (
            cap[6].parse::<isize>().unwrap(),
            cap[7].parse::<isize>().unwrap(),
        );

        operations.push((op, range_x, range_y, range_z));
    }

    operations
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let operations = get_input();
    let mut cube: HashMap<(isize, isize, isize), bool> = HashMap::new();

    // This is actually very wrong in some inputs cases
    for (status, range_x, range_y, range_z) in operations {
        for x in range_x.0.max(-50)..=range_x.1.min(50) {
            for y in range_y.0.max(-50)..=range_y.1.min(50) {
                for z in range_z.0.max(-50)..=range_z.1.min(50) {
                    let light = cube.entry((x, y, z)).or_insert(false);
                    *light = status;
                }
            }
        }
    }

    let cube_on = cube.values().filter(|&&x| x).count();

    println!(
        "There are {} cubes on in the initialization region",
        cube_on
    );

    Ok(())
}

struct Cuboid {
    p1: (isize, isize, isize),
    p2: (isize, isize, isize),
    lights: bool,
}

impl Cuboid {
    fn area(&self) -> isize {
        let x = self.p2.0 - self.p1.0;
        let y = self.p2.1 - self.p1.1;
        let z = self.p2.2 - self.p1.2;

        x * y * z
    }

    fn intersect(&self, other: &Self) -> Option<Cuboid> {
        let p1 = (
            self.p1.0.max(other.p1.0),
            self.p1.1.max(other.p1.1),
            self.p1.2.max(other.p1.2),
        );
        let p2 = (
            self.p2.0.min(other.p2.0),
            self.p2.1.min(other.p2.1),
            self.p2.2.min(other.p2.2),
        );
        return if p1.0 > p2.0 || p1.1 > p2.1 || p1.2 > p2.2 {
            None
        } else {
            Some(Cuboid {
                p1,
                p2,
                lights: self.lights,
            })
        };
    }

    fn remaining(&self, intersected: &Self) -> Vec<Self> {
        let mut remaining = vec![];
        let mut child = Cuboid {
            p1: (self.p1.0, self.p1.1, intersected.p1.2),
            p2: (intersected.p2.0, intersected.p1.1, intersected.p2.2),
            lights: self.lights,
        };
        if child.p1.1 != child.p2.1 {
            remaining.push(child);
        }
        child = Cuboid {
            p1: (intersected.p2.0, self.p1.1, intersected.p1.2),
            p2: (self.p2.0, intersected.p2.1, intersected.p2.2),
            lights: self.lights,
        };
        if child.p1.0 != child.p2.0 {
            remaining.push(child);
        }
        child = Cuboid {
            p1: (intersected.p1.0, intersected.p2.1, intersected.p1.2),
            p2: (self.p2.0, self.p2.1, intersected.p2.2),
            lights: self.lights,
        };
        if child.p1.1 != child.p2.1 {
            remaining.push(child);
        }
        child = Cuboid {
            p1: (self.p1.0, intersected.p1.1, intersected.p1.2),
            p2: (intersected.p2.0, self.p2.1, intersected.p2.2),
            lights: self.lights,
        };
        if child.p1.0 != child.p2.0 {
            remaining.push(child);
        }
        child = Cuboid {
            p1: (self.p1.0, self.p1.1, self.p1.2),
            p2: (self.p2.0, self.p2.1, intersected.p1.2),
            lights: self.lights,
        };
        if child.p1.2 != child.p2.2 {
            remaining.push(child);
        }
        child = Cuboid {
            p1: (self.p1.0, self.p1.1, intersected.p1.2),
            p2: (self.p2.0, self.p2.1, self.p2.2),
            lights: self.lights,
        };
        if child.p1.2 != child.p2.2 {
            remaining.push(child);
        }
        remaining
    }
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    let operations = get_input();
    let mut cubes: Vec<Cuboid> = vec![];

    let mut lights_count = 0;

    for (lights, range_x, range_y, range_z) in operations {
        let cube = Cuboid {
            p1: (range_x.0, range_y.0, range_z.0),
            p2: (range_x.1, range_y.1, range_z.1),
            lights,
        };

        let mut covered_area = cube.area();

        let mut intersectors: Vec<Cuboid> = vec![];

        for other_cube in cubes.iter().rev() {
            let intersec = cube.intersect(&other_cube);
            if cube.lights && other_cube.lights {}
        }
        cubes.push(cube);
    }

    Ok(())
}
