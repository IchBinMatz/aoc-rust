mod day01;
mod day02;
use std::fs;

fn main() {
    println!("Hello, world!");

    // Day 1 Puzzle
    let contents = fs::read_to_string("input/20150101.txt").unwrap();
    let result0101 = day01::step1::parse_floor(contents.as_str());
    println!("step1: {}", result0101);
    let result0102 = day01::step2::parse_floor(contents.as_str());
    println!("step2: {}", result0102)
}
