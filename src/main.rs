use std::fs;

pub mod day01;
pub mod day02;

fn main() {
    println!("");
    println!("❄️  Starting Advent of Code 2023 ❄️");
    println!("");

    println!("========= Day 01 =========");
    let file_path = "inputs/day01_1.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Path not readable");
    println!("Ex.1 - The sum of all calibration values is {}", day01::exercice1(contents));
    let file_path = "inputs/day01_2.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Path not readable");
    println!("Ex.2 - The sum of all calibration values is {}", day01::exercice2(contents));
    println!("");


    println!("========= Day 02 =========");
    let file_path = "inputs/day02_1.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Path not readable");
    println!("Ex.1 - ???? {}", day02::exercice1(contents));
    // let file_path = "inputs/day02_2.txt";
    // let contents = fs::read_to_string(file_path)
    //     .expect("Path not readable");
    // println!("Ex.2 - ???? {}", day02::exercice2(contents));
    println!("");
}
