use anyhow::{Error, Result};

use crate::solution::Solution;

pub struct Day10Solution;

#[derive(Default, PartialEq, Clone, Copy)]
struct Move {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
}

impl Move {
    fn can_transist(from: &Move, to: &Move, direction: &Direction) -> bool {
        // --
        if from.right && to.left && *direction == Direction::Right {
            return true;
        }
        if from.left && to.right && *direction == Direction::Left {
            return true;
        }

        // |
        // |
        if from.top && to.bottom && *direction == Direction::Top {
            return true;
        }
        if from.bottom && to.top && *direction == Direction::Bottom {
            return true;
        }

        false
    }
}

struct MoveBuilder {
    move_struct: Move,
}

impl MoveBuilder {
    fn new() -> Self {
        MoveBuilder {
            move_struct: Move::default(),
        }
    }

    fn top(mut self, top: bool) -> Self {
        self.move_struct.top = top;
        self
    }

    fn bottom(mut self, bottom: bool) -> Self {
        self.move_struct.bottom = bottom;
        self
    }

    fn left(mut self, left: bool) -> Self {
        self.move_struct.left = left;
        self
    }

    fn right(mut self, right: bool) -> Self {
        self.move_struct.right = right;
        self
    }

    fn build(self) -> Move {
        self.move_struct
    }
}

