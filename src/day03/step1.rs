#[derive(Debug)]
pub struct House {
    pub x: i32,
    pub y: i32,
    pub visited: u32,
}

impl House {
    pub fn visit(self: &mut Self) {
        self.visited = self.visited + 1;
    }
}

pub fn houses_with_one_present(delivery_path: &str) -> u32 {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut houses: Vec<House> = vec![House { x, y, visited: 1 }];
    for c in delivery_path.chars() {
        x = match c {
            '>' => x + 1,
            '<' => x - 1,
            _ => x,
        };
        y = match c {
            'v' => y + 1,
            '^' => y - 1,
            _ => y,
        };
        if let Some(h) = houses.iter_mut().find(|h| (x,y) == (h.x,h.y)) {
            h.visit()
        } else {
            houses.push(House { x, y, visited: 1 })
        }
    }
    houses.len() as u32
}

#[test]
fn example() {
    assert_eq!(houses_with_one_present(">"), 2);
    assert_eq!(houses_with_one_present("^>v<"), 4);
    assert_eq!(houses_with_one_present("^v^v^v^v^v"), 2);
}
