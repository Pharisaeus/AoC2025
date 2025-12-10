use itertools::Itertools;
use std::fs;
use std::str::FromStr;

struct Switchboard {
    target: Vec<usize>,
    switches: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Switchboard {
    fn new(line: &str) -> Self {
        let (t, rest) = line.split(']').collect_tuple().unwrap();
        let target = t
            .chars()
            .skip(1)
            .map(|c| match (c) {
                '.' => 0,
                '#' => 1,
                _ => panic!(),
            })
            .collect();
        let (s, j) = rest.split("{").collect_tuple().unwrap();
        let switches = s
            .trim()
            .split(" ")
            .map(|switch| {
                switch
                    .replace("(", "")
                    .replace(")", "")
                    .split(",")
                    .map(usize::from_str)
                    .map(Result::unwrap)
                    .collect()
            })
            .collect();
        let joltages = j
            .replace("}", "")
            .split(",")
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect();
        Self {
            target,
            switches,
            joltages,
        }
    }

    fn check(&self, switches: &Vec<usize>) -> bool {
        let mut res = vec![0; self.target.len()];
        let selected: Vec<&Vec<usize>> = switches
            .iter()
            .map(|idx| self.switches.get(*idx).unwrap())
            .collect();
        for switch in selected {
            for flip in switch {
                res[*flip] = (res[*flip] + 1) % 2;
            }
        }
        res == self.target
    }

    fn least_presses(&self) -> usize {
        for presses in 1..self.switches.len() {
            for combination in (0..self.switches.len()).into_iter().combinations(presses) {
                if self.check(&combination) {
                    return presses;
                }
            }
        }
        0
    }
}

fn part1(switchboards: &Vec<Switchboard>) -> usize {
    switchboards
        .iter()
        .map(move |switchboard| switchboard.least_presses())
        .sum()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("10.txt").unwrap();
    let switchboards = contents
        .lines()
        .map(|line| Switchboard::new(line))
        .collect();
    println!("{}", part1(&switchboards));
}