impl TryFrom<u8> for Move {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'|' => Ok(MoveBuilder::new().top(true).bottom(true).build()),
            b'-' => Ok(MoveBuilder::new().left(true).right(true).build()),
            b'L' => Ok(MoveBuilder::new().top(true).right(true).build()),
            b'J' => Ok(MoveBuilder::new().top(true).left(true).build()),
            b'F' => Ok(MoveBuilder::new().bottom(true).right(true).build()),
            b'7' => Ok(MoveBuilder::new().bottom(true).left(true).build()),
            _ => Err(Error::msg("Not direction")),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Field {
    Move(Move),
    Start,
    Wall,
}

impl Field {
    fn can_transist(from: &Field, to: &Field, direction: &Direction) -> bool {
        match (from, to) {
            (Field::Move(from), Field::Move(to)) => Move::can_transist(from, to, direction),
            (_, _) => false,
        }
    }
}

impl From<u8> for Field {
    fn from(value: u8) -> Self {
        match Move::try_from(value) {
            Ok(move_value) => Field::Move(move_value),
            Err(_) => match value {
                b'S' => Field::Start,
                _ => Field::Wall,
            },
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    row: usize,
    column: usize,
}

impl Point {
    fn apply_direction(&self, direction: &Direction) -> Point {
        match direction {
            Direction::Top => Point {
                row: self.row - 1,
                column: self.column,
            },
            Direction::Bottom => Point {
                row: self.row + 1,
                column: self.column,
            },
            Direction::Left => Point {
                row: self.row,
                column: self.column - 1,
            },
            Direction::Right => Point {
                row: self.row,
                column: self.column + 1,
            },
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

impl Direction {
    fn get_opposite(&self) -> Direction {
        match self {
            Direction::Top => Direction::Bottom,
            Direction::Bottom => Direction::Top,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Board {
    board: Vec<Vec<Field>>,
    starting_point: Point,
}

impl Board {
    fn get_starting_point_move(&self) -> Move {
        let max_row = self.board.len();
        let max_col = self.board[0].len() - 1;
        // We know starting point is connected to exactly two other pipes
        match (self.starting_point.row, self.starting_point.column) {
            (0, 0) => MoveBuilder::new().bottom(true).right(true).build(),
            (row, 0) if row == max_row => MoveBuilder::new().top(true).right(true).build(),
            (0, col) if col == max_col => MoveBuilder::new().bottom(true).left(true).build(),
            (row, col) if row == max_row && col == max_col => {
                MoveBuilder::new().top(true).left(true).build()
            }
            _ => {
                let top = self.starting_point.row > 0
                    && match self.board[self.starting_point.row - 1][self.starting_point.column] {
                        Field::Move(m) => m.bottom,
                        _ => false,
                    };
                let bottom = self.starting_point.row < max_row
                    && match self.board[self.starting_point.row + 1][self.starting_point.column] {
                        Field::Move(m) => m.top,
                        _ => false,
                    };
                let left = self.starting_point.column > 0
                    && match self.board[self.starting_point.row][self.starting_point.column - 1] {
                        Field::Move(m) => m.right,
                        _ => false,
                    };
                let right = self.starting_point.column < max_col
                    && match self.board[self.starting_point.row][self.starting_point.column + 1] {
                        Field::Move(m) => m.left,
                        _ => false,
                    };

                MoveBuilder::new()
                    .top(top)
                    .bottom(bottom)
                    .left(left)
                    .right(right)
                    .build()
            }
        }
    }

    /// Returns how many fields we passed during travel of whole loop, counting starting_point only once
    fn travel(&self) -> usize {
        let mut current_position = self.starting_point;
        let starting_field = self.get_field(&current_position);

        let starting_direction = match starting_field {
            Field::Start => panic!("Starting field must be move"),
            Field::Wall => panic!("Starting field must be move"),
            Field::Move(m) => {
                if m.top {
                    Direction::Top
                } else if m.bottom {
                    Direction::Bottom
                } else if m.left {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
        };

        current_position = current_position.apply_direction(&starting_direction);
        let mut last_direction_moved = starting_direction;
        let mut counter: usize = 1;

        while current_position != self.starting_point {
            last_direction_moved =
                self.find_next_move(&current_position, last_direction_moved.get_opposite());
            current_position = current_position.apply_direction(&last_direction_moved);
            counter += 1;
        }

        counter
    }

    fn find_next_move(&self, current_position: &Point, skip_direction: Direction) -> Direction {
        // We are sure that one will be correct, se we don;t have to check the last one
        let max_row = self.board.len() - 1;
        if current_position.row > 0
            && skip_direction != Direction::Top
            && Field::can_transist(
                self.get_field(current_position),
                self.get_field(&current_position.apply_direction(&Direction::Top)),
                &Direction::Top,
            )
        {
            Direction::Top
        } else if current_position.row < max_row
            && skip_direction != Direction::Bottom
            && Field::can_transist(
                self.get_field(current_position),
                self.get_field(&current_position.apply_direction(&Direction::Bottom)),
                &Direction::Bottom,
            )
        {
            Direction::Bottom
        } else if current_position.column > 0
            && skip_direction != Direction::Left
            && Field::can_transist(
                self.get_field(current_position),
                self.get_field(&current_position.apply_direction(&Direction::Left)),
                &Direction::Left,
            )
        {
            Direction::Left
        } else {
            Direction::Right
        }
    }

    fn get_field(&self, point: &Point) -> &Field {
        &self.board[point.row][point.column]
    }
}

impl Day10Solution {
    fn get_board(&self, data: String) -> Result<Board> {
        let mut board = vec![];

        let lines = data.lines();
        for line in lines {
            let fields_in_line: Vec<Field> = line
                .as_bytes()
                .iter()
                .map(|byte| Field::from(*byte))
                .collect();
            board.push(fields_in_line);
        }

        let mut starting_point = None;

        for (row_id, row) in board.iter().enumerate() {
            for (col_id, item) in row.iter().enumerate() {
                if *item == Field::Start {
                    starting_point = Some(Point {
                        row: row_id,
                        column: col_id,
                    });
                }
            }
        }

        match starting_point {
            Some(starting_point) => Ok(Board {
                board,
                starting_point,
            }),
            None => Err(Error::msg("Missing starting point")),
        }
    }
}

impl Solution for Day10Solution {
    fn calculate_solution_easy(&self, data: String) -> anyhow::Result<impl std::fmt::Display> {
        let mut board = self.get_board(data)?;

        let starting_point_field = Field::Move(board.get_starting_point_move());
        board.board[board.starting_point.row][board.starting_point.column] = starting_point_field;

        let count = board.travel();

        Ok((count + 1) / 2)
    }

    fn calculate_solution_hard(&self, data: String) -> anyhow::Result<impl std::fmt::Display> {
        Ok(false)
    }
}
