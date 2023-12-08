use std::collections::HashMap;

use regex::Regex;

pub fn solve() {
    let input = include_str!("./input.txt");

    let sim = Simulation::from_str(input);

    dbg!(sim.run());
    dbg!(sim.run_part2());
}

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("no such dir"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    text: String,
    left: String,
    right: String,
}

#[derive(Debug, PartialEq)]
struct Simulation {
    graph: HashMap<String, Node>,
    instructions: Vec<Direction>,
}

impl Simulation {
    fn from_str(input: &str) -> Simulation {
        let mut lines = input.split("\n");
        let directions_str = lines.next().unwrap();
        let instructions = directions_str
            .chars()
            .map(Direction::from_char)
            .collect::<Vec<_>>();

        lines.next().unwrap();

        let line_regex = Regex::new(r"([0-9A-Z]+) = \(([0-9A-Z]+), ([0-9A-Z]+)\)").unwrap();

        let graph = lines
            .map(|l| {
                let parts = line_regex.captures(l).unwrap();
                let current = parts.get(1).unwrap().as_str();
                let left = parts.get(2).unwrap().as_str();
                let right = parts.get(3).unwrap().as_str();

                (
                    current.to_string(),
                    Node {
                        text: current.into(),
                        left: left.into(),
                        right: right.into(),
                    },
                )
            })
            .collect();

        Simulation {
            graph,
            instructions,
        }
    }

    fn run(&self) -> u64 {
        let mut cur = self.graph.get("AAA".into()).unwrap();
        let mut instruction_i = 0;

        while cur.text != "ZZZ".to_string() {
            cur = self.get_next(instruction_i, cur);

            instruction_i += 1;
        }

        instruction_i as u64
    }

    fn get_next(&self, instruction_i: usize, from_node: &Node) -> &Node {
        let d = self
            .instructions
            .get(instruction_i % self.instructions.len())
            .unwrap();

        let cur = match d {
            Direction::Right => self.graph.get(&from_node.right).unwrap(),
            Direction::Left => self.graph.get(&from_node.left).unwrap(),
        };

        cur
    }

    // returns (number of iterations until reached z, z node)
    fn run_from(&self, instruction_i: usize, from: &str) -> (usize, &Node) {
        let mut cur = self.graph.get(from).unwrap();
        let mut delta = 0;

        while !cur.text.ends_with("Z") {
            cur = self.get_next(instruction_i + delta, &cur);

            delta += 1;
        }

        (delta, cur)
    }

    fn run_part2(&self) -> u64 {
        let start_nodes = self
            .graph
            .iter()
            .filter(|(k, _)| k.ends_with("A"))
            .map(|(_, v)| v)
            .collect::<Vec<_>>();

        // for each start node find number of steps to reach end node, and circle length from z_node to itelf
        // there must be a circle since we have limited nodes in the graph and everything connects to something
        let data = start_nodes
            .iter()
            .map(|c| {
                let (n_steps, z_node) = self.run_from(0, &c.text);
                let next = self.get_next(n_steps, &z_node);
                let (n_circle, _) = self.run_from(n_steps + 1, &next.text);

                (n_steps, n_circle + 1)
            })
            .collect::<Vec<_>>();

        let max_n_steps = data.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

        let max_steps = max_n_steps.0;
        let circle_factor = max_n_steps.1;

        let mut instruction_i = max_steps;

        // start from the start node that has max steps to z. increase the index by it's cirlcle length
        // until we find a number that matches all other start nodes
        while !data.iter().all(|d| (instruction_i - d.0) % d.1 == 0) {
            instruction_i += circle_factor;
        }

        instruction_i as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simulation_from_str_works() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let expected = Simulation {
            instructions: vec![Direction::Left, Direction::Left, Direction::Right],
            graph: HashMap::from([
                (
                    "AAA".into(),
                    Node {
                        text: "AAA".into(),
                        left: "BBB".into(),
                        right: "BBB".into(),
                    },
                ),
                (
                    "BBB".into(),
                    Node {
                        text: "BBB".into(),
                        left: "AAA".into(),
                        right: "ZZZ".into(),
                    },
                ),
                (
                    "ZZZ".into(),
                    Node {
                        text: "ZZZ".into(),
                        left: "ZZZ".into(),
                        right: "ZZZ".into(),
                    },
                ),
            ]),
        };

        assert_eq!(Simulation::from_str(input), expected);
    }

    #[test]
    fn simulation_run() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let sim = Simulation::from_str(input);

        assert_eq!(sim.run(), 6)
    }

    #[test]
    fn simulation_run2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let sim = Simulation::from_str(input);

        assert_eq!(sim.run_part2(), 6)
    }
}
