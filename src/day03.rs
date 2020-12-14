use crate::common::Problem;
use std::fmt::Debug;

pub struct Day03 {
    grid: Vec<Vec<Location>>,
}

#[derive(Debug)]
pub enum Location {
    Ground,
    Tree,
}

#[derive(Copy, Clone, Debug)]
struct XyTuple {
    x: usize,
    y: usize,
}

impl Day03 {
    pub fn new(input: String) -> Self {
        let mut rows = Vec::<Vec<Location>>::new();
        for line in input.lines() {
            let line_as_nums = line
                .chars()
                .map(|c| match c {
                    '.' => Location::Ground,
                    '#' => Location::Tree,
                    _ => unreachable!(),
                })
                .collect::<Vec<Location>>();

            rows.push(line_as_nums);
        }

        Day03 { grid: rows }
    }
}

impl std::ops::Add for XyTuple {
    type Output = XyTuple;

    fn add(self, rhs: Self) -> Self::Output {
        XyTuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Problem for Day03 {
    fn part1(&self) -> String {
        self.iter_trees(XyTuple { x: 3, y: 1 }).count().to_string()
    }

    fn part2(&self) -> String {
        [
            self.iter_trees(XyTuple { x: 1, y: 1 }).count(),
            self.iter_trees(XyTuple { x: 3, y: 1 }).count(),
            self.iter_trees(XyTuple { x: 5, y: 1 }).count(),
            self.iter_trees(XyTuple { x: 7, y: 1 }).count(),
            self.iter_trees(XyTuple { x: 1, y: 2 }).count(),
        ]
        .iter()
        .fold(1, |acc, i| acc * i)
        .to_string()
    }
}

struct TreeIterator<'a> {
    deltas: XyTuple,
    current_pos: XyTuple,
    source: &'a Day03,
}

impl<'a> TreeIterator<'a> {
    fn rows(&self) -> usize {
        self.source.grid.len()
    }

    fn columns(&self) -> usize {
        self.source.grid[0].len()
    }

    fn move_current_pos(&mut self) {
        self.current_pos = self.current_pos + self.deltas;
        self.current_pos.x %= self.columns();
    }
}

impl<'a> Iterator for TreeIterator<'a> {
    type Item = XyTuple;

    fn next(&mut self) -> Option<Self::Item> {
        let grid = &self.source.grid;
        let rows = self.rows();

        loop {
            if self.current_pos.y >= rows {
                return None;
            }

            let location = &grid[self.current_pos.y][self.current_pos.x];
            match location {
                Location::Tree => {
                    let result = Some(self.current_pos);
                    self.move_current_pos();
                    return result;
                }
                _ => {}
            }

            self.move_current_pos();

            if self.current_pos.y >= rows {
                return None;
            }
        }
    }
}

trait TreeIteratorFactory {
    fn iter_trees(&self, delta: XyTuple) -> TreeIterator;
}

impl TreeIteratorFactory for Day03 {
    fn iter_trees(&self, deltas: XyTuple) -> TreeIterator {
        TreeIterator {
            deltas,
            current_pos: deltas,
            source: self,
        }
    }
}
