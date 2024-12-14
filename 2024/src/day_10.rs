use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

const MAX_VALUE: u8 = 9;

pub fn solve() {
    let content = load_from_file();
    let trail_directed_graph = TrailDirectedGraph::from(content);
    let first_part = trail_directed_graph.trailheads_score(MAX_VALUE);
    println!("First part result: {}", first_part);
    let second_part = trail_directed_graph.trailheads_rating(MAX_VALUE);
    println!("Second part result: {}", second_part);
}

fn load_from_file() -> Vec<Vec<u8>> {
    let mut file = File::open("./input/day_10/data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| char.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

struct TrailDirectedGraph {
    _rows: usize,
    _cols: usize,
    nodes: Vec<Vec<u8>>,
    // Key - node, Value - list of its neighbours
    edges: HashMap<(usize, usize), Vec<(usize, usize)>>,
    // positions of zeros
    zeros: Vec<(usize, usize)>,
}

impl TrailDirectedGraph {
    fn trailheads_score(&self, value: u8) -> usize {
        self.zeros.iter().fold(0, |acc, &from| {
            acc + self.how_many_distinct_targets(from, value)
        })
    }

    fn how_many_distinct_targets(&self, from: (usize, usize), value: u8) -> usize {
        let mut set = HashSet::<(usize, usize)>::new();

        let mut stack: Vec<(usize, usize)> = vec![from];
        let mut visited = HashSet::<(usize, usize)>::new();

        while let Some(node) = stack.pop() {
            let neighbours = self.edges.get(&node);
            if let Some(neighbours) = neighbours {
                for neigh in neighbours {
                    if visited.contains(neigh) {
                        continue;
                    }
                    if self.nodes[neigh.0][neigh.1] == value {
                        set.insert(*neigh);
                    } else {
                        stack.push(*neigh);
                    }
                    visited.insert(*neigh);
                }
            }
        }
        set.len()
    }

    fn trailheads_rating(&self, value: u8) -> usize {
        self.zeros.iter().fold(0, |acc, &from| {
            acc + self.how_many_distinct_paths(from, value)
        })
    }

    fn how_many_distinct_paths(&self, from: (usize, usize), value: u8) -> usize {
        let mut counter = 0;
        let mut stack: Vec<(usize, usize)> = vec![from];

        while let Some(node) = stack.pop() {
            let neighbours = self.edges.get(&node);
            if let Some(neighbours) = neighbours {
                for neigh in neighbours {
                    if self.nodes[neigh.0][neigh.1] == value {
                        counter += 1;
                    } else {
                        stack.push(*neigh);
                    }
                }
            }
        }
        counter
    }
}

impl From<Vec<Vec<u8>>> for TrailDirectedGraph {
    fn from(value: Vec<Vec<u8>>) -> Self {
        let rows = value.len();
        let cols = value[0].len();
        let mut edges: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
        let mut zeros = vec![];

        for i in 0..rows {
            for j in 0..cols {
                let val = value[i][j];
                if val == 0 {
                    zeros.push((i, j));
                }
                if i > 0 && value[i - 1][j] == val + 1 {
                    edges.entry((i, j)).or_default().push((i - 1, j));
                }
                if j > 0 && value[i][j - 1] == val + 1 {
                    edges.entry((i, j)).or_default().push((i, j - 1));
                }
                if i + 1 < rows && value[i + 1][j] == val + 1 {
                    edges.entry((i, j)).or_default().push((i + 1, j));
                }
                if j + 1 < cols && value[i][j + 1] == val + 1 {
                    edges.entry((i, j)).or_default().push((i, j + 1));
                }
            }
        }

        TrailDirectedGraph {
            _rows: rows,
            _cols: cols,
            nodes: value,
            edges,
            zeros,
        }
    }
}
