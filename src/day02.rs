use crate::common::{Problem, ProblemFactory};
use regex::Regex;
use std::collections::HashMap;

pub struct Day02 {
    passwords: Vec<PasswordLine>,
}

#[derive(Debug)]
pub struct PasswordLine {
    num1: u32,
    num2: u32,
    letter: char,
    password: String,
}

impl From<&str> for PasswordLine {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(
                r"(?P<min>[\d]{1,3})-(?P<max>[\d]{1,3}) (?P<letter>[a-z]): (?P<password>[a-z]+)"
            )
            .unwrap();
        }

        let content = REGEX.captures_iter(s).next().unwrap();

        PasswordLine {
            num1: content["min"].parse::<u32>().unwrap(),
            num2: content["max"].parse::<u32>().unwrap(),
            letter: content["letter"].chars().next().unwrap(),
            password: content["password"].to_string(),
        }
    }
}

impl ProblemFactory for Day02 {
    fn new(input: &str) -> Self {
        let passwords: Vec<PasswordLine> = input.lines().map(|l| l.into()).collect();
        Day02 { passwords }
    }
}

impl Problem for Day02 {
    fn part1(&self) -> String {
        let wrong_passwords = self
            .passwords
            .iter()
            .filter(|l| {
                let mut map: HashMap<char, u32> = HashMap::new();
                for c in l.password.chars() {
                    map.insert(c, map.get(&c).map_or_else(|| 1, |count| *count + 1));
                }

                let letter = &l.letter;
                if !&map.contains_key(letter) {
                    return false;
                }
                let occurrences = (&map).get(letter).unwrap();
                return l.num1 <= *occurrences && *occurrences <= l.num2;
            })
            .count();

        wrong_passwords.to_string()
    }

    fn part2(&self) -> String {
        let wrong_passwords = self
            .passwords
            .iter()
            .filter(|l| {
                let letter = l.letter;
                let mut char_idx = 0;
                let mut seen = 0;
                for c in l.password.chars() {
                    char_idx += 1;

                    if (char_idx == l.num1) && c == letter {
                        seen += 1;
                        continue;
                    }

                    if char_idx == l.num2 {
                        if c == letter {
                            seen += 1;
                        }
                        break;
                    }
                }

                seen == 1
            })
            .count();

        wrong_passwords.to_string()
    }
}
