use std::collections::HashSet;

#[derive(Debug)]
pub struct Day01 {
    input: Vec<u64>,
}

impl crate::common::Problem for Day01 {
    fn new(input_contents: &str) -> Day01 {
        let input: Vec<u64> = input_contents
            .lines()
            .map(|l| l.parse::<u64>().unwrap())
            .collect();
        Day01 { input }
    }

    fn part1(&self) -> String {
        for n1 in &self.input {
            for n2 in &self.input {
                if n1 + n2 == 2020 {
                    return (n1 * n2).to_string();
                }
            }
        }

        unreachable!()
    }

    fn part2(&self) -> String {
        let mut map = HashSet::<u64>::with_capacity(self.input.len());
        for i in self.input.iter() {
            map.insert(*i);
        }

        for i in &self.input {
            for j in &self.input {
                let k = 2020 - i - j;
                if map.contains(&k) {
                    return (i * j * k).to_string();
                }
            }
        }

        unreachable!()
    }
}
