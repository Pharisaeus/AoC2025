use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

struct Graph {
    connections: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(data: &str) -> Self {
        let mut connections = HashMap::new();
        for line in data.lines() {
            let (node, connected) = line.split(": ").collect_tuple().unwrap();
            connections.insert(
                node.to_string(),
                connected.split(" ").map(str::to_string).collect(),
            );
        }

        Self { connections }
    }

    fn count_paths(&self, start: &str, end: &str) -> usize {
        let mut known = HashMap::new();
        self.rec_count_paths(&start, &end, &mut known)
    }

    fn rec_count_paths(
        &self,
        current: &str,
        end: &str,
        known: &mut HashMap<String, usize>,
    ) -> usize {
        if current == end {
            known.insert(current.to_string(), 1);
            1
        } else if known.contains_key(current) {
            known[current]
        } else {
            let result = self
                .connections
                .get(current)
                .unwrap_or(&vec![])
                .iter()
                .map(|node| self.rec_count_paths(&node, &end, known))
                .sum();
            known.insert(current.to_string(), result);
            result
        }
    }

    fn count_paths_dac_fft(&self, start: &str, end: &str) -> usize {
        let mut known_simple = HashMap::new();
        let mut known_complex = HashMap::new();
        let result = self.rec_count_paths_dac_fft(
            &start,
            &end,
            false,
            false,
            &mut known_simple,
            &mut known_complex,
        );
        result
    }

    fn rec_count_paths_dac_fft(
        &self,
        current: &str,
        end: &str,
        seen_dac: bool,
        seen_fft: bool,
        known_simple: &mut HashMap<String, usize>,
        known_complex: &mut HashMap<(String, bool, bool), usize>,
    ) -> usize {
        let key = &(current.to_string(), seen_dac, seen_fft);
        if known_complex.contains_key(key) {
            // we already know how many paths from current lead to a proper solution
            known_complex[key]
        } else if (seen_dac && seen_fft) {
            // all needed nodes seen, just count all the path till the end
            self.rec_count_paths(current, end, known_simple)
        } else if current == end {
            // end reached but not all nodes seen, bad path
            0
        } else {
            let result = self
                .connections
                .get(current)
                .unwrap_or(&vec![])
                .iter()
                .map(|node| {
                    self.rec_count_paths_dac_fft(
                        &node,
                        &end,
                        seen_dac || current == "dac",
                        seen_fft || current == "fft",
                        known_simple,
                        known_complex,
                    )
                })
                .sum();
            known_complex.insert((current.to_string(), seen_dac, seen_fft), result);
            result
        }
    }
}
fn part1(graph: &Graph) -> usize {
    graph.count_paths("you", "out")
}

fn part2(graph: &Graph) -> usize {
    graph.count_paths_dac_fft("svr", "out")
}

pub(crate) fn solve() {
    let contents = fs::read_to_string("11.txt").unwrap();
    let graph = Graph::new(&contents);
    println!("{}", part1(&graph));
    println!("{}", part2(&graph));
}
