use crate::common::Problem;
use regex::Regex;

pub struct Day02 {
    passwords: Vec<PasswordLine>,
}

#[derive(Debug)]
pub struct PasswordLine {
    num1: usize,
    num2: usize,
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
            num1: content["min"].parse::<usize>().unwrap(),
            num2: content["max"].parse::<usize>().unwrap(),
            letter: content["letter"].chars().next().unwrap(),
            password: content["password"].to_string(),
        }
    }
}

impl Day02 {
    pub fn new(input: String) -> Self {
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
                const ASCII_SMALL_LETTERS_STARTPOINT: usize = 97;
                const ALPHABET_LENGTH: usize = 26;

                let mut letter_occurrences: [usize; ALPHABET_LENGTH] = [0; ALPHABET_LENGTH];

                for c in l.password.chars() {
                    letter_occurrences[c as usize - ASCII_SMALL_LETTERS_STARTPOINT] += 1;
                }

                let occurrences =
                    letter_occurrences[l.letter as usize - ASCII_SMALL_LETTERS_STARTPOINT];

                l.num1 <= occurrences && occurrences <= l.num2
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
