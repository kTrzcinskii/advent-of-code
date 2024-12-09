use std::{collections::HashSet, fs::File, io::Read};

pub fn solve() {
    let content = load_from_file();
    let map = parse_to_2d(&content);
    let first_part_result = count_visited_nodes(&map);
    println!("First part result: {}", first_part_result);
    let second_part_result = count_posibilites_to_create_loop(&map);
    println!("Second part result: {}", second_part_result);
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone)]
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
                || next_position.0 < 0
                || next_position.1 < 0
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

    fn is_in_loop(&self, map: &[Vec<char>]) -> bool {
        let mut p = self.clone();
        let row_limit = map.len() as i32;
        let col_limit = map[0].len() as i32;
        let mut steps_made = HashSet::<(i32, i32, Direction)>::new();
        loop {
            if p.row >= row_limit || p.col >= col_limit || p.row < 0 || p.col < 0 {
                return false;
            }
            let current_step = (p.row, p.col, p.dir);
            if steps_made.contains(&current_step) {
                return true;
            }
            steps_made.insert(current_step);
            p.advance(map, row_limit, col_limit);
        }
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
        if guard_position.row >= row_limit
            || guard_position.col >= col_limit
            || guard_position.row < 0
            || guard_position.col < 0
        {
            break;
        }
        visited_position.insert(guard_position.pos());
        guard_position.advance(map, row_limit, col_limit);
    }
    visited_position.len()
}

// Brute force :)
fn count_posibilites_to_create_loop(map: &[Vec<char>]) -> usize {
    let starting_position = find_starting_position(map);
    let guard_position = GuardPosition {
        row: starting_position.0 as i32,
        col: starting_position.1 as i32,
        dir: Direction::Up,
    };
    let row_limit = map.len() as i32;
    let col_limit = map[0].len() as i32;
    let mut new_blockers = HashSet::<(i32, i32)>::new();

    let mut map_with_new_blocker = map.to_vec();

    for i in 0..row_limit {
        for j in 0..col_limit {
            let should_check = map_with_new_blocker[i as usize][j as usize] == '.';
            if should_check {
                map_with_new_blocker[i as usize][j as usize] = '#';
                if guard_position.is_in_loop(&map_with_new_blocker) {
                    new_blockers.insert((i, j));
                }
                map_with_new_blocker[i as usize][j as usize] = ',';
            }
        }
    }

    new_blockers.len()
}
