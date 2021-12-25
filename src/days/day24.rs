use std::error::Error;

static INPUT: &str = include_str!("../../data/day24.input");

#[derive(Debug, Clone, Copy)]
struct Alu {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl Alu {
    fn new() -> Self {
        Alu {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }
    fn get_value(&self, reg: &str) -> isize {
        if let Ok(value) = reg.parse::<isize>() {
            return value;
        }
        match reg {
            "w" => self.w,
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            _ => unreachable!(),
        }
    }
    fn get_reg(&mut self, reg: &str) -> &mut isize {
        match reg {
            "w" => &mut self.w,
            "x" => &mut self.x,
            "y" => &mut self.y,
            "z" => &mut self.z,
            _ => unreachable!(),
        }
    }
    fn inp(&mut self, reg: &str, input: isize) {
        let reg = self.get_reg(reg);
        *reg = input;
    }
    fn add(&mut self, reg_a: &str, reg_b: &str) {
        let b = self.get_value(reg_b);
        let a = self.get_reg(reg_a);
        *a += b;
    }
    fn mul(&mut self, reg_a: &str, reg_b: &str) {
        let b = self.get_value(reg_b);
        let a = self.get_reg(reg_a);
        *a *= b;
    }
    fn div(&mut self, reg_a: &str, reg_b: &str) -> bool {
        let b = self.get_value(reg_b);
        let a = self.get_reg(reg_a);
        if b == 0 {
            false
        } else {
            *a /= b;
            true
        }
    }
    fn modulo(&mut self, reg_a: &str, reg_b: &str) -> bool {
        let b = self.get_value(reg_b);
        let a = self.get_reg(reg_a);
        if b <= 0 || *a < 0 {
            false
        } else {
            *a %= b;
            true
        }
    }
    fn eql(&mut self, reg_a: &str, reg_b: &str) {
        let b = self.get_value(reg_b);
        let a = self.get_reg(reg_a);

        *a = if *a == b { 1 } else { 0 }
    }
}

type Operation = (&'static str, Vec<&'static str>);

fn get_input() -> Vec<Operation> {
    INPUT
        .trim()
        .lines()
        .into_iter()
        .map(|line| {
            let (operation, operands) = line.trim().split_once(' ').unwrap();
            let operands: Vec<&str> = operands.split_whitespace().collect();
            (operation, operands)
        })
        .collect()
}

pub fn first_star() -> Result<(), Box<dyn Error + 'static>> {
    let mut alus = vec![(Alu::new(), 9, 0, String::new())];
    let operations = get_input();
    let mut valid_id = String::new();

    while !alus.is_empty() {
        let mut crashed = false;
        let (mut alu, mut id_val, mut index, mut id) = alus.pop().unwrap();
        while index < operations.len() {
            let (operation, operators) = &operations[index];
            match *operation {
                "inp" => {
                    if let Ok(value) = &operations[index + 5].1[1].parse::<isize>() {
                        if *value <= 9 {
                            let target_id = (alu.z % 26) + *value;
                            if target_id > 0 && target_id <= 9 {
                                alu.inp(operators[0], target_id);
                                id += &target_id.to_string();
                                index += 1;
                                continue;
                            } else {
                                break;
                            }
                        }
                    }
                    if id_val > 1 {
                        alus.push((alu, id_val - 1, index, id.clone()))
                    }
                    alu.inp(operators[0], id_val);
                    id += &id_val.to_string();
                    id_val = 9;
                }
                "add" => {
                    alu.add(operators[0], operators[1]);
                }
                "mul" => alu.mul(operators[0], operators[1]),
                "div" => {
                    crashed = !alu.div(operators[0], operators[1]);
                    if crashed {
                        break;
                    }
                }
                "mod" => {
                    crashed = !alu.modulo(operators[0], operators[1]);
                    if crashed {
                        break;
                    }
                }
                "eql" => {
                    alu.eql(operators[0], operators[1]);
                }
                _ => unreachable!(),
            }
            index += 1;
        }
        if crashed {
            continue;
        }
        if alu.z == 0 {
            valid_id = id;
            break;
        }
    }

    println!("Biggest valid MONAD: {}", valid_id);

    Ok(())
}

pub fn second_star() -> Result<(), Box<dyn Error + 'static>> {
    Ok(())
}
