use crate::common::Problem;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Day07 {
    input: String,
    bags: HashMap<String, Vec<Bag>>,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Bag {
    color: String,
    count: usize,
}

impl From<&str> for Bag {
    fn from(s: &str) -> Self {
        Bag {
            count: s[0..1].parse().unwrap(),
            color: s[2..].to_string(),
        }
    }
}

impl Day07 {
    pub fn new(input: String) -> Self {
        let delimiters = Regex::new(r"( bags?,? ?|contain |\.)").unwrap();
        let mut all_bags: HashMap<String, Vec<Bag>> = HashMap::new();

        for line in input.lines() {
            let mut parts = delimiters.split(line).filter(|&p| !p.is_empty());
            let container_bag_color = parts.next().unwrap();

            for bag in parts
                .filter(|p| !p.starts_with("no other"))
                .map_into::<Bag>()
            {
                let parent_bag = Bag {
                    color: container_bag_color.to_string(),
                    count: bag.count,
                };
                match all_bags.get_mut(&bag.color) {
                    Some(parents) => {
                        parents.push(parent_bag);
                    }
                    None => {
                        all_bags.insert(bag.color, vec![parent_bag]);
                    }
                }
            }
        }

        Day07 {
            input,
            bags: all_bags,
        }
    }
}

impl Problem for Day07 {
    fn part1(&self) -> String {
        let mut to_visit = vec![];
        let mut visited = HashSet::new();

        to_visit.extend(self.bags.get("shiny gold").unwrap());

        while !to_visit.is_empty() {
            let bag = to_visit.pop().unwrap();
            if let Some(bag_parents) = self.bags.get(&bag.color) {
                to_visit.extend(bag_parents);
            }
            visited.insert(&bag.color);
        }

        visited.len().to_string()
    }

    fn part2(&self) -> String {
        String::new()
    }
}
