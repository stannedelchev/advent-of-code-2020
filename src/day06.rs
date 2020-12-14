use crate::common::Problem;

pub struct Day06 {
    input: String,
}

impl Day06 {
    pub fn new(input: String) -> Self {
        Day06 { input }
    }
}

impl Problem for Day06 {
    fn part1(&self) -> String {
        self.answers()
            .map(|ga| ga.unique)
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self) -> String {
        self.answers()
            .map(|ga| ga.answered_by_all)
            .sum::<usize>()
            .to_string()
    }
}

#[derive(Debug)]
struct GroupAnswers {
    answers: [usize; 26],
    unique: usize,
    answered_by_all: usize,
    people_in_group: usize,
}

struct GroupAnswersIterator<'a> {
    input: &'a str,
}

impl<'a> Iterator for GroupAnswersIterator<'a> {
    type Item = GroupAnswers;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }

        let mut result = [0; 26];

        let input = self.input;
        let mut index = 0;
        let mut eol_seen = false;
        let mut people_in_group = 0;

        for c in input.chars() {
            index += 1;

            if c.is_ascii_whitespace() {
                if eol_seen {
                    break;
                }

                people_in_group += 1;
                eol_seen = true;
                continue;
            }
            eol_seen = false;
            let char_ascii_value = c as usize - 97;
            result[char_ascii_value] += 1;
        }

        self.input = &self.input[index..];

        let answers = result;
        let unique = result
            .iter()
            .map(|i| if *i >= 1 { 1 } else { 0 })
            .sum::<usize>();
        let answered_by_all = result
            .iter()
            .filter(|&i| *i == people_in_group)
            .map(|i| if *i >= 1 { 1 } else { 0 })
            .sum::<usize>();

        Some(GroupAnswers {
            answers,
            unique,
            answered_by_all,
            people_in_group,
        })
    }
}

trait CustomsFormIterator {
    fn answers(&self) -> GroupAnswersIterator;
}

impl CustomsFormIterator for Day06 {
    fn answers(&self) -> GroupAnswersIterator {
        GroupAnswersIterator { input: &self.input }
    }
}
