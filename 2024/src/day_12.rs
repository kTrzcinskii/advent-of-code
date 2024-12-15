use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

pub fn solve() {
    let garden_map = load_from_file();
    let first_part = garden_map.total_price();
    println!("First part result: {}", first_part);
    let second_part = garden_map.total_price_with_discount();
    println!("Second part result: {}", second_part);
}

fn load_from_file() -> GardenMap {
    let mut file = File::open("./input/day_12/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.as_str().into()
}

struct GardenMap {
    edges: HashMap<(usize, usize), HashSet<(usize, usize)>>,
    cols: usize,
    rows: usize,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Top,  // when second coord increases
    Down, // when second coord decreases
}

impl GardenMap {
    fn total_price(&self) -> usize {
        let mut current_region: Vec<(usize, usize)> = vec![];
        let mut other_region: Vec<(usize, usize)> = vec![(0, 0)];

        let mut visited = HashSet::<(usize, usize)>::new();

        let mut total = 0;

        while let Some(region_start) = other_region.pop() {
            current_region.push(region_start);

            let mut region_count = 0;
            let mut region_pelimeter = 0;

            while let Some(el) = current_region.pop() {
                if visited.contains(&el) {
                    continue;
                }
                visited.insert(el);
                region_count += 1;
                if el.0 > 0 {
                    let next_pos = (el.0 - 1, el.1);
                    if self.is_same_region(el, next_pos) {
                        current_region.push(next_pos);
                    } else {
                        other_region.push(next_pos);
                        region_pelimeter += 1;
                    }
                } else {
                    region_pelimeter += 1;
                }

                if el.0 + 1 < self.rows {
                    let next_pos = (el.0 + 1, el.1);
                    if self.is_same_region(el, next_pos) {
                        current_region.push(next_pos);
                    } else {
                        other_region.push(next_pos);
                        region_pelimeter += 1;
                    }
                } else {
                    region_pelimeter += 1;
                }

                if el.1 > 0 {
                    let next_pos = (el.0, el.1 - 1);
                    if self.is_same_region(el, next_pos) {
                        current_region.push(next_pos);
                    } else {
                        other_region.push(next_pos);
                        region_pelimeter += 1;
                    }
                } else {
                    region_pelimeter += 1;
                }

                if el.1 + 1 < self.cols {
                    let next_pos = (el.0, el.1 + 1);
                    if self.is_same_region(el, next_pos) {
                        current_region.push(next_pos);
                    } else {
                        other_region.push(next_pos);
                        region_pelimeter += 1;
                    }
                } else {
                    region_pelimeter += 1;
                }
            }

            total += region_count * region_pelimeter;
        }

        total
    }

    fn total_price_with_discount(&self) -> usize {
        let mut current_region: Vec<(usize, usize)> = vec![];
        let mut other_region: Vec<(usize, usize)> = vec![(0, 0)];

        let mut visited = HashSet::<(usize, usize)>::new();

        let mut total = 0;

        while let Some(region_start) = other_region.pop() {
            current_region.push(region_start);

            let mut region_count = 0;
            let mut region_pelimeters = HashSet::<(i32, i32, Direction)>::new();

            while let Some(el) = current_region.pop() {
                if visited.contains(&el) {
                    continue;
                }
                visited.insert(el);
                region_count += 1;
                if el.0 > 0 {
                    let next_pos = (el.0 - 1, el.1);
                    if self.is_same_region(el, next_pos) {
                        current_region.push(next_pos);
                    } else {
                        other_region.push(next_pos);
                        region_pelimeters.insert((
                            next_pos.0 as i32,
                            next_pos.1 as i32,
                            Direction::Down,
                        ));
                    }
                } else {
                    region_pelimeters.insert((el.0 as i32 - 1, el.1 as i32, Direction::Down));
                }

                if el.0 + 1 < self.rows {
                    let next_pos = (el.0 + 1, el.1);
                    if self.is_same_region(el, next_pos) {
                        current_region.push(next_pos);
                    } else {
                        other_region.push(next_pos);
                        region_pelimeters.insert((
                            next_pos.0 as i32,
                            next_pos.1 as i32,
                            Direction::Top,
                        ));
                    }
                } else {
                    region_pelimeters.insert((el.0 as i32 + 1, el.1 as i32, Direction::Top));
                }

                if el.1 > 0 {
                    let next_pos = (el.0, el.1 - 1);
                    if self.is_same_region(el, next_pos) {
                        current_region.push(next_pos);
                    } else {
                        other_region.push(next_pos);
                        region_pelimeters.insert((
                            next_pos.0 as i32,
                            next_pos.1 as i32,
                            Direction::Left,
                        ));
                    }
                } else {
                    region_pelimeters.insert((el.0 as i32, el.1 as i32 - 1, Direction::Left));
                }

                if el.1 + 1 < self.cols {
                    let next_pos = (el.0, el.1 + 1);
                    if self.is_same_region(el, next_pos) {
                        current_region.push(next_pos);
                    } else {
                        other_region.push(next_pos);
                        region_pelimeters.insert((
                            next_pos.0 as i32,
                            next_pos.1 as i32,
                            Direction::Right,
                        ));
                    }
                } else {
                    region_pelimeters.insert((el.0 as i32, el.1 as i32 + 1, Direction::Right));
                }
            }

            let region_sides = Self::count_region_sides(&region_pelimeters);

            total += region_count * region_sides;
        }

        total
    }

    fn is_same_region(&self, pos: (usize, usize), next_pos: (usize, usize)) -> bool {
        self.edges
            .get(&pos)
            .unwrap_or(&HashSet::new())
            .contains(&next_pos)
    }

    fn count_region_sides(region_pelimeters: &HashSet<(i32, i32, Direction)>) -> usize {
        if region_pelimeters.is_empty() {
            return 0;
        }
        let mut vec: Vec<_> = region_pelimeters.iter().collect();

        let mut count = 0;

        while let Some(el) = vec.pop() {
            if vec.contains(&&(el.0, el.1 + 1, el.2)) || vec.contains(&&(el.0, el.1 - 1, el.2)) {
                let mut c = el;
                loop {
                    let result = vec
                        .iter()
                        .enumerate()
                        .find(|(_, p)| p.0 == c.0 && p.1 == c.1 + 1 && p.2 == el.2);
                    match result {
                        Some((idx, p)) => {
                            c = p;
                            vec.remove(idx);
                        }
                        None => break,
                    }
                }
                let mut c = el;
                loop {
                    let result = vec
                        .iter()
                        .enumerate()
                        .find(|(_, p)| p.0 == c.0 && p.1 == c.1 - 1 && p.2 == el.2);
                    match result {
                        Some((idx, p)) => {
                            c = p;
                            vec.remove(idx);
                        }
                        None => break,
                    }
                }
            }

            if vec.contains(&&(el.0 + 1, el.1, el.2)) || vec.contains(&&(el.0 - 1, el.1, el.2)) {
                let mut c = el;
                loop {
                    let result = vec
                        .iter()
                        .enumerate()
                        .find(|(_, p)| p.0 == c.0 + 1 && p.1 == c.1 && p.2 == el.2);
                    match result {
                        Some((idx, p)) => {
                            c = p;
                            vec.remove(idx);
                        }
                        None => break,
                    }
                }
                let mut c = el;
                loop {
                    let result = vec
                        .iter()
                        .enumerate()
                        .find(|(_, p)| p.0 == c.0 - 1 && p.1 == c.1 && p.2 == el.2);
                    match result {
                        Some((idx, p)) => {
                            c = p;
                            vec.remove(idx);
                        }
                        None => break,
                    }
                }
            }

            count += 1;
        }

        count
    }
}

impl From<&str> for GardenMap {
    fn from(value: &str) -> Self {
        let nodes: Vec<_> = value
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect();
        let rows = nodes.len();
        let cols = nodes[0].len();

        let mut edges = HashMap::<(usize, usize), HashSet<(usize, usize)>>::new();

        for i in 0..rows {
            for j in 0..cols {
                if i > 0 {
                    populate_edges_and_pelimeters(&mut edges, (i, j), (i - 1, j), &nodes);
                }
                if j > 0 {
                    populate_edges_and_pelimeters(&mut edges, (i, j), (i, j - 1), &nodes);
                }
                if i + 1 < rows {
                    populate_edges_and_pelimeters(&mut edges, (i, j), (i + 1, j), &nodes);
                }
                if j + 1 < cols {
                    populate_edges_and_pelimeters(&mut edges, (i, j), (i, j + 1), &nodes);
                }
            }
        }

        GardenMap { edges, cols, rows }
    }
}

fn populate_edges_and_pelimeters(
    edges: &mut HashMap<(usize, usize), HashSet<(usize, usize)>>,
    pos: (usize, usize),
    next_pos: (usize, usize),
    nodes: &[Vec<char>],
) {
    if nodes[next_pos.0][next_pos.1] == nodes[pos.0][pos.1] {
        edges.entry(pos).or_default().insert(next_pos);
        edges.entry(next_pos).or_default().insert(pos);
    }
}
