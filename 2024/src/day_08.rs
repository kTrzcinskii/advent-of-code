use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub fn solve() {
    let input = load_from_file();
    let map = Map::from(&input);
    let first_part_result = map.find_unique_antinode_locations().len();
    println!("First part result: {}", first_part_result);
    let second_part_result = map
        .find_unique_antinode_locations_with_resonant_harmonics()
        .len();
    println!("Second part result: {}", second_part_result);
}

fn load_from_file() -> Vec<Vec<char>> {
    let mut file = File::open("./input/day_08/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

struct Map {
    data: HashMap<char, Vec<Position>>,
    row_limit: i32,
    col_limit: i32,
}

impl Map {
    fn find_unique_antinode_locations(&self) -> HashSet<Position> {
        let mut locations = HashSet::<Position>::new();

        for positions in self.data.values() {
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let lhs = positions[i];
                    let rhs = positions[j];

                    let diff = Position {
                        row: lhs.row - rhs.row,
                        col: lhs.col - rhs.col,
                    };

                    let pos1 = Position {
                        col: lhs.col + diff.col,
                        row: lhs.row + diff.row,
                    };

                    if self.is_in_range(&pos1) {
                        locations.insert(pos1);
                    }

                    let pos2 = Position {
                        col: rhs.col - diff.col,
                        row: rhs.row - diff.row,
                    };

                    if self.is_in_range(&pos2) {
                        locations.insert(pos2);
                    }
                }
            }
        }

        locations
    }

    fn find_unique_antinode_locations_with_resonant_harmonics(&self) -> HashSet<Position> {
        let mut locations = HashSet::<Position>::new();

        for positions in self.data.values() {
            for i in 0..positions.len() {
                for j in (i + 1)..positions.len() {
                    let lhs = positions[i];
                    let rhs = positions[j];

                    locations.insert(lhs);
                    locations.insert(rhs);

                    let diff = Position {
                        row: lhs.row - rhs.row,
                        col: lhs.col - rhs.col,
                    };

                    let mut pos1 = Position {
                        col: lhs.col + diff.col,
                        row: lhs.row + diff.row,
                    };

                    while self.is_in_range(&pos1) {
                        locations.insert(pos1);

                        pos1.col += diff.col;
                        pos1.row += diff.row;
                    }

                    let mut pos2 = Position {
                        col: rhs.col - diff.col,
                        row: rhs.row - diff.row,
                    };

                    while self.is_in_range(&pos2) {
                        locations.insert(pos2);

                        pos2.col -= diff.col;
                        pos2.row -= diff.row;
                    }
                }
            }
        }

        locations
    }

    fn is_in_range(&self, pos: &Position) -> bool {
        pos.col >= 0 && pos.col < self.col_limit && pos.row >= 0 && pos.row < self.row_limit
    }
}

impl From<&Vec<Vec<char>>> for Map {
    fn from(value: &Vec<Vec<char>>) -> Self {
        let mut map = HashMap::<char, Vec<Position>>::new();
        for i in 0..value.len() {
            for j in 0..value[0].len() {
                let c = value[i][j];
                if c.is_alphanumeric() {
                    let pos = Position {
                        row: i as i32,
                        col: j as i32,
                    };
                    map.entry(c).or_default().push(pos);
                }
            }
        }
        Map {
            data: map,
            row_limit: value.len() as i32,
            col_limit: value[0].len() as i32,
        }
    }
}
