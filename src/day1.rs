use std::fs;

enum Direction {
    Left,
    Right,
}

impl Direction {
    fn new(line: &str) -> Direction {
        let first = line.chars().nth(0).unwrap();
        match first {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Spin {
    direction: Direction,
    amount: i64,
}

impl Spin {
    fn new(line: &str) -> Self {
        let direction = Direction::new(line);
        let amount = line[1..].parse::<i64>().unwrap();
        Self { direction, amount }
    }
}

struct Sequence {
    spins: Vec<Spin>,
}

impl Sequence {
    fn new(content: &str) -> Self {
        Sequence {
            spins: content.lines().map(|line| Spin::new(line)).collect(),
        }
    }
}

struct Safe {
    current: i64,
    limit: i64,
}

impl Safe {
    fn count_zeros(&mut self, sequence: &Sequence) -> usize {
        sequence
            .spins
            .iter()
            .map(|spin| self.apply_spin(spin))
            .filter(|(pos, _)| pos == &0)
            .count()
    }

    fn count_zero_passes(&mut self, sequence: &Sequence) -> usize {
        sequence
            .spins
            .iter()
            .map(|spin| self.apply_spin(spin))
            .map(|(_, flips)| flips)
            .sum()
    }

    fn apply_spin(&mut self, spin: &Spin) -> (i64, usize) {
        let delta = match spin.direction {
            Direction::Left => -spin.amount,
            Direction::Right => spin.amount,
        };
        let position = self.current + delta;
        let dropped = if self.current != 0 {
            (position <= 0) as usize
        } else {
            0
        };
        let flips = (position / (self.limit + 1)).abs() as usize + dropped;
        self.current = position.rem_euclid(self.limit + 1);
        (self.current, flips)
    }
}

fn part1(sequence: &Sequence) -> usize {
    let mut safe = Safe {
        current: 50,
        limit: 99,
    };
    safe.count_zeros(sequence)
}

fn part2(sequence: &Sequence) -> usize {
    let mut safe = Safe {
        current: 50,
        limit: 99,
    };
    safe.count_zero_passes(sequence)
}
pub(crate) fn solve() {
    let contents = fs::read_to_string("1.txt").unwrap();
    let sequence = Sequence::new(&contents);
    println!("{}", part1(&sequence));
    println!("{}", part2(&sequence));
}
