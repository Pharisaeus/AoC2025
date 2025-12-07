use std::collections::{HashMap, HashSet};
use std::fs;

struct Manifold {
    start: usize,
    rows: usize,
    splitters: HashSet<(usize, usize)>,
}

impl Manifold {
    fn new(data: &str) -> Self {
        let mut splitters = HashSet::new();
        let mut start = 0;
        let mut rows = 0;
        for (row, line) in data.lines().enumerate() {
            for (col, c) in line.chars().enumerate() {
                match (c) {
                    'S' => start = col,
                    '^' => {
                        splitters.insert((row, col));
                    }
                    '.' => continue,
                    _ => panic!(),
                }
            }
            rows = row;
        }
        Manifold {
            start,
            rows,
            splitters,
        }
    }

    fn count_splits(&self) -> usize {
        let mut beam_cols = HashSet::new();
        beam_cols.insert(self.start);
        let mut splits = 0;
        for row in 1..self.rows {
            let mut new_beam_cols = HashSet::new();
            for col in beam_cols {
                if self.splitters.contains(&(row, col)) {
                    new_beam_cols.insert(col - 1);
                    new_beam_cols.insert(col + 1);
                    splits += 1;
                } else {
                    new_beam_cols.insert(col);
                }
            }
            beam_cols = new_beam_cols;
        }
        splits
    }

    fn count_timelines(&self) -> usize {
        self.count_timelines_recursively((0, self.start), &mut HashMap::new())
    }

    fn count_timelines_recursively(
        &self,
        (row, col): (usize, usize),
        known: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if known.contains_key(&(row, col)) {
            known[&(row, col)]
        } else if row == self.rows {
            known.insert((row, col), 1);
            1
        } else if self.splitters.contains(&(row, col)) {
            let left = self.count_timelines_recursively((row + 1, col - 1), known);
            let right = self.count_timelines_recursively((row + 1, col + 1), known);
            known.insert((row, col), left + right);
            left + right
        } else {
            self.count_timelines_recursively((row + 1, col), known)
        }
    }
}
fn part1(manifold: &Manifold) -> usize {
    manifold.count_splits()
}
fn part2(manifold: &Manifold) -> usize {
    manifold.count_timelines()
}
pub(crate) fn solve() {
    let contents = fs::read_to_string("7.txt").unwrap();
    let manifold = Manifold::new(&contents);
    println!("{}", part1(&manifold));
    println!("{}", part2(&manifold));
}
