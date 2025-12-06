use std::fs;
use std::str::FromStr;

#[derive(Clone)]
enum Operation {
    ADD,
    MUL,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (s) {
            "+" => Ok(Operation::ADD),
            "*" => Ok(Operation::MUL),
            _ => Err(()),
        }
    }
}
struct Equation {
    numbers: Vec<usize>,
    operation: Operation,
}

impl Equation {
    fn calculate(&self) -> usize {
        match (self.operation) {
            Operation::ADD => self.numbers.iter().sum(),
            Operation::MUL => self.numbers.iter().product(),
        }
    }
}
struct Equations {
    equations: Vec<Equation>,
}

impl Equations {
    fn new(operations: &Vec<Operation>, numbers: &Vec<Vec<usize>>) -> Self {
        let equations = operations
            .iter()
            .zip(numbers)
            .map(|(operation, numbers)| Equation {
                operation: operation.clone(),
                numbers: numbers.clone(),
            })
            .collect();
        Equations { equations }
    }
}

impl Equations {
    fn parse(data: &str) -> Equations {
        let operations = Self::parse_operations(data);
        let numbers = Self::parse_row_numbers(data);
        Equations::new(&operations, &numbers)
    }

    fn parse_columns(data: &str) -> Equations {
        let operations = Self::parse_operations(data);
        let numbers = Self::parse_column_numbers(data);
        Equations::new(&operations, &numbers)
    }

    fn parse_row_numbers(data: &str) -> Vec<Vec<usize>> {
        let row_numbers: Vec<Vec<usize>> = data
            .lines()
            .rev()
            .skip(1)
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();
        Self::transpose(row_numbers)
    }

    fn transpose(row_numbers: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let num_cols = row_numbers.first().unwrap().len();
        let mut row_iters: Vec<_> = row_numbers.into_iter().map(Vec::into_iter).collect();
        (0..num_cols)
            .map(|_| row_iters.iter_mut().map(|it| it.next().unwrap()).collect())
            .collect()
    }

    fn parse_column_numbers(data: &str) -> Vec<Vec<usize>> {
        let digits: Vec<Vec<char>> = data
            .lines()
            .rev()
            .skip(1)
            .map(|line| line.chars().collect())
            .collect();
        let mut number_sets = vec![];
        let mut current_set = vec![];
        for column_index in 0..digits[0].len() {
            let parsed = Self::parse_one_column_number(column_index, &digits);
            if parsed.is_some() {
                current_set.push(parsed.unwrap());
            } else {
                number_sets.push(current_set);
                current_set = vec![];
            }
        }
        number_sets.push(current_set);
        number_sets
    }

    fn parse_one_column_number(column: usize, digits: &Vec<Vec<char>>) -> Option<usize> {
        let reassembled = digits
            .iter()
            .map(|line| line.get(column).unwrap_or(&' '))
            .rev()
            .collect::<String>();
        if reassembled.trim().is_empty() {
            None
        } else {
            Some(reassembled.trim().parse().unwrap())
        }
    }

    fn parse_operations(data: &str) -> Vec<Operation> {
        data.lines()
            .last()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|x| Operation::from_str(x).unwrap())
                    .collect()
            })
            .unwrap()
    }
    fn total_sum(&self) -> usize {
        self.equations.iter().map(Equation::calculate).sum()
    }
}

fn part1(equations: &Equations) -> usize {
    equations.total_sum()
}

fn part2(equations: &Equations) -> usize {
    equations.total_sum()
}
pub(crate) fn solve() {
    let contents = fs::read_to_string("6.txt").unwrap();
    println!("{}", part1(&Equations::parse(&contents)));
    println!("{}", part2(&Equations::parse_columns(&contents)));
}
