use std::str::FromStr;

use crate::day02::step1::Box;

impl Box {
    fn volume(self: &Self) -> u32 {
        self.l * self.w * self.h
    }

    fn feet_of_ribbon(self: &Self) -> u32{
        let mut vec = vec![self.l, self.w, self.h];
        vec.sort();
        let (shortest, second) = (vec[0], vec[1]);
        let ribbon = shortest + shortest + second + second;
        let bow = self.volume();
        ribbon + bow
        
    }
}
fn calculate_ribbon(dimensions: &str) -> u32 {
    let my_box = Box::from_str(dimensions).unwrap();
    my_box.feet_of_ribbon()
}

pub fn total_feet_of_ribbon(list : &str) -> u32 {
    let mut ribbon = 0;
    for d in list.lines() {
        ribbon = ribbon + calculate_ribbon(d);
    }
    ribbon
}

#[test]
fn example() {
    assert_eq!(calculate_ribbon("2x3x4"), 34);
    assert_eq!(calculate_ribbon("1x1x10"), 14);
}