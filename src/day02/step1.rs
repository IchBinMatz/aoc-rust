use std::str::FromStr;

struct Box {
    l: u32,
    w: u32,
    h: u32,
}

impl FromStr for Box {
    type Err = std::num::ParseIntError;
    fn from_str(dimensions: &str) -> Result<Self, Self::Err> {
        let mut split = dimensions.split('x');
        let l = u32::from_str_radix(split.next().unwrap(), 10)?;
        let w = u32::from_str_radix(split.next().unwrap(), 10)?;
        let h = u32::from_str_radix(split.next().unwrap(), 10)?;

        Ok(Self { l, w, h })
    }
}

impl Box {
    fn surface_area(self: &Self) -> u32 {
        let (l, w, h) = (self.l, self.w, self.h);
        2 * l * w + 2 * w * h + 2 * h * l
    }
    fn extra_paper(self: &Self) -> u32 {
        let mut vec = vec![self.l, self.w, self.h];
        vec.sort();
        vec[0] * vec[1]
    }
}

fn calculate_wrapping_paper(dimensions: &str) -> u32 {
    let mybox = Box::from_str(dimensions).unwrap();
    mybox.surface_area() + mybox.extra_paper()
}

pub fn wrapping_paper_all(list: &str) -> u32 {
    let mut area = 0;
    for d in list.lines(){
        area = area + calculate_wrapping_paper(d)
    }

    area
}

#[test]
fn example() {
    assert_eq!(calculate_wrapping_paper("2x3x4"), 58);
    assert_eq!(calculate_wrapping_paper("1x1x10"), 43);
}
