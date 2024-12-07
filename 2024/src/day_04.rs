use std::{fs::File, io::Read};

pub fn solve() {
    let contents = load_from_file();
    let letters = parse_to_2d(&contents);
    let first_part_result = first_part(&letters);
    println!("First part result: {}", first_part_result);
    // TODO: second part
}

fn load_from_file() -> String {
    let mut file = File::open("./input/day_04/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn parse_to_2d(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn first_part(letters: &[Vec<char>]) -> usize {
    let rows = letters.len();
    let cols = letters[0].len();

    let mut x_positions = vec![];

    #[allow(clippy::needless_range_loop)]
    for i in 0..rows {
        for j in 0..cols {
            if letters[i][j] == 'X' {
                x_positions.push((i, j));
            }
        }
    }

    x_positions.iter().fold(0, |acc, &pos| {
        let mut full = 0;
        if check_left(letters, pos) {
            full += 1;
        }
        if check_right(letters, pos, cols) {
            full += 1;
        }
        if check_up(letters, pos) {
            full += 1;
        }
        if check_down(letters, pos, rows) {
            full += 1;
        }
        if check_left_up(letters, pos) {
            full += 1;
        }
        if check_left_down(letters, pos, rows) {
            full += 1;
        }
        if check_right_up(letters, pos, cols) {
            full += 1;
        }
        if check_right_down(letters, pos, rows, cols) {
            full += 1;
        }

        acc + full
    })
}

#[inline(always)]
fn check_left(letters: &[Vec<char>], pos: (usize, usize)) -> bool {
    let (row, col) = pos;
    if col < 3 {
        false
    } else {
        letters[row][col - 1] == 'M' && letters[row][col - 2] == 'A' && letters[row][col - 3] == 'S'
    }
}

#[inline(always)]
fn check_right(letters: &[Vec<char>], pos: (usize, usize), max_col: usize) -> bool {
    let (row, col) = pos;
    if col + 3 >= max_col {
        false
    } else {
        letters[row][col + 1] == 'M' && letters[row][col + 2] == 'A' && letters[row][col + 3] == 'S'
    }
}

#[inline(always)]
fn check_up(letters: &[Vec<char>], pos: (usize, usize)) -> bool {
    let (row, col) = pos;
    if row < 3 {
        false
    } else {
        letters[row - 1][col] == 'M' && letters[row - 2][col] == 'A' && letters[row - 3][col] == 'S'
    }
}

#[inline(always)]
fn check_down(letters: &[Vec<char>], pos: (usize, usize), max_row: usize) -> bool {
    let (row, col) = pos;
    if row + 3 >= max_row {
        false
    } else {
        letters[row + 1][col] == 'M' && letters[row + 2][col] == 'A' && letters[row + 3][col] == 'S'
    }
}

#[inline(always)]
fn check_left_up(letters: &[Vec<char>], pos: (usize, usize)) -> bool {
    let (row, col) = pos;
    if row < 3 || col < 3 {
        false
    } else {
        letters[row - 1][col - 1] == 'M'
            && letters[row - 2][col - 2] == 'A'
            && letters[row - 3][col - 3] == 'S'
    }
}

#[inline(always)]
fn check_left_down(letters: &[Vec<char>], pos: (usize, usize), max_row: usize) -> bool {
    let (row, col) = pos;
    if row + 3 >= max_row || col < 3 {
        false
    } else {
        letters[row + 1][col - 1] == 'M'
            && letters[row + 2][col - 2] == 'A'
            && letters[row + 3][col - 3] == 'S'
    }
}

#[inline(always)]
fn check_right_up(letters: &[Vec<char>], pos: (usize, usize), max_col: usize) -> bool {
    let (row, col) = pos;
    if row < 3 || col + 3 >= max_col {
        false
    } else {
        letters[row - 1][col + 1] == 'M'
            && letters[row - 2][col + 2] == 'A'
            && letters[row - 3][col + 3] == 'S'
    }
}

#[inline(always)]
fn check_right_down(
    letters: &[Vec<char>],
    pos: (usize, usize),
    max_row: usize,
    max_col: usize,
) -> bool {
    let (row, col) = pos;
    if row + 3 >= max_row || col + 3 >= max_col {
        false
    } else {
        letters[row + 1][col + 1] == 'M'
            && letters[row + 2][col + 2] == 'A'
            && letters[row + 3][col + 3] == 'S'
    }
}
