use std::collections::HashMap;
use std::error::Error;

use winnow::branch::alt;
use winnow::bytes::tag;
use winnow::bytes::take_while1;
use winnow::character::alpha1;
use winnow::character::digit1;
use winnow::dispatch;
use winnow::sequence::preceded;
use winnow::sequence::separated_pair;
use winnow::stream::AsChar;
use winnow::IResult;
use winnow::Parser;

enum OPERANT {
    AND,
    OR,
}
enum SHIFT {
    LEFT,
    RIGHT,
}
enum LHS<'a> {
    VALUE(u16),
    NOT(&'a str),
    OP(&'a str, OPERANT, &'a str),
    SHIFT(&'a str, SHIFT, u16),
}

fn parse_assignment(input: &str) -> IResult<&str, (LHS, &str)> {
    separated_pair(parse_lhs, tag(" -> "), take_while1(AsChar::is_alpha)).parse_next(input)
}

fn parse_lhs(input: &str) -> IResult<&str, LHS> {
    alt((parse_number, parse_not, parse_operation, parse_shift)).parse_next(input)
}

fn parse_number(input: &str) -> IResult<&str, LHS> {
    let (rem, value): (&str, u16) = match digit1.parse_to().parse_next(input) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };
    Ok((rem, LHS::VALUE(value)))
}

fn parse_not(input: &str) -> IResult<&str, LHS> {
    let (rem, signal) = match preceded(tag("NOT "), alpha1).parse_next(input) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    Ok((rem, LHS::NOT(signal)))
}
fn parse_operator(input: &str) -> IResult<&str, OPERANT> {
    let (rem, op) = match alt((" AND ", " OR ")).parse_next(input) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let operant = match op {
        " AND " => OPERANT::AND,
        " OR " => OPERANT::OR,
        _ => panic!(),
    };
    Ok((rem, operant))
}
fn parse_operation(input: &str) -> IResult<&str, LHS> {
    let (rem, (s1, op, s2)) = match (alpha1, parse_operator, alpha1).parse_next(input) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    Ok((rem, LHS::OP(s1, op, s2)))
}

fn parse_shift(input: &str) -> IResult<&str, LHS> {
    let (rem, (s1, op, val)) = match (alpha1, parse_shift_dir, digit1.parse_to()).parse_next(input)
    {
        Ok(s) => s,
        Err(e) => return Err(e),
    };
    Ok((rem, LHS::SHIFT(s1, op, val)))
}

fn parse_shift_dir(input: &str) -> IResult<&str, SHIFT> {
    let (rem, op) = match alt((" LSHIFT ", " RSHIFT ")).parse_next(input) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let dir = match op {
        " LSHIFT " => SHIFT::LEFT,
        " RSHIFT " => SHIFT::RIGHT,
        _ => panic!(),
    };
    Ok((rem, dir))
}

pub fn parse_instructions(instructions: &str) -> IResult<&str, HashMap<&str, u16>> {
    let mut circuit: HashMap<&str, u16> = HashMap::new();
    for instruction in instructions.lines() {
        let (_, (assignment, signal)) = match parse_assignment(instruction) {
            Ok(s) => s,
            Err(e) => return Err(e),
        };
        let newvalue = match assignment {
            LHS::VALUE(v) => v,
            LHS::NOT(s) => !*circuit
                .get(s)
                .expect(&["could not find signal ", s].concat()),
            LHS::OP(s1, o, s2) => match o {
                OPERANT::AND => {
                    *circuit.get(s1).expect("could not find key")
                        & *circuit.get(s2).expect("could not find key")
                }
                OPERANT::OR => {
                    *circuit.get(s1).expect("could not find key")
                        | *circuit.get(s2).expect("could not find key")
                }
            },
            LHS::SHIFT(s, dir, val) => match dir {
                SHIFT::LEFT => *circuit.get(s).expect("could not find key") << val,
                SHIFT::RIGHT => *circuit.get(s).expect("could not find key") >> val,
            },
        };
        circuit.insert(signal, newvalue);
    }

    Ok(("", circuit))
}

#[test]
fn test_parse_instructions() {
    let circuit = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
    let (rem, circuit) = parse_instructions.parse_next(circuit).unwrap();
    assert_eq!(rem, "");

    assert_eq!(circuit.get("d"), Some(&72));
    assert_eq!(circuit.get("e"), Some(&507));
    assert_eq!(circuit.get("f"), Some(&492));
    assert_eq!(circuit.get("g"), Some(&114));
    assert_eq!(circuit.get("h"), Some(&65412));
    assert_eq!(circuit.get("i"), Some(&65079));
    assert_eq!(circuit.get("x"), Some(&123));
    assert_eq!(circuit.get("y"), Some(&456));
}
