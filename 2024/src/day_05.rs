use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub fn solve() {
    let (rules, instructions) = load_from_file();
    let first_part_result = first_part(&rules, &instructions);
    println!("First part: {}", first_part_result);
    let second_part_result = second_part(&rules, &instructions);
    println!("Second part: {}", second_part_result);
}

// Each entry in hashmap is set of numbers that can be printed only after the key
fn load_from_file() -> (HashMap<u32, HashSet<u32>>, Vec<Vec<u32>>) {
    let mut file = File::open("./input/day_05/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let parts: Vec<_> = contents.split("\n\n").collect();

    assert!(parts.len() == 2);

    let rules_input = parts[0];
    let instructions_input = parts[1];

    let rules = rules_input
        .lines()
        .fold(HashMap::<u32, HashSet<u32>>::new(), |mut acc, line| {
            let parts: Vec<_> = line.split("|").collect();
            assert!(parts.len() == 2);
            let key = parts[0].parse::<u32>().unwrap();
            let value = parts[1].parse::<u32>().unwrap();
            let set = acc.entry(key).or_default();
            set.insert(value);
            acc
        });

    let instructions: Vec<_> = instructions_input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    (rules, instructions)
}

fn first_part(rules: &HashMap<u32, HashSet<u32>>, instructions: &[Vec<u32>]) -> u32 {
    instructions.iter().fold(0, |acc, instruction_line| {
        if is_instruction_line_correct(rules, instruction_line) {
            acc + instruction_line[instruction_line.len() / 2]
        } else {
            acc
        }
    })
}

fn is_instruction_line_correct(
    rules: &HashMap<u32, HashSet<u32>>,
    instruction_line: &[u32],
) -> bool {
    let mut already_seen = HashSet::<u32>::new();
    let empty = HashSet::<u32>::new();
    for &instruction in instruction_line {
        let number_rules = rules.get(&instruction).unwrap_or(&empty);
        if already_seen.intersection(number_rules).next().is_some() {
            return false;
        }
        already_seen.insert(instruction);
    }
    true
}

fn second_part(rules: &HashMap<u32, HashSet<u32>>, instructions: &[Vec<u32>]) -> u32 {
    instructions.iter().fold(0, |acc, instruction_line| {
        if !is_instruction_line_correct(rules, instruction_line) {
            acc + middle_number_after_correction(rules, instruction_line)
        } else {
            acc
        }
    })
}

fn middle_number_after_correction(
    rules: &HashMap<u32, HashSet<u32>>,
    instruction_line: &[u32],
) -> u32 {
    let mut intr: Vec<u32> = instruction_line.to_vec();
    let empty = HashSet::<u32>::new();

    // good old bubble sort
    for i in 0..instruction_line.len() {
        for j in (i + 1)..instruction_line.len() {
            let r = rules.get(&intr[j]).unwrap_or(&empty);
            let should_swap = r.contains(&intr[i]);
            if should_swap {
                intr.swap(i, j);
            }
        }
    }

    intr[intr.len() / 2]
}
