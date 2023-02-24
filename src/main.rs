mod day01;
mod day02;
mod day03;
use std::fs;

fn main() {
    println!("Hello, world!");

    // Day 1 Puzzle
    let contents = fs::read_to_string("input/201501.txt").unwrap();
    println!("step1: {}", day01::step1::parse_floor(contents.as_str()));
    println!("step2: {}", day01::step2::parse_floor(contents.as_str()));

    let contents = fs::read_to_string("input/201502.txt").unwrap();
    println!(
        "step1: {}",
        day02::step1::wrapping_paper_all(contents.as_str())
    );
    println!(
        "step2: {}",
        day02::step2::total_feet_of_ribbon(contents.as_str())
    );

    let contents = fs::read_to_string("input/201503.txt").unwrap();
    println!(
        "step1: {}",
        day03::step1::houses_with_one_present(contents.as_str())
    );
    println!(
        "step2: {}",
        day03::step2::houses_with_one_present(contents.as_str())
    );
}
