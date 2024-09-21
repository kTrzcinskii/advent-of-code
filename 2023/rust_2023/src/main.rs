use solution::Solution;
use solutions::day_10::Day10Solution;

pub mod solution;
pub mod solutions;

fn main() {
    let file_path = "./input/day10.txt";
    let solver = Day10Solution;
    let solution = solver.get_solution(file_path).unwrap();
    println!("Solution (easy part): {}", solution.0.unwrap());
    println!("Solution (hard part): {}", solution.1.unwrap());
}
