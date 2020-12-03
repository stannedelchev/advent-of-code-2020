use crate::common::{ProblemFactory, Problem};
use std::fmt::{Debug};

pub struct Day03 {
    grid: Vec<Vec<Location>>
}

#[derive(Debug)]
pub enum Location {
    Ground,
    Tree
}

impl ProblemFactory for Day03 {
    fn new(input: &str) -> Self {

        let mut rows = Vec::<Vec<Location>>::new();
        for line in input.lines() {
            let line_as_nums = line.chars().map(|c| match c {
               '.' => Location::Ground,
                '#' => Location::Tree,
                _ => unreachable!()
            }).collect::<Vec<Location>>();

            rows.push(line_as_nums);
        }

        Day03 {
            grid: rows
        }
    }
}
#[derive(Copy, Clone, Debug)]
struct XyTuple {
    x: usize,
    y: usize,
}

impl std::ops::Add for XyTuple {
    type Output = XyTuple;

    fn add(self, rhs: Self) -> Self::Output {
        XyTuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Problem for Day03 {
    fn part1(&self) -> String {
        let deltas = XyTuple { x: 3, y: 1};
        let mut current_pos = deltas;
        let grid = &self.grid;
        let mut result = 0;

        let rows = grid.len();
        let columns = grid[0].len();

        loop {
            let location = &grid[current_pos.y][current_pos.x];
            match location {
                Location::Tree => result += 1,
                _ => {}
            }

            current_pos = current_pos + deltas;
            current_pos.x %= columns;

            if current_pos.y >= rows {
                return result.to_string();
            }
        }
    }

    fn part2(&self) -> String {
        "".to_string()
    }
}