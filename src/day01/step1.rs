
pub fn parse_floor(directions: &str) -> i32 {
    let mut floor = 0;
    for c in directions.chars() {
        floor = match c {
            '(' => floor + 1,
            ')' => floor - 1,
            _ => floor
        };
        println!("{}", floor)
    }
    floor
}

mod tests {

    #[test]
    fn example1() {
        use super::parse_floor;

        assert_eq!(parse_floor("(())"),0);
        assert_eq!(parse_floor("()()"),0);
        assert_eq!(parse_floor("((("),3);
        assert_eq!(parse_floor("(()(()("),3);
        assert_eq!(parse_floor("())"),-1);
        assert_eq!(parse_floor("))("),-1);
        assert_eq!(parse_floor(")))"),-3);
        assert_eq!(parse_floor(")())())"),-3);
    }
}
