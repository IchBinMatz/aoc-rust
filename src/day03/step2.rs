use crate::day03::step1::House;

pub fn houses_with_one_present(delivery_path: &str) -> u32 {
    let (mut santa_x, mut santa_y) = (0, 0);
    let (mut robo_x, mut robo_y) = (0, 0);
    let mut houses: Vec<House> = vec![House {
        x: 0,
        y: 0,
        visited: 2,
    }];
    let mut robos_turn = false;
    for c in delivery_path.chars() {
        if robos_turn {
            (robo_x, robo_y) = match c {
                '>' => (robo_x + 1, robo_y),
                '<' => (robo_x - 1, robo_y),
                'v' => (robo_x, robo_y + 1),
                '^' => (robo_x, robo_y - 1),
                _ => (robo_x, robo_y),
            };
            if let Some(h) = houses.iter_mut().find(|h| (robo_x, robo_y) == (h.x, h.y)) {
                h.visit()
            } else {
                houses.push(House {
                    x: robo_x,
                    y: robo_y,
                    visited: 1,
                })
            }
        } else {
            (santa_x, santa_y) = match c {
                '>' => (santa_x + 1, santa_y),
                '<' => (santa_x - 1, santa_y),
                'v' => (santa_x, santa_y + 1),
                '^' => (santa_x, santa_y - 1),
                _ => (santa_x, santa_y),
            };
            if let Some(h) = houses.iter_mut().find(|h| (santa_x, santa_y) == (h.x, h.y)) {
                h.visit()
            } else {
                houses.push(House {
                    x: santa_x,
                    y: santa_y,
                    visited: 1,
                })
            }
        }
        robos_turn = !robos_turn
    }
    houses.len() as u32
}

#[test]
fn example() {
    assert_eq!(houses_with_one_present("^v"), 3);
    assert_eq!(houses_with_one_present("^>v<"), 3);
    assert_eq!(houses_with_one_present("^v^v^v^v^v"), 11);
}
