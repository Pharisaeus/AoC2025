use std::collections::{HashMap, HashSet};
use std::fs;

struct Board {
    neighbours: HashMap<(i64, i64), usize>,
}

impl Board {
    fn new(data: &str) -> Self {
        let positions: HashSet<(i64, i64)> = data
            .lines()
            .enumerate()
            .map(|(row, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| c.eq(&'@'))
                    .map(move |(col, _)| (row as i64, col as i64))
            })
            .flatten()
            .collect();
        Self {
            neighbours: positions
                .iter()
                .map(|&pos| (pos, Self::find_neighbours(&positions, &pos).len()))
                .collect(),
        }
    }

    fn count_can_be_moved(&self) -> usize {
        self.neighbours.values().filter(|&&v| v < 4).count()
    }

    fn can_be_moved(&self, pos: &(i64, i64)) -> bool {
        self.neighbours.get(pos).unwrap() < &4
    }

    fn find_neighbours(
        positions: &HashSet<(i64, i64)>,
        (row, col): &(i64, i64),
    ) -> Vec<(i64, i64)> {
        Self::neighbour_deltas()
            .iter()
            .map(|(dr, dc)| (row + dr, col + dc))
            .filter(|pos| positions.contains(&(pos)))
            .collect()
    }

    fn remove_movable(&mut self) {
        let to_remove: Vec<(i64, i64)> = self
            .neighbours
            .keys()
            .filter(|pos| self.can_be_moved(pos))
            .map(|pos| *pos)
            .collect();
        let positions = self.neighbours.keys().map(|x| *x).collect();
        let to_decrement: Vec<(i64, i64)> = to_remove
            .iter()
            .map(|pos| Self::find_neighbours(&positions, pos))
            .flatten()
            .collect();
        for pos in to_decrement {
            self.neighbours.insert(pos, self.neighbours[&pos] - 1);
        }
        for pos in to_remove {
            self.neighbours.remove(&pos);
        }
    }

    fn neighbour_deltas() -> Vec<(i64, i64)> {
        vec![
            (0, -1),
            (0, 1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (-1, -1),
            (1, 1),
            (1, -1),
        ]
    }
}
fn part1(board: &Board) -> usize {
    board.count_can_be_moved()
}

fn part2(board: &Board) -> usize {
    let mut work_board = Board {
        neighbours: board.neighbours.clone(),
    };
    while work_board.count_can_be_moved() > 0 {
        work_board.remove_movable()
    }
    board.neighbours.len() - work_board.neighbours.len()
}
pub(crate) fn solve() {
    let contents = fs::read_to_string("4.txt").unwrap();
    let board = Board::new(&contents);
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
