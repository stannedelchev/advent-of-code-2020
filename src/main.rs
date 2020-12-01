mod common;
mod day01;

use crate::day01::Day01;
use common::Problem;
use std::time::Duration;

fn main() {
    let days = [&Day01::from_file("inputs/Day01.txt")];
    let days = days.iter().enumerate();
    for (idx, &day) in days {
        let (part1_time, part1_result) = time(|| day.part1(), 10000);
        let (part2_time, part2_result) = time(|| day.part2(), 10000);
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
}

fn time<F>(func: F, loops: usize) -> (Duration, String)
where
    F: Fn() -> String,
{
    let mut durations = Vec::<Duration>::new();
    let mut result: String = String::new();

    for i in 0..=loops {
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
