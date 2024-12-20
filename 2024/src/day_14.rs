use std::{fs::File, io::Read, thread, time::Duration};

#[allow(dead_code)]
const TEST_WIDTH: usize = 11;
#[allow(dead_code)]
const TEST_HEIGHT: usize = 7;

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

const SECONDS: usize = 100;

pub fn solve() {
    let robots = load_from_file();
    let area = Area::new(robots, WIDTH, HEIGHT);
    let robot_counts = area.robots_count_in_quadrants_after_seconds(SECONDS);
    let first_part_result: usize = robot_counts.iter().product();
    println!("First part result: {}", first_part_result);
    let mut secs = 0;
    // For second part, we must manually check if there is a tree or not
    // I choose 0.7 as treshold by guessing, my first pick was 0.9 and it was too big, 0.5 was too small
    // Actual ratio of my input was 0.742, but I guess that for different input 0.7 might not work
    loop {
        let ratio = area.how_many_robots_next_to_each_other_after_seconds(secs);
        println!("Ratio: {}", ratio);
        if ratio > 0.7 {
            println!();
            area.display_area_after_seconds(secs);
            println!("After {}", secs);
            thread::sleep(Duration::from_millis(500));
        }
        secs += 1;
    }
}

fn load_from_file() -> Vec<Robot> {
    let mut file = File::open("./input/day_14/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines().map(Robot::from).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

struct Robot {
    pos: Pos,
    vel_x: i64,
    vel_y: i64,
}

impl Robot {
    fn position_after_seconds(&self, seconds: usize, width: usize, height: usize) -> Pos {
        let seconds = seconds as i64;
        let width = width as i64;
        let height = height as i64;
        let x = self.pos.x + seconds * self.vel_x;
        let final_x = if x >= 0 {
            x % width
        } else if (-x) % width == 0 {
            0
        } else {
            width - ((-x) % width)
        };
        let y = self.pos.y + seconds * self.vel_y;
        let final_y = if y >= 0 {
            y % height
        } else if (-y) % height == 0 {
            0
        } else {
            height - ((-y) % height)
        };
        Pos {
            x: final_x,
            y: final_y,
        }
    }
}

impl From<&str> for Robot {
    fn from(value: &str) -> Self {
        let parts = value.split_once(" ").unwrap();
        let (x_str, y_str) = parts.0.split_once("=").unwrap().1.split_once(",").unwrap();
        let (vel_x_str, vel_y_str) = parts.1.split_once("=").unwrap().1.split_once(",").unwrap();
        Self {
            pos: Pos {
                x: x_str.parse().unwrap(),
                y: y_str.parse().unwrap(),
            },
            vel_x: vel_x_str.parse().unwrap(),
            vel_y: vel_y_str.parse().unwrap(),
        }
    }
}

struct Area {
    robots: Vec<Robot>,
    width: usize,
    height: usize,
}

impl Area {
    fn new(robots: Vec<Robot>, width: usize, height: usize) -> Self {
        Area {
            robots,
            width,
            height,
        }
    }

    fn robots_count_in_quadrants_after_seconds(&self, seconds: usize) -> [usize; 4] {
        let positions: Vec<_> = self
            .robots
            .iter()
            .map(|robot| robot.position_after_seconds(seconds, self.width, self.height))
            .collect();

        positions.iter().fold([0, 0, 0, 0], |mut acc, pos| {
            let quadrant = self.find_quadrant(pos);
            if let Some(quadrant) = quadrant {
                acc[quadrant] += 1;
            }
            acc
        })
    }

    fn find_quadrant(&self, pos: &Pos) -> Option<usize> {
        if pos.x as usize == self.width / 2 || pos.y as usize == self.height / 2 {
            return None;
        }
        let quadrant = match (
            (pos.x as usize) < self.width / 2,
            (pos.y as usize) < self.height / 2,
        ) {
            (true, true) => 0,
            (true, false) => 1,
            (false, true) => 2,
            (false, false) => 3,
        };
        Some(quadrant)
    }

    fn how_many_robots_next_to_each_other_after_seconds(&self, seconds: usize) -> f32 {
        let positions: Vec<_> = self
            .robots
            .iter()
            .map(|robot| robot.position_after_seconds(seconds, self.width, self.height))
            .collect();

        let mut count = 0;
        for pos in &positions {
            if pos.x > 0 {
                let mut p = *pos;
                p.x -= 1;
                if positions.contains(&p) {
                    count += 1;
                    continue;
                }
            }
            if pos.y > 0 {
                let mut p = *pos;
                p.y -= 1;
                if positions.contains(&p) {
                    count += 1;
                    continue;
                }
            }
            if pos.x < self.width as i64 - 1 {
                let mut p = *pos;
                p.x += 1;
                if positions.contains(&p) {
                    count += 1;
                    continue;
                }
            }
            if pos.y < self.height as i64 - 1 {
                let mut p = *pos;
                p.y += 1;
                if positions.contains(&p) {
                    count += 1;
                    continue;
                }
            }

            if pos.x > 0 && pos.y > 0 {
                let mut p = *pos;
                p.x -= 1;
                p.y -= 1;
                if positions.contains(&p) {
                    count += 1;
                    continue;
                }
            }

            if pos.x < self.width as i64 - 1 && pos.y < self.height as i64 - 1 {
                let mut p = *pos;
                p.x += 1;
                p.y += 1;
                if positions.contains(&p) {
                    count += 1;
                    continue;
                }
            }

            if pos.x > 0 && pos.y < self.height as i64 - 1 {
                let mut p = *pos;
                p.x -= 1;
                p.y += 1;
                if positions.contains(&p) {
                    count += 1;
                    continue;
                }
            }

            if pos.x < self.width as i64 - 1 && pos.y > 0 {
                let mut p = *pos;
                p.x += 1;
                p.y -= 1;
                if positions.contains(&p) {
                    count += 1;
                    continue;
                }
            }
        }

        count as f32 / positions.len() as f32
    }

    fn display_area_after_seconds(&self, seconds: usize) {
        let positions: Vec<_> = self
            .robots
            .iter()
            .map(|robot| robot.position_after_seconds(seconds, self.width, self.height))
            .collect();

        for x in 0..self.width {
            for y in 0..self.height {
                if positions.contains(&Pos {
                    x: x as i64,
                    y: y as i64,
                }) {
                    print!("#");
                } else {
                    print!("_")
                }
            }
            println!()
        }
    }
}
