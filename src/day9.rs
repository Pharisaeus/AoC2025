use geo::{Contains, LineString, Polygon, line_string};
use itertools::Itertools;
use std::fs;
use std::iter::repeat;

struct Board {
    corners: Vec<(usize, usize)>,
}

impl Board {
    fn new(data: &str) -> Self {
        let corners = data
            .lines()
            .map(|line| line.split(",").map(|coord| coord.parse().unwrap()))
            .map(|x| x.collect_tuple())
            .map(Option::unwrap)
            .collect();
        Self { corners }
    }

    fn rectangle_corners(&self) -> Vec<((usize, usize), (usize, usize))> {
        self.corners
            .iter()
            .flat_map(|x| repeat(x).zip(self.corners.iter()))
            .filter(|(a, b)| a != b)
            .filter(|((ax, ay), (bx, by))| ax < bx)
            .map(|(a, b)| (a.clone(), b.clone()))
            .collect()
    }
}

fn area(((ax, ay), (bx, by)): &((usize, usize), (usize, usize))) -> usize {
    (ax.abs_diff(*bx) + 1) * (ay.abs_diff(*by) + 1)
}

fn part1(board: &Board) -> usize {
    board.rectangle_corners().iter().map(area).max().unwrap()
}
fn build_rectangle(((ax, ay), (bx, by)): &((usize, usize), (usize, usize))) -> LineString<f64> {
    line_string![
        (x: *ax as f64, y: *ay as f64),
        (x: *bx as f64, y: *ay as f64),
        (x: *bx as f64, y: *by as f64),
        (x: *ax as f64, y: *by as f64),
    ]
}

fn part2(board: &Board) -> usize {
    let areas_desc: Vec<(((usize, usize), (usize, usize)), usize)> = board
        .rectangle_corners()
        .iter()
        .map(|&corner| (corner, area(&corner)))
        .sorted_by_key(|(_, area)| area.clone())
        .rev()
        .collect();
    let closed_loop: Vec<(f64, f64)> = [&board.corners[..], &vec![board.corners[0]]]
        .concat()
        .iter()
        .map(|&(x, y)| (x as f64, y as f64))
        .collect();
    let polygon = Polygon::new(closed_loop.into(), vec![]);
    for (corner, current_area) in areas_desc {
        let rectangle = Polygon::new(build_rectangle(&corner), vec![]);
        if polygon.contains(&rectangle) {
            return current_area;
        }
    }
    0
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("9.txt").unwrap();
    let board = Board::new(&contents);
    println!("{}", part1(&board));
    println!("{}", part2(&board));
}
