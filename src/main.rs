use dotenv::dotenv;
use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

fn main() {
    dotenv().ok();
    println!("Hello, world!");

    // let day01_input = fs::read_to_string("input/201501.txt").unwrap();
    // println!("step1: {}", day01::step1::parse_floor(day01_input.as_str()));
    // println!("step2: {}", day01::step2::parse_floor(day01_input.as_str()));
    //
    // let day02_input = fs::read_to_string("input/201502.txt").unwrap();
    // println!(
    //     "step1: {}",
    //     day02::step1::wrapping_paper_all(day02_input.as_str())
    // );
    // println!(
    //     "step2: {}",
    //     day02::step2::total_feet_of_ribbon(day02_input.as_str())
    // );
    //
    // let day03_input = fs::read_to_string("input/201503.txt").unwrap();
    // println!(
    //     "step1: {}",
    //     day03::step1::houses_with_one_present(day03_input.as_str())
    // );
    // println!(
    //     "step2: {}",
    //     day03::step2::houses_with_one_present(day03_input.as_str())
    // );
    // let day04_input = dotenv::var("INPUT_DAY_04").unwrap();
    // println!(
    //     "day 04 - step 1: {}",
    //     day04::step1::mine_advent_coin_hash(day04_input.as_str())
    // );
    // println!(
    //     "day 04 - step 2: {}",
    //     day04::step2::mine_advent_coin_hash(day04_input.as_str())
    // );
    // let day05_input = fs::read_to_string("input/201505.txt").unwrap();
    // println!(
    //     "day 05 - step1: {}",
    //     day05::step1::help_santa(&day05_input)
    // );
    // println!(
    //     "day 05 - step2: {}",
    //     day05::step2::help_santa(&day05_input)
    // );
    // let day06_input = fs::read_to_string("input/201506.txt").unwrap();
    // println!(
    //     "day 06 - step1: {}",
    //     day06::step1::count_lights(&day06_input)
    // );
    // println!(
    //     "day 06 - step2: {}",
    //     day06::step2::count_lights(&day06_input)
    // );
    let day07_input = fs::read_to_string("input/201507.txt").unwrap();
    let day07_result = day07::step1::simulate_circuit(&day07_input);
    println!(
        "day 06 - step1: {}",
        day07_result.get("a").expect("could not find key")
    )
}
