use itertools::Itertools;
use std::fs;

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new(data: &str) -> Range {
        let (start, end) = data
            .split("-")
            .map(str::parse)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap();
        Range { start, end }
    }

    fn count_invalid(&self) -> usize {
        (self.start..=self.end)
            .filter(|x| Self::is_invalid(&x.to_string(), 2))
            .sum()
    }

    fn count_all_invalid(&self) -> usize {
        (self.start..=self.end)
            .filter(|id| Self::is_invalid_any_length(&id.to_string()))
            .sum()
    }

    fn is_invalid_any_length(id: &String) -> bool {
        (2..=id.len()).any(|repeat| Self::is_invalid(id, repeat))
    }

    fn is_invalid(id: &String, repeats: usize) -> bool {
        if id.len() % repeats != 0 {
            false
        } else {
            id.chars()
                .chunks(id.len() / repeats)
                .into_iter()
                .map(|chunk| chunk.collect::<String>())
                .all_equal()
        }
    }
}

fn part1(ranges: &Vec<Range>) -> usize {
    ranges.iter().map(Range::count_invalid).sum()
}

fn part2(ranges: &Vec<Range>) -> usize {
    ranges.iter().map(Range::count_all_invalid).sum()
}
pub(crate) fn solve() {
    let contents = fs::read_to_string("2.txt").unwrap();
    let ranges = contents.split(",").map(|data| Range::new(data)).collect();
    println!("{}", part1(&ranges));
    println!("{}", part2(&ranges));
}
