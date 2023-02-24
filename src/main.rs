mod day01;
mod day02;
use std::fs;

fn main() {
    println!("Hello, world!");

    // Day 1 Puzzle
    let contents = fs::read_to_string("input/201501.txt").unwrap();
    let result0101 = day01::step1::parse_floor(contents.as_str());
    println!("step1: {}", result0101);
    let result0102 = day01::step2::parse_floor(contents.as_str());
    println!("step2: {}", result0102);

    let contents = fs::read_to_string("input/201502.txt").unwrap();
    let result0201 = day02::step1::wrapping_paper_all(contents.as_str());
    println!("step1: {}", result0201);
    println!("step2: {}", day02::step2::total_feet_of_ribbon(contents.as_str()));
}
