use std::{fs::File, io::Read};

pub fn solve() {
    let equations = load_from_file();
    let first_part_result = first_part(&equations);
    println!("First part result: {}", first_part_result);
    let second_part_result = second_part(&equations);
    println!("Second part result: {}", second_part_result);
}

struct Equation {
    test_value: usize,
    numbers: Vec<usize>,
}

enum Operation {
    Add,
    Multiply,
}

enum ExtendedOperation {
    Add,
    Multiply,
    Concatenation,
}

impl Equation {
    fn check_if_can_be_true(&self) -> bool {
        self.can_be_true(1, Operation::Add, self.numbers[0])
            || self.can_be_true(1, Operation::Multiply, self.numbers[0])
    }

    fn can_be_true(&self, idx: usize, op: Operation, current_value: usize) -> bool {
        let next_value = match op {
            Operation::Add => current_value + self.numbers[idx],
            Operation::Multiply => current_value * self.numbers[idx],
        };
        if idx == self.numbers.len() - 1 {
            return self.test_value == next_value;
        }
        self.can_be_true(idx + 1, Operation::Add, next_value)
            || self.can_be_true(idx + 1, Operation::Multiply, next_value)
    }

    fn check_if_can_be_true_with_conc(&self) -> bool {
        self.can_be_true_with_conc(1, ExtendedOperation::Add, self.numbers[0])
            || self.can_be_true_with_conc(1, ExtendedOperation::Multiply, self.numbers[0])
            || self.can_be_true_with_conc(1, ExtendedOperation::Concatenation, self.numbers[0])
    }

    fn can_be_true_with_conc(
        &self,
        idx: usize,
        op: ExtendedOperation,
        current_value: usize,
    ) -> bool {
        let next_value = match op {
            ExtendedOperation::Add => current_value + self.numbers[idx],
            ExtendedOperation::Multiply => current_value * self.numbers[idx],
            ExtendedOperation::Concatenation => {
                let d = Self::count_digits(self.numbers[idx]);
                current_value * 10_usize.pow(d) + self.numbers[idx]
            }
        };
        if idx == self.numbers.len() - 1 {
            return self.test_value == next_value;
        }
        self.can_be_true_with_conc(idx + 1, ExtendedOperation::Add, next_value)
            || self.can_be_true_with_conc(idx + 1, ExtendedOperation::Multiply, next_value)
            || self.can_be_true_with_conc(idx + 1, ExtendedOperation::Concatenation, next_value)
    }

    fn count_digits(n: usize) -> u32 {
        if n == 0 {
            return 1;
        }
        (n as f64).log10().floor() as u32 + 1
    }
}

fn load_from_file() -> Vec<Equation> {
    let mut file = File::open("./input/day_07/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    contents
        .lines()
        .map(|line| {
            let parts: Vec<_> = line.split(": ").collect();
            assert!(parts.len() == 2);
            let test_value = parts[0].parse::<usize>().unwrap();
            let numbers: Vec<_> = parts[1]
                .split(" ")
                .map(|n| n.parse::<usize>().unwrap())
                .collect();
            Equation {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn first_part(equations: &[Equation]) -> usize {
    equations.iter().fold(0, |acc, eq| {
        if eq.check_if_can_be_true() {
            acc + eq.test_value
        } else {
            acc
        }
    })
}

fn second_part(equations: &[Equation]) -> usize {
    equations.iter().fold(0, |acc, eq| {
        if eq.check_if_can_be_true_with_conc() {
            acc + eq.test_value
        } else {
            acc
        }
    })
}
