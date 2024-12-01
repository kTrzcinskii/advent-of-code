use std::{collections::HashMap, fs::File, io::Read};

pub fn solve_firs_part() {
    let (mut left, mut right) = load_from_file();
    left.sort();
    right.sort();
    let distance = calculate_distance(&left, &right);
    println!("Distance: {}", distance);
}

pub fn solve_second_part() {
    let (left, right) = load_from_file();
    let counts = count_numbers(&right);
    let similarity = calculate_similarity(&left, &counts);
    println!("Similarity: {}", similarity);
}

fn load_from_file() -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    let mut file = File::open("./input/day_01/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    for line in contents.lines() {
        let numbers: Vec<_> = line.split_ascii_whitespace().collect();
        let left_item = numbers[0].parse::<usize>().unwrap();
        left.push(left_item);
        let right_item = numbers[1].parse::<usize>().unwrap();
        right.push(right_item);
    }
    (left, right)
}

// At this point both left and right are sorted
fn calculate_distance(left: &[usize], right: &[usize]) -> usize {
    left.iter()
        .zip(right.iter())
        .fold(0, |acc, (&l, &r)| acc + l.abs_diff(r))
}

fn count_numbers(numbers: &[usize]) -> HashMap<usize, usize> {
    let mut counts = HashMap::<usize, usize>::new();
    for &number in numbers {
        *counts.entry(number).or_insert(0) += 1;
    }
    counts
}

fn calculate_similarity(numbers: &[usize], counts: &HashMap<usize, usize>) -> usize {
    numbers
        .iter()
        .fold(0, |acc, &x| acc + x * counts.get(&x).unwrap_or(&0))
}
