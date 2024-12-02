use std::{fs::File, io::Read};

const MIN_DIFF: u8 = 1;
const MAX_DIFF: u8 = 3;

#[derive(PartialEq, Eq)]
enum ChangeType {
    Increasing,
    Decreasing,
}

impl From<(u8, u8)> for ChangeType {
    fn from(value: (u8, u8)) -> Self {
        if value.0 < value.1 {
            ChangeType::Increasing
        } else {
            ChangeType::Decreasing
        }
    }
}

struct Level {
    data: Vec<u8>,
}

impl Level {
    pub fn is_safe(&self) -> bool {
        let change_type = ChangeType::from((self.data[0], self.data[1]));
        for i in 0..(self.data.len() - 1) {
            let diff = self.data[i].abs_diff(self.data[i + 1]);
            if !(MIN_DIFF..=MAX_DIFF).contains(&diff) {
                return false;
            }
            let current_change_type = ChangeType::from((self.data[i], self.data[i + 1]));
            if current_change_type != change_type {
                return false;
            }
        }
        true
    }

    pub fn is_safe_with_one_removal(&self) -> bool {
        if self.is_safe() {
            return true;
        }
        for skip_i in 0..self.data.len() {
            let data: Vec<_> = self
                .data
                .iter()
                .enumerate()
                .filter_map(|(index, &x)| if index != skip_i { Some(x) } else { None })
                .collect();
            let level = Level { data };
            if level.is_safe() {
                return true;
            }
        }
        false
    }
}

impl From<&str> for Level {
    fn from(value: &str) -> Self {
        let nums = value.split_ascii_whitespace();
        let data = nums.map(|n| n.parse::<u8>().unwrap()).collect();
        Level { data }
    }
}

pub fn solve() {
    let levels = load_from_file();
    let count = count_safe(&levels);
    let count_with_one_removal = count_safe_with_one_removal(&levels);
    println!("Safe: {}", count);
    println!("Safe with one removal: {}", count_with_one_removal);
}

fn load_from_file() -> Vec<Level> {
    let mut file = File::open("./input/day_02/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(Level::from).collect()
}

fn count_safe(levels: &[Level]) -> usize {
    levels
        .iter()
        .fold(0, |acc, l| if l.is_safe() { acc + 1 } else { acc })
}

fn count_safe_with_one_removal(levels: &[Level]) -> usize {
    levels.iter().fold(0, |acc, l| {
        if l.is_safe_with_one_removal() {
            acc + 1
        } else {
            acc
        }
    })
}
