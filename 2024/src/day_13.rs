use std::{fs::File, io::Read};

pub fn solve() {
    let machines = load_from_file();
    let first_part_result = first_part(&machines);
    println!("First part result: {}", first_part_result);
    let second_part_result = second_part(&machines);
    println!("Second part result: {}", second_part_result);
}

fn load_from_file() -> Vec<Machine> {
    let mut file = File::open("./input/day_13/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .split("\n\n")
        .filter(|&p| !p.trim().is_empty())
        .map(Machine::from)
        .collect()
}

fn first_part(machines: &[Machine]) -> usize {
    machines.iter().fold(0, |acc, machine| {
        let cost = machine.minimal_cost_to_get_prize();
        if let Some(cost) = cost {
            acc + cost
        } else {
            acc
        }
    })
}

fn second_part(machines: &[Machine]) -> usize {
    const DIFF: f64 = 10000000000000.0;
    machines.iter().fold(0, |acc, machine| {
        let mut machine = *machine;
        machine.prize.x += DIFF;
        machine.prize.y += DIFF;
        let cost = machine.minimal_cost_to_get_prize();
        if let Some(cost) = cost {
            acc + cost
        } else {
            acc
        }
    })
}

#[derive(Clone, Copy)]
struct Button {
    cost: usize,
    right: f64,
    forward: f64,
}

impl From<&str> for Button {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split(": ").collect();
        assert_eq!(parts.len(), 2);
        let cost: usize = if parts[0].ends_with("A") {
            3
        } else if parts[0].ends_with("B") {
            1
        } else {
            panic!("Invalid button type")
        };
        let coords: Vec<_> = parts[1].split(", ").collect();
        assert_eq!(coords.len(), 2);
        let right = coords[0][2..].parse::<f64>().unwrap();
        let forward = coords[1][2..].parse::<f64>().unwrap();
        Button {
            cost,
            right,
            forward,
        }
    }
}

#[derive(Clone, Copy)]
struct Prize {
    x: f64,
    y: f64,
}

impl From<&str> for Prize {
    fn from(value: &str) -> Self {
        let numbers: Vec<_> = value.split(": ").collect::<Vec<_>>()[1]
            .split(", ")
            .collect();
        let x = numbers[0][2..].parse::<f64>().unwrap();
        let y = numbers[1][2..].parse::<f64>().unwrap();
        Prize { x, y }
    }
}

#[derive(Clone, Copy)]
struct Machine {
    button_cheap: Button,
    button_exp: Button,
    prize: Prize,
}

impl From<&str> for Machine {
    fn from(value: &str) -> Self {
        let lines: Vec<_> = value.lines().collect();
        assert_eq!(lines.len(), 3);

        let button_0 = Button::from(lines[0]);
        let button_1 = Button::from(lines[1]);

        let (button_cheap, button_exp) = match button_0.cost > button_1.cost {
            true => (button_1, button_0),
            false => (button_0, button_1),
        };

        let prize = Prize::from(lines[2]);
        Machine {
            button_cheap,
            button_exp,
            prize,
        }
    }
}

impl Machine {
    /// Return None if the prize cannot be obtained
    fn minimal_cost_to_get_prize(&self) -> Option<usize> {
        #[allow(non_snake_case)]
        let D = (self.button_cheap.right * self.button_exp.forward)
            - (self.button_cheap.forward * self.button_exp.right);

        #[allow(non_snake_case)]
        let D_cheap =
            (self.prize.x * self.button_exp.forward) - (self.prize.y * self.button_exp.right);

        #[allow(non_snake_case)]
        let D_exp =
            (self.button_cheap.right * self.prize.y) - (self.button_cheap.forward * self.prize.x);

        if D == 0.0 && D_exp == 0.0 && D_cheap == 0.0 {
            return self.minimal_cost_to_get_prize_colinear();
        }

        let c_cheap = (D_cheap / D) as i64;
        let c_exp = (D_exp / D) as i64;

        if c_cheap <= 0 || c_exp <= 0 {
            return None;
        }

        if c_cheap * self.button_cheap.right as i64 + c_exp * self.button_exp.right as i64
            != self.prize.x as i64
            || c_cheap * self.button_cheap.forward as i64 + c_exp * self.button_exp.forward as i64
                != self.prize.y as i64
        {
            return None;
        }

        Some(c_cheap as usize * self.button_cheap.cost + c_exp as usize * self.button_exp.cost)
    }

    fn minimal_cost_to_get_prize_colinear(&self) -> Option<usize> {
        let cost_ratio = self.button_exp.cost as f64 / self.button_cheap.cost as f64;
        let right_ratio = self.button_exp.right / self.button_cheap.right;
        let btn = if right_ratio > cost_ratio {
            self.button_exp
        } else {
            self.button_cheap
        };

        let c = (self.prize.x / btn.right) as i64;

        if c * btn.right as i64 != self.prize.x as i64
            || c * btn.forward as i64 != self.prize.y as i64
        {
            return None;
        }

        Some(c as usize * btn.cost)
    }
}
