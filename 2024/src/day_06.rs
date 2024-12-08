use std::{collections::HashSet, fs::File, io::Read};

pub fn solve() {
    let content = load_from_file();
    let map = parse_to_2d(&content);
    let first_part_result = count_visited_nodes(&map);
    println!("First part result: {}", first_part_result);
}

fn load_from_file() -> String {
    let mut file = File::open("./input/day_06/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn parse_to_2d(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_starting_position(map: &[Vec<char>]) -> (usize, usize) {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch == '^' {
                return (row_idx, col_idx);
            }
        }
    }
    panic!("Starting position must be set");
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct GuardPosition {
    row: i32,
    col: i32,
    dir: Direction,
}

impl GuardPosition {
    fn next_position(&self) -> (i32, i32) {
        match self.dir {
            Direction::Up => (self.row - 1, self.col),
            Direction::Down => (self.row + 1, self.col),
            Direction::Left => (self.row, self.col - 1),
            Direction::Right => (self.row, self.col + 1),
        }
    }
    fn turn_right(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
            Direction::Right => self.dir = Direction::Down,
        }
    }
    fn advance(&mut self, map: &[Vec<char>], row_limit: i32, col_limit: i32) {
        let mut next_position = self.next_position();
        loop {
            if next_position.0 >= row_limit
                || next_position.1 >= col_limit
                || map[next_position.0 as usize][next_position.1 as usize] != '#'
            {
                break;
            }
            self.turn_right();
            next_position = self.next_position();
        }
        (self.row, self.col) = next_position;
    }
    fn pos(&self) -> (i32, i32) {
        (self.row, self.col)
    }
}

fn count_visited_nodes(map: &[Vec<char>]) -> usize {
    let starting_position = find_starting_position(map);
    let mut visited_position = HashSet::<(i32, i32)>::new();
    visited_position.insert((starting_position.0 as i32, starting_position.1 as i32));
    let mut guard_position = GuardPosition {
        row: starting_position.0 as i32,
        col: starting_position.1 as i32,
        dir: Direction::Up,
    };
    let row_limit = map.len() as i32;
    let col_limit = map[0].len() as i32;
    // Loops until guard goes out of map
    loop {
        if guard_position.row >= row_limit || guard_position.col >= col_limit {
            break;
        }
        visited_position.insert(guard_position.pos());
        guard_position.advance(map, row_limit, col_limit);
    }
    visited_position.len()
}
