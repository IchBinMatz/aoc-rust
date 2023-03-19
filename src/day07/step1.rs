use itertools::Itertools;
use std::collections::HashMap;
use crate::day07::parseInput;

use regex::Regex;

pub fn simulate_circuit(wiring: &str) -> HashMap<&str, u16> {
    let (_, circuit) = parseInput::parse_instructions(wiring).unwrap();
    circuit
}

pub fn signal_of_wire_a() -> u32 {
    0
}

#[test]
fn example() {
    let wiring = "123 -> x
    456 -> y
    x AND y -> d
    x OR y -> e
    x LSHIFT 2 -> f
    y RSHIFT 2 -> g
    NOT x -> h
    NOT y -> i";
    let signals = simulate_circuit(wiring);
    println!("{:?}", signals);
    let signal_d = signals.get("a").unwrap();
    assert_eq!(signal_d, &72);
}
