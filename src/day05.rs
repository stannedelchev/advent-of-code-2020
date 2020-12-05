use crate::common::{Problem, ProblemFactory};

pub struct Day05 {
    input: Vec<usize>,
}

impl ProblemFactory for Day05 {
    fn new(input: &str) -> Self {
        Day05 {
            input: input
                .to_string()
                .replace("B", "1")
                .replace("F", "0")
                .replace("R", "1")
                .replace("L", "0")
                .lines()
                .map(|l| usize::from_str_radix(l, 2).unwrap())
                .collect(),
        }
    }
}

impl Problem for Day05 {
    fn part1(&self) -> String {
        self.input.iter().max().unwrap().to_string()
    }

    fn part2(&self) -> String {
        let mut seats = self.input.clone();
        seats.sort();

        let sorted_seats = seats;
        (sorted_seats[0..sorted_seats.len() - 1]
            .windows(2)
            .filter(|i| i[0] + 2 == i[1])
            .next()
            .unwrap()[0]
            + 1)
        .to_string()
    }
}
