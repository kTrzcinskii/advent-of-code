use std::{fs::File, io::Read};

use regex::Regex;

pub fn solve() {
    let content = load_from_file();
    let first_part_result = first_part(&content);
    let second_part_result = second_part(&content);
    println!("First part: {}", first_part_result);
    println!("Second part: {}", second_part_result);
}

fn load_from_file() -> String {
    let mut file = File::open("./input/day_03/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn first_part(input: &str) -> u32 {
    let skip_count = if input.starts_with("mul") { 0 } else { 1 };
    let re = Regex::new(r"^\((\d+),(\d+)\)").unwrap();
    input.split("mul").skip(skip_count).fold(0, |acc, p| {
        if let Some(numbers) = re.captures(p) {
            let lhs = numbers[1].parse::<u32>().unwrap();
            let rhs = numbers[2].parse::<u32>().unwrap();
            acc + lhs * rhs
        } else {
            acc
        }
    })
}

fn second_part(input: &str) -> u32 {
    let skip_count = if input.starts_with("mul") { 0 } else { 1 };
    let re = Regex::new(r"^\((\d+),(\d+)\)").unwrap();
    let first_mul = input.find("mul").expect("Input contains mul");
    let mut should_count = !input[0..first_mul].contains("don't");
    input.split("mul").skip(skip_count).fold(0, |acc, p| {
        let previous = should_count;

        let last_dont = p.rfind("don't");
        let last_do = p.rfind("do");

        match (last_do, last_dont) {
            (None, None) => {}
            (None, Some(_)) => {
                should_count = false;
            }
            (Some(_), None) => {
                should_count = true;
            }
            (Some(d), Some(dn)) => {
                should_count = d > dn;
            }
        };

        if previous {
            if let Some(numbers) = re.captures(p) {
                let lhs = numbers[1].parse::<u32>().unwrap();
                let rhs = numbers[2].parse::<u32>().unwrap();
                acc + lhs * rhs
            } else {
                acc
            }
        } else {
            acc
        }
    })
}
