use itertools::Itertools;
use std::cmp::{max, min};
use std::fs;
use std::str::FromStr;

#[derive(PartialEq)]
struct InclusiveRange {
    start: usize,
    stop: usize,
}

impl InclusiveRange {
    fn new(line: &str) -> Self {
        let (start, stop) = line
            .split("-")
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect_tuple()
            .unwrap();
        Self { start, stop }
    }

    fn contains(&self, item: &usize) -> bool {
        item >= &self.start && item <= &self.stop
    }

    fn overlaps(&self, other: &InclusiveRange) -> bool {
        self.contains(&other.start) || self.contains(&other.stop)
    }

    fn can_be_merged_with(&self, other: &InclusiveRange) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }

    fn combine(&self, other: &InclusiveRange) -> InclusiveRange {
        let start = min(other.start, self.start);
        let stop = max(other.stop, self.stop);
        Self { start, stop }
    }

    fn element_count(&self) -> usize {
        self.stop - self.start + 1
    }
}

struct FreshIngredients {
    fresh: Vec<InclusiveRange>,
}

impl FreshIngredients {
    fn new(ranges: &str) -> Self {
        let fresh = ranges.lines().map(InclusiveRange::new).collect();
        let compacted = FreshIngredients::compact(fresh);
        Self { fresh: compacted }
    }

    fn contains(&self, item: &usize) -> bool {
        self.fresh.iter().any(|r| r.contains(item))
    }

    fn compact(initial: Vec<InclusiveRange>) -> Vec<InclusiveRange> {
        let (mut compacted_ranges, mut compacted) = FreshIngredients::compact_once(&initial);
        while compacted {
            (compacted_ranges, compacted) = FreshIngredients::compact_once(&compacted_ranges);
        }
        compacted_ranges
    }

    fn compact_once(ranges: &Vec<InclusiveRange>) -> (Vec<InclusiveRange>, bool) {
        let (compacted, did_expand): (Vec<InclusiveRange>, Vec<bool>) = ranges
            .iter()
            .map(|item| FreshIngredients::expand_range(item, ranges))
            .unique_by(|(r, _)| (r.start, r.stop))
            .unzip();
        let something_compacted = did_expand.iter().any(|x| *x);
        (compacted, something_compacted)
    }

    fn expand_range(
        current: &InclusiveRange,
        others: &Vec<InclusiveRange>,
    ) -> (InclusiveRange, bool) {
        let mut acc = InclusiveRange {
            start: current.start,
            stop: current.stop,
        };
        let mut expanded = false;
        for other in others {
            if current != other && (acc.can_be_merged_with(other)) {
                acc = acc.combine(other);
                expanded = true;
            }
        }
        (acc, expanded)
    }

    fn count_all(&self) -> usize {
        self.fresh.iter().map(InclusiveRange::element_count).sum()
    }
}

struct IngredientDB {
    fresh: FreshIngredients,
    ingredients: Vec<usize>,
}

impl IngredientDB {
    fn new(data: &str) -> Self {
        let (ranges, items) = data.split_once("\n\n").unwrap();
        let fresh = FreshIngredients::new(ranges);
        let ingredients = items
            .lines()
            .map(usize::from_str)
            .map(Result::unwrap)
            .collect();
        Self { fresh, ingredients }
    }

    fn fresh_ingredients(&self) -> Vec<usize> {
        self.ingredients
            .iter()
            .filter(|item| self.fresh.contains(item))
            .map(usize::clone)
            .collect()
    }

    fn count_all_fresh(&self) -> usize {
        self.fresh.count_all()
    }
}

fn part1(db: &IngredientDB) -> usize {
    db.fresh_ingredients().len()
}

fn part2(db: &IngredientDB) -> usize {
    db.count_all_fresh()
}
pub(crate) fn solve() {
    let contents = fs::read_to_string("5.txt").unwrap();
    let db = IngredientDB::new(&contents);
    println!("{}", part1(&db));
    println!("{}", part2(&db));
}
