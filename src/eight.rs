use std::collections::HashMap;

fn part_one(input: &str) -> u32 {
    let (instructions, nodes) = parse_input(input);

    let start_key = NodeKey("AAA".to_string());
    let end_key = NodeKey("ZZZ".to_string());

    let mut current_node = nodes.get(&start_key).unwrap();

    let mut steps = 0;
    for instruction in instructions.into_iter() {
        steps += 1;
        let next_key = current_node.get_direction_key(&instruction);

        if next_key == &end_key {
            return steps;
        }

        current_node = nodes.get(next_key).unwrap();
    }

    steps
}

fn part_two(input: &str) -> u32 {
    let (instructions, nodes) = parse_input(input);

    let mut current_nodes: Vec<&Node> = nodes.values().filter(|v| v.key.0.ends_with('A')).collect();

    let mut steps = 0;
    for instruction in instructions.into_iter() {
        steps += 1;

        // Build next nodes
        let next_nodes = current_nodes.iter().map(|node| {
            let next_key = node.get_direction_key(&instruction);
            nodes.get(next_key).unwrap()
        });

        // Check if all nodes are done
        let count = next_nodes
            .clone()
            .filter(|n| !n.key.0.ends_with('Z'))
            .count();
        if count == 0 {
            return steps;
        }

        current_nodes = next_nodes.collect();
    }

    steps
}
fn parse_input(input: &str) -> (Instructions, Nodes) {
    let mut lines = input.lines();

    let instructions = Instructions::from(lines.next().unwrap());

    // Skip empty line
    lines.next();

    let mut nodes = Nodes::new();
    for line in lines {
        // Remove unneeded characters
        let line = line.replace([' ', '(', ')'], "");

        let mut split = line.split('=');

        let key = split.next().unwrap();
        let key = NodeKey(key.to_string());

        let node = split.next().unwrap();
        let mut node = node.split(',');

        let node = Node {
            key: key.clone(),
            left: NodeKey(node.next().unwrap().to_string()),
            right: NodeKey(node.next().unwrap().to_string()),
        };

        nodes.insert(key, node);
    }

    (instructions, nodes)
}

type Nodes = HashMap<NodeKey, Node>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct NodeKey(String);

#[derive(Debug, Clone)]
struct Node {
    key: NodeKey,
    left: NodeKey,
    right: NodeKey,
}

impl Node {
    fn get_direction_key(&self, instruction: &Instruction) -> &NodeKey {
        match instruction {
            Instruction::Left => &self.left,
            Instruction::Right => &self.right,
        }
    }
}

#[derive(Debug, Clone)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instructions {
    step: usize,
    instructions: Vec<Instruction>,
}

impl Iterator for Instructions {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let instruction = self.instructions.get(self.step).unwrap();

        // Increment step
        self.step += 1;
        if self.step >= self.instructions.len() {
            self.step = 0;
        }

        Some(instruction.clone())
    }
}

impl From<&str> for Instructions {
    fn from(value: &str) -> Self {
        let instructions = value
            .chars()
            .map(|c| match c {
                'R' => Instruction::Right,
                'L' => Instruction::Left,
                _ => unreachable!(),
            })
            .collect();

        Self {
            step: 0,
            instructions,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("../assets/eight.txt");

    const SAMPLE_INPUT: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE_INPUT_2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const SAMPLE_INPUT_3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_one() {
        let res = part_one(SAMPLE_INPUT);
        assert_eq!(res, 2);

        let res = part_one(SAMPLE_INPUT_2);
        assert_eq!(res, 6);
    }

    #[test]
    fn solution_one() {
        let res = part_one(INPUT);
        dbg!(res);
    }

    #[test]
    fn test_two() {
        let res = part_two(SAMPLE_INPUT_3);
        assert_eq!(res, 6);
    }

    #[test]
    fn solution_two() {
        let res = part_two(INPUT);
        dbg!(res);
    }
}
