use std::collections::HashMap;

fn main() {
    let input = include_str!("../day8.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Clone, Copy)]
enum Dir {
    Left,
    Right,
}

#[derive(Clone)]
struct Path {
    dirs: Box<[Dir]>,
    index: usize,
    len: usize,
}

impl Path {
    fn next(&mut self) -> Dir {
        if self.index == self.len {
            self.index = 0;
        }
        let val = self.dirs[self.index];
        self.index += 1;
        val
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let dirs = value
            .bytes()
            .map(|c| match c {
                b'R' => Dir::Right,
                b'L' => Dir::Left,
                c => panic!("Invalid value: {c}"),
            })
            .collect::<Box<[Dir]>>();
        let len = dirs.len();
        Self {
            dirs,
            index: 0,
            len,
        }
    }
}

struct Network<'a> {
    nodes: HashMap<&'a str, (&'a str, &'a str)>,
}

impl<'a> Network<'a> {
    fn walk_part1(&self, mut path: Path) -> u32 {
        let mut steps = 0;
        let mut node = "AAA";
        loop {
            if node == "ZZZ" {
                break;
            }
            let (left, right) = self.nodes.get(node).unwrap();
            match path.next() {
                Dir::Left => node = left,
                Dir::Right => node = right,
            }
            steps += 1;
        }
        steps
    }

    fn walk_part2(&self, path: Path) -> u64 {
        self.nodes
            .keys()
            .filter_map(|node| {
                if !node.ends_with('A') {
                    return None;
                }
                Some(*node)
            })
            .map(|mut node| {
                let mut steps = 0u64;
                let mut path = path.clone();
                loop {
                    if node.ends_with('Z') {
                        break;
                    }
                    let (left, right) = self.nodes.get(node).unwrap();
                    match path.next() {
                        Dir::Left => node = left,
                        Dir::Right => node = right,
                    }
                    steps += 1;
                }
                steps
            })
            .reduce(num::integer::lcm)
            .unwrap()
    }
}

impl<'a> From<&'a str> for Network<'a> {
    fn from(value: &'a str) -> Self {
        let mut nodes = HashMap::new();
        value.lines().for_each(|line| {
            let (key, values) = line.split_once(" = ").unwrap();
            let (left, right) = values
                .split_once(", ")
                .map(|(left, right)| {
                    (
                        left.strip_prefix('(').unwrap(),
                        right.strip_suffix(')').unwrap(),
                    )
                })
                .unwrap();
            nodes.insert(key, (left, right));
        });
        Self { nodes }
    }
}

fn part1(input: &str) -> String {
    let (dirs, network) = input.split_once("\n\n").unwrap();
    let path = Path::from(dirs);
    let network = Network::from(network);
    network.walk_part1(path).to_string()
}

fn part2(input: &str) -> String {
    let (dirs, network) = input.split_once("\n\n").unwrap();
    let path = Path::from(dirs);
    let network = Network::from(network);
    network.walk_part2(path).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    static INPUT2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "6".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "6".to_string())
    }
}
