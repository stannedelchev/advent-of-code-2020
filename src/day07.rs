use crate::common::Problem;
use regex::Regex;
use std::collections::{HashMap, HashSet};

#[derive(Eq, PartialEq, Hash, Debug)]
struct BagRule {
    parent: String,
    color: String,
    count: usize,
}

pub struct Day07 {
    children: HashMap<String, Vec<BagRule>>,
    parents: HashMap<String, Vec<String>>,
}

impl BagRule {
    fn new(s: &str, parent: String) -> Self {
        BagRule {
            parent,
            color: s[2..].to_string(),
            count: s[0..1].parse().unwrap(),
        }
    }
}

impl Day07 {
    pub fn new(input: String) -> Self {
        let delimiters = Regex::new(r"( bags?,? ?|contain |\.)").unwrap();
        let mut parents = HashMap::new();
        let mut children = HashMap::new();
        input.lines().for_each(|line| {
            let mut parts = delimiters.split(line).filter(|&p| !p.is_empty());
            let container_bag_color = parts.next().unwrap();

            for rule in parts
                .filter(|p| !p.starts_with("no other"))
                .map(|s| BagRule::new(s, container_bag_color.to_string()))
            {
                let parent_vec = parents.entry(rule.color.clone()).or_insert(vec![]);
                (*parent_vec).push(rule.parent.clone());

                let child_vec = children.entry(rule.parent.clone()).or_insert(vec![]);
                (*child_vec).push(rule);
            }
        });

        Day07 { children, parents }
    }

    fn sum_bag(&self, bag: &str) -> usize {
        match self.children.get(bag) {
            None => 0,
            Some(child_bags) => child_bags
                .iter()
                .fold(0, |acc, v| acc + v.count + v.count * self.sum_bag(&v.color)),
        }
    }
}

impl Problem for Day07 {
    fn part1(&self) -> String {
        let shiny_gold_parents = self.parents.get("shiny gold").unwrap();

        let mut to_visit = shiny_gold_parents.clone();
        let mut visited = HashSet::new();

        while !to_visit.is_empty() {
            let bag = to_visit.pop().unwrap();

            if let Some(bag_parents) = self.parents.get(&bag) {
                for parent in bag_parents.iter() {
                    to_visit.push(parent.to_string());
                }
            }
            visited.insert(bag);
        }

        visited.len().to_string()
    }

    fn part2(&self) -> String {
        self.sum_bag("shiny gold").to_string()
    }
}
