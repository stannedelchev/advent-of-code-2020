pub trait Problem
where
    Self: Sized,
{
    fn from_file(input_filename: &str) -> Self {
        let input = &std::fs::read_to_string(input_filename).unwrap();
        Self::new(input)
    }

    fn new(input: &str) -> Self;
    fn part1(&self) -> String;
    fn part2(&self) -> String;
}
