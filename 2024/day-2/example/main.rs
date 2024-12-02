use aoc_2024_day_2::{first_part, second_part};

const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("2024 Day 2: Red-Nosed Reports");
    let sum = first_part(INPUT);
    println!("Number of safe reports: {}", sum);
    let sum = second_part(INPUT);
    println!("Number of safe reports with problem dampener: {}", sum);
}
