mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use crate::day01::Day01;
use crate::day02::Day02;
use crate::day03::Day03;
use crate::day04::Day04;
use crate::day05::Day05;
use crate::day06::Day06;
use crate::day07::Day07;
use crate::day08::Day08;
use common::Problem;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

fn main() {
    let (total_time, _) = time(
        || {
            let days: Vec<Box<dyn Problem>> = vec![
                Box::from(Day01::new(read_file("inputs/Day01.txt"))),
                Box::from(Day02::new(read_file("inputs/Day02.txt"))),
                Box::from(Day03::new(read_file("inputs/Day03.txt"))),
                Box::from(Day04::new(read_file("inputs/Day04.txt"))),
                Box::from(Day05::new(read_file("inputs/Day05.txt"))),
                Box::from(Day06::new(read_file("inputs/Day06.txt"))),
                Box::from(Day07::new(read_file("inputs/Day07.txt"))),
                Box::from(Day08::new(read_file("inputs/Day08.txt"))),
            ];
            let days = days.iter().enumerate();
            for (idx, day) in days {
                let (part1_time, part1_result) = time(|| (*day).part1(), 1);
                let (part2_time, part2_result) = time(|| (*day).part2(), 1);
                println!(
                    "Day {} part 1: {} in {}sec",
                    idx + 1,
                    part1_result,
                    part1_time.as_secs_f64()
                );
                println!(
                    "Day {} part 2: {} in {}sec",
                    idx + 1,
                    part2_result,
                    part2_time.as_secs_f64()
                );
            }
            "".to_string()
        },
        1,
    );
    println!("Total time: {}sec", total_time.as_secs_f64());
}

fn time<F>(func: F, loops: usize) -> (Duration, String)
where
    F: Fn() -> String,
{
    let mut durations = Vec::<Duration>::new();
    let mut result: String = String::new();

    for _ in 0..loops {
        let now = std::time::Instant::now();
        result = func();
        durations.push(now.elapsed());
    }

    let total_duration = {
        let mut total_duration = Duration::default();
        for &d in durations.iter() {
            total_duration += d;
        }
        total_duration
    };

    let average_duration = total_duration.div_f32(durations.len() as f32);
    (average_duration, result)
}

fn read_file(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap()
}
