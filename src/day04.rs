use crate::common::{Problem, ProblemFactory};
use regex::Regex;
use std::collections::HashMap;
use std::str::Lines;

pub struct Day04 {
    input: String,
}

#[derive(Debug)]
pub struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl Passport<'_> {
    fn valid_fields(&self) -> bool {
        self.fields.contains_key("byr")
            && self.fields.contains_key("iyr")
            && self.fields.contains_key("eyr")
            && self.fields.contains_key("hgt")
            && self.fields.contains_key("hcl")
            && self.fields.contains_key("ecl")
            && self.fields.contains_key("pid")
    }

    fn valid_fields_values(&self) -> bool {
        lazy_static! {
            static ref HEIGHT_REGEX: Regex =
                Regex::new(r"^(?P<value>\d*)(?P<type>cm|in)$").unwrap();
            static ref HAIR_COLOR_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            static ref EYE_COLOR_REGEX: Regex =
                Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
            static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
        }

        let byr = self.fields["byr"].parse::<usize>().unwrap();
        let yir = self.fields["iyr"].parse::<usize>().unwrap();
        let eyr = self.fields["eyr"].parse::<usize>().unwrap();

        let byr_ok = 1920 <= byr && byr <= 2002;
        let iyr_ok = 2010 <= yir && yir <= 2020;
        let eyr_ok = 2020 <= eyr && eyr <= 2030;
        let hgt_ok = {
            if let Some(c) = HEIGHT_REGEX.captures_iter(&self.fields["hgt"]).next() {
                let value = c["value"].parse::<usize>().unwrap();
                if &c["type"] == "cm" {
                    150 <= value && value <= 193
                } else if &c["type"] == "in" {
                    59 <= value && value <= 76
                } else {
                    false
                }
            } else {
                false
            }
        };
        let hcl_ok = HAIR_COLOR_REGEX.is_match(&self.fields["hcl"]);
        let ecl_ok = EYE_COLOR_REGEX.is_match(&self.fields["ecl"]);
        let pid_ok = PID_REGEX.is_match(&self.fields["pid"]);

        byr_ok && iyr_ok && eyr_ok && hgt_ok && hcl_ok && ecl_ok && pid_ok
    }
}

impl<'a> ProblemFactory for Day04 {
    fn new(input: &str) -> Self {
        Day04 {
            input: input.to_string(),
        }
    }
}

impl<'a> Problem for Day04 {
    fn part1(&self) -> String {
        self.passports()
            .filter(|p| p.valid_fields())
            .count()
            .to_string()
    }

    fn part2(&self) -> String {
        self.passports()
            .filter(|p| p.valid_fields() && p.valid_fields_values())
            .count()
            .to_string()
    }
}

struct PassportIterator<'a> {
    lines_iterator: Lines<'a>,
}

impl<'a> Iterator for PassportIterator<'a> {
    type Item = Passport<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut map = HashMap::<&str, &str>::new();

        while let Some(c) = self.lines_iterator.next() {
            if c.trim().is_empty() {
                let passport = Passport { fields: map };
                return Some(passport);
            } else {
                c.split_whitespace().for_each(|s| {
                    let mut column_split = s.split(":");
                    let field = column_split.next().unwrap();
                    let value = column_split.next().unwrap();
                    map.insert(field, value);
                });
            }
        }

        if map.is_empty() {
            return None;
        }

        let passport = Passport { fields: map };
        return Some(passport);
    }
}

trait IteratePassports {
    fn passports(&self) -> PassportIterator;
}

impl IteratePassports for Day04 {
    fn passports(&self) -> PassportIterator {
        PassportIterator {
            lines_iterator: self.input.lines(),
        }
    }
}
