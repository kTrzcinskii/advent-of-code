use std::{collections::HashMap, fs::File, io::Read};

const BLINK_COUNT: usize = 25;
const MORE_BLINK_COUNT: usize = 75;

pub fn solve() {
    let stones = load_from_file();
    let first_part = stones.count_after_blinks(BLINK_COUNT);
    println!("First part result: {}", first_part);
    let second_part = stones.count_after_blinks(MORE_BLINK_COUNT);
    println!("Second part result: {}", second_part);
}

fn load_from_file() -> Stones {
    let mut file = File::open("./input/day_11/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let data: Vec<_> = contents
        .split_whitespace()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    Stones { data }
}

struct Stones {
    data: Vec<usize>,
}

impl Stones {
    fn count_after_blinks(&self, count: usize) -> usize {
        let mut stone_count =
            self.data
                .iter()
                .fold(HashMap::<usize, usize>::new(), |mut acc, &stone| {
                    acc.insert(stone, 1);
                    acc
                });

        for _ in 0..count {
            let mut new_counts = HashMap::<usize, usize>::new();
            for k in stone_count.keys() {
                let k_count = *stone_count.get(k).unwrap();
                let digits_count = Self::count_digits(*k);
                if *k == 0 {
                    *new_counts.entry(1).or_default() += k_count;
                } else if digits_count & 1 == 0 {
                    let half_digits = digits_count as u32 / 2;
                    let left_stone = k / 10_usize.pow(half_digits);
                    let right_stone = k % 10_usize.pow(half_digits);
                    *new_counts.entry(left_stone).or_default() += k_count;
                    *new_counts.entry(right_stone).or_default() += k_count;
                } else {
                    *new_counts.entry(k * 2024).or_default() += k_count;
                }
            }
            stone_count = new_counts;
        }

        stone_count.iter().fold(0, |acc, (_, &v)| acc + v)
    }

    fn count_digits(value: usize) -> usize {
        if value == 0 {
            return 1;
        }
        (value as f64).log10().floor() as usize + 1
    }
}
