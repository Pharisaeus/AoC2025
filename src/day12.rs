use itertools::Itertools;
use std::fs;
struct Present {
    size: usize,
}

impl Present {
    fn new(data: &str) -> Present {
        Present {
            size: data.chars().filter(|c| *c == '#').count(),
        }
    }
}

struct Region {
    x: usize,
    y: usize,
    presents: Vec<usize>,
}

impl Region {
    fn new(line: &str) -> Region {
        let (dim, selected) = line.split(": ").collect_tuple().unwrap();
        let (x, y) = dim
            .split("x")
            .map(|value| value.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        let presents = selected
            .split(" ")
            .map(|value| value.parse::<usize>().unwrap())
            .collect();
        Region { x, y, presents }
    }

    fn area(&self) -> usize {
        (self.x - self.x % 3) * (self.y - self.y % 3) // only part that fits whole blocks!
    }

    fn can_cover(&self, presents: &Vec<Present>) -> bool {
        let minimum = self
            .presents
            .iter()
            .enumerate()
            .map(|(index, count)| &presents[index].size * count)
            .sum::<usize>();
        let maximum = self.presents.iter().sum::<usize>() * (3 * 3);
        let available = self.area();
        if available >= minimum && available < maximum {
            //we would actually need to do some "packing" :(
            panic!("{} {} {}", minimum, maximum, available)
        }
        available >= maximum
    }
}

struct Board {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

impl Board {
    fn new(data: &str) -> Board {
        let presents = data.split("\n\n").map(|data| Present::new(data)).collect();
        let regions = data
            .split("\n\n")
            .last()
            .unwrap()
            .split("\n")
            .map(|line| Region::new(line))
            .collect();
        Board { presents, regions }
    }

    fn count_valid(&self) -> usize {
        self.regions
            .iter()
            .filter(|region| region.can_cover(&self.presents))
            .count()
    }
}

fn part1(board: &Board) -> usize {
    board.count_valid()
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("12.txt").unwrap();
    let board = Board::new(&contents);
    println!("{}", part1(&board));
}
