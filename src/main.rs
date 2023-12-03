use std::fs;

pub mod day01;

fn main() {
    println!("Starting Advent of Code 2023!");

    println!("Day 01");
    let file_path = "resources/day01.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Path not readable");

    day01::exercice(contents);
}
