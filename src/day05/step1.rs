fn naughty_or_nice(l: &str) -> bool {
    if contains_at_less_than_three_vowels(l) {
        return false;
    }
    if !contains_double_letters(l) {
        return false;
    }
    if l.contains("ab") {
        return false;
    };
    if l.contains("cd") {
        return false;
    };
    if l.contains("pq") {
        return false;
    };
    if l.contains("xy") {
        return false;
    };
    true
}

fn contains_at_less_than_three_vowels(s: &str) -> bool {
    let vowels: Vec<char> = s
        .chars()
        .into_iter()
        .filter(|c| "aeiou".contains(&c.to_string()))
        .collect();
    vowels.len() < 3
}

fn contains_double_letters(s: &str) -> bool {
    let mut chars = s.chars();
    let mut old_letter = chars.next().unwrap();
    for c in chars {
        if c == old_letter {
            return true;
        }
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
    assert_eq!(naughty_or_nice("ugknbfddgicrmopn"), true);
}
#[test]
fn example2() {
    assert_eq!(naughty_or_nice("aaa"), true);
}

#[test]
fn example3() {
    assert_eq!(naughty_or_nice("jchzalrnumimnmhp"), false);
}

#[test]
fn example4() {
    assert_eq!(naughty_or_nice("haegwjzuvuyypxyu"), false);
}

#[test]
fn example5() {
    assert_eq!(naughty_or_nice("dvszwmarrgswjxmb"), false);
}
