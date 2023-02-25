use regex::Regex;
enum Command {
    TurnOn(u32, u32, u32, u32),
    TurnOff(u32, u32, u32, u32),
    Toggle(u32, u32, u32, u32),
    Err,
}

fn parse_instruction(instruction: &str) -> Command {
    let re: Regex =
        Regex::new(r"(toggle|turn on|turn off) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    if let Some(cap) = re.captures(instruction) {
        let (command, x1, y1, x2, y2) = (&cap[1], &cap[2], &cap[3], &cap[4], &cap[5]);
        if command == "turn on" {
            return Command::TurnOn(
                u32::from_str_radix(x1, 10).unwrap(),
                u32::from_str_radix(y1, 10).unwrap(),
                u32::from_str_radix(x2, 10).unwrap(),
                u32::from_str_radix(y2, 10).unwrap(),
            );
        }
        if command == "turn off" {
            return Command::TurnOff(
                u32::from_str_radix(x1, 10).unwrap(),
                u32::from_str_radix(y1, 10).unwrap(),
                u32::from_str_radix(x2, 10).unwrap(),
                u32::from_str_radix(y2, 10).unwrap(),
            );
        }
        if command == "toggle" {
            return Command::Toggle(
                u32::from_str_radix(x1, 10).unwrap(),
                u32::from_str_radix(y1, 10).unwrap(),
                u32::from_str_radix(x2, 10).unwrap(),
                u32::from_str_radix(y2, 10).unwrap(),
            );
        }
    }
    Command::Err
}

fn turn_on(grid: &mut Vec<u32>, x1: u32, x2: u32, y1: u32, y2: u32) {
    for row in y1..=y2 {
        for col in x1..=x2 {
            let index: usize = (row + col * 1000).try_into().unwrap();
            grid[index] = grid[index] + 1;
        }
    }
}
fn turn_off(grid: &mut Vec<u32>, x1: u32, x2: u32, y1: u32, y2: u32) {
    for row in y1..=y2 {
        for col in x1..=x2 {
            let index: usize = (row + col * 1000).try_into().unwrap();
            if grid[index] > 0 {
                grid[index] = grid[index] - 1;
            }
        }
    }
}

fn toggle(grid: &mut Vec<u32>, x1: u32, x2: u32, y1: u32, y2: u32) {
    for row in y1..=y2 {
        for col in x1..=x2 {
            let index: usize = (row + col * 1000).try_into().unwrap();
            grid[index] = grid[index] + 2;
        }
    }
}

pub fn count_lights(instructions: &str) -> u32 {
    let mut grid = vec![0u32; 1000000];
    for i in instructions.lines() {
        match parse_instruction(i) {
            Command::TurnOn(x1, y1, x2, y2) => turn_on(&mut grid, x1, x2, y1, y2),
            Command::Toggle(x1, y1, x2, y2) => toggle(&mut grid, x1, x2, y1, y2),
            Command::TurnOff(x1, y1, x2, y2) => turn_off(&mut grid, x1, x2, y1, y2),
            Command::Err => {},
        }
    }
    let total_brightness = grid
        .to_vec()
        .into_iter()
        .filter(|&light| light > 0)
        .reduce(|acc, brightness| acc + brightness)
        .unwrap();
    total_brightness
}

#[test]
fn test1() {
    let i: &str = "turn on 0,999 through 999,999
turn on 0,999 through 999,999";
    let total_b = count_lights(i);
    assert_eq!(total_b, 2000);
}
