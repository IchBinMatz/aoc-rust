fn naughty_or_nice(l: &str) -> bool {
    if l.is_empty() {return false;}
    contains_double_letters_with_one_inbetween(l) && contains_two_pairs(l) 
}

fn contains_two_pairs(s: &str) -> bool {
    let mut i = 0;
    loop {
        let pair = &s[i..i+2];
        if s[i+2..].contains(pair) {return true;}
        i = i + 1;
        if i > s.len()-2 {break;}
    }
    false
}

fn contains_double_letters_with_one_inbetween(s: &str) -> bool {
    let mut chars = s.chars();
    let mut older_letter = chars.next().unwrap();
    let mut old_letter = chars.next().unwrap();
    for c in chars {
        if c == older_letter {
            return true;
        }
        older_letter = old_letter;
        old_letter = c;
    }
    false
}

pub fn help_santa(text_file: &str) -> u32 {
    let nice_strings: Vec<&str> = text_file
        .lines()
        .into_iter()
        .filter(|l| naughty_or_nice(l))
        .collect();
    nice_strings.len() as u32
}

#[test]
fn example1() {
    assert_eq!(naughty_or_nice("qjhvhtzxzqqjkmpb"), true);
}
#[test]
fn example2() {
    assert_eq!(naughty_or_nice("xxyxx"), true);
}
#[test]
fn example3() {
    assert_eq!(naughty_or_nice("uurcxstgmygtbstg"), false);
}
#[test]
fn example4() {
    assert_eq!(naughty_or_nice("ieodomkazucvgmuy"), false);
}
