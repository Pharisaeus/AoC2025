use itertools::Itertools;
use std::collections::HashMap;
use std::fs;
use std::iter::repeat;

#[derive(Eq, PartialEq, Clone, Copy, Hash)]
struct Coord3d {
    x: usize,
    y: usize,
    z: usize,
}
impl Coord3d {
    fn euclid_distance(&self, other: &Coord3d) -> f64 {
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f64)
            .sqrt()
    }
}

struct Boxes {
    boxes: Vec<Coord3d>,
    distances: Vec<(f64, Coord3d, Coord3d)>,
}

impl Boxes {
    fn new(data: &str) -> Self {
        let boxes: Vec<Coord3d> = data
            .lines()
            .map(|line| {
                line.split(",")
                    .map(|x| x.parse().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .map(|(x, y, z)| Coord3d { x, y, z })
            .collect();
        let distances: Vec<(f64, Coord3d, Coord3d)> = boxes
            .iter()
            .flat_map(|x| repeat(x).zip(boxes.iter()))
            .filter(|(a, b)| a != b)
            .filter(|(a, b)| a.x < b.x)
            .map(|(a, b)| (a.euclid_distance(b), a.clone(), b.clone()))
            .sorted_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap())
            .collect();
        Boxes { boxes, distances }
    }

    fn connect(&self, limit: usize) -> (HashMap<Coord3d, Vec<Coord3d>>, (Coord3d, Coord3d)) {
        let mut representatives: HashMap<Coord3d, Coord3d> =
            self.boxes.iter().map(|b| (b.clone(), b.clone())).collect();
        let mut groups: HashMap<Coord3d, Vec<Coord3d>> = self
            .boxes
            .iter()
            .map(|b| (b.clone(), vec![b.clone()]))
            .collect();
        let mut lastb1 = None;
        let mut lastb2 = None;
        for (index, (_, box1, box2)) in self.distances.iter().enumerate() {
            if index >= limit || groups.len() == 1 {
                break;
            }
            let rep1 = Self::resolve_rep(box1, &mut representatives);
            let rep2 = Self::resolve_rep(box2, &mut representatives);
            let group1 = &groups[&rep1];
            let group2 = &groups[&rep2];
            if rep1 != rep2 {
                let concatenated = [&group1[..], &group2[..]].concat();
                groups.insert(rep1.clone(), concatenated);
                groups.remove(&rep2);
                representatives.insert(box1.clone(), rep1.clone());
                representatives.insert(box2.clone(), rep1.clone());
                representatives.insert(rep2.clone(), rep1.clone());
            }
            lastb1 = Some(box1);
            lastb2 = Some(box2);
        }
        (groups, (lastb1.unwrap().clone(), lastb2.unwrap().clone()))
    }

    fn resolve_rep(b: &Coord3d, representatives: &mut HashMap<Coord3d, Coord3d>) -> Coord3d {
        let mut rep = representatives[b];
        while rep != representatives[&rep] {
            rep = representatives[&rep];
        }
        representatives.insert(b.clone(), rep);
        rep
    }
}

fn part1(boxes: &Boxes) -> usize {
    boxes
        .connect(1000)
        .0
        .values()
        .map(|x| x.len())
        .sorted()
        .rev()
        .take(3)
        .product()
}
fn part2(boxes: &Boxes) -> usize {
    let (_, (b1, b2)) = boxes.connect(usize::MAX);
    b1.x * b2.x
}
pub(crate) fn solve() {
    let contents = fs::read_to_string("8.txt").unwrap();
    let boxes = Boxes::new(&contents);
    println!("{}", part1(&boxes));
    println!("{}", part2(&boxes));
}
