use crate::common::{Problem, ProblemFactory};
use regex::Regex;
use std::collections::HashMap;

pub struct Day02 {
    passwords: Vec<PasswordLine>
}

#[derive(Debug)]
pub struct PasswordLine {
    num1: u32,
    num2: u32,
    letter: char,
    password: String
}

impl From<&str> for PasswordLine {
    fn from(s: &str) -> Self {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"(?P<min>[\d]{1,3})-(?P<max>[\d]{1,3}) (?P<letter>[a-z]): (?P<password>[a-z]+)").unwrap();
        }

        let content = REGEX.captures_iter(s).next().unwrap();

        PasswordLine {
            num1: content["min"].parse::<u32>().unwrap(),
            num2: content["max"].parse::<u32>().unwrap(),
            letter: content["letter"].chars().next().unwrap(),
            password : content["password"].to_string()
        }
    }
}

impl ProblemFactory for Day02{
    fn new(input: &str) -> Self {
        let passwords: Vec<PasswordLine> = input.lines().map(|l| l.into()).collect();
        Day02 {
            passwords
        }
    }
}

impl Problem for Day02 {
    fn part1(&self) -> String {
        let wrong_passwords = self.passwords.iter()
            .filter(|l| {
                let mut map: HashMap<char, u32> = HashMap::new();
                for c in l.password.chars() {
                    map .insert(c, map.get(&c).map_or_else(|| 1, |count| *count + 1));
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
        let wrong_passwords = self.passwords.iter()
            .filter(|l| {
                let c1 = l.password.chars().nth((l.num1 - 1) as usize).unwrap();
                let c2 = l.password.chars().nth((l.num2 - 1) as usize).unwrap();

                let letter = l.letter;
                return (c1 == letter || c2 == letter) && c1 != c2;
            })
            .count();

        wrong_passwords.to_string()
    }
}