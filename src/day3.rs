use std::fs;
struct Bank {
    batteries: Vec<usize>,
}

impl Bank {
    fn new(line: &str) -> Self {
        Self {
            batteries: line
                .chars()
                .map(|x| x.to_string())
                .map(|x| x.parse::<usize>())
                .map(Result::unwrap)
                .collect(),
        }
    }

    fn find_max_two(&self) -> usize {
        let len = self.batteries.len();
        let max = self.find_max_index(&self.batteries);
        if max == len - 1 {
            // max is the right-most
            let left = self.batteries[..max].iter().max().unwrap();
            10 * left + self.batteries[max]
        } else {
            let right = self.batteries[max + 1..].iter().max().unwrap();
            10 * self.batteries[max] + right
        }
    }

    fn find_max_n(&self, needed: usize) -> usize {
        let mut start_position = 0;
        let mut result = 0;
        for still_needed in (0..needed).rev() {
            let end_position = self.batteries.len() - still_needed;
            let search_space = &self.batteries[start_position..end_position];
            let max_index = self.find_max_index(search_space);
            start_position = start_position + max_index + 1;
            result = result * 10 + search_space[max_index]
        }
        result
    }
    fn find_max_index(&self, search_space: &[usize]) -> usize {
        // find index of first occurrence of max value
        search_space
            .iter()
            .enumerate()
            .rev()
            .max_by_key(|(idx, v)| *v)
            .map(|(idx, _)| idx)
            .unwrap()
    }
}

fn part1(ranges: &Vec<Bank>) -> usize {
    ranges.iter().map(Bank::find_max_two).sum()
}

fn part2(ranges: &Vec<Bank>) -> usize {
    ranges.iter().map(|x| x.find_max_n(12)).sum()
}
pub(crate) fn solve() {
    let contents = fs::read_to_string("3.txt").unwrap();
    let banks = contents.lines().map(|line| Bank::new(line)).collect();
    println!("{}", part1(&banks));
    println!("{}", part2(&banks));
}
