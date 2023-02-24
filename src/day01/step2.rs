pub fn parse_floor(directions: &str) -> u32 {
    let mut floor = 0;
    for (step, c) in directions.chars().enumerate() {
        floor = match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor,
        };
        if floor == -1 {
            return 1 + step as u32;
        }
    }
    0
}

#[test]
fn example() {
    assert_eq!(parse_floor(")"), 1);
    assert_eq!(parse_floor("()())"), 5);
}