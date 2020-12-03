pub trait Problem
{
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}

pub trait ProblemFactory {
    fn new(input: &str) -> Self;
}