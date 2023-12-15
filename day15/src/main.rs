use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_str!("../day15.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    input
        .trim()
        .split(',')
        .map(|val| hash(val.as_bytes()))
        .sum::<usize>()
        .to_string()
}

fn hash(word: &[u8]) -> usize {
    word.iter()
        .fold(0, |acc, &x| ((acc + usize::from(x)) * 17) % 256)
}

fn part2(input: &str) -> String {
    let mut hash_map: HashMap<usize, Vec<(&str, usize)>> = HashMap::new();
    'step: for step in input.trim().split(',') {
        if step.contains('-') {
            let label = step.split_once('-').unwrap().0;
            if let Some(lenses) = hash_map.get_mut(&hash(label.as_bytes())) {
                lenses.retain(|(key, _)| *key != label);
            }
        } else {
            let (label, lens) = step.split_once('=').unwrap();
            let lens = lens.parse::<usize>().unwrap();
            let id = hash(label.as_bytes());
            if let Some(lenses) = hash_map.get_mut(&id) {
                for (name, size) in lenses.iter_mut() {
                    if label == *name {
                        *size = lens;
                        continue 'step;
                    }
                }
                lenses.push((label, lens));
            } else {
                hash_map.insert(id, vec![(label, lens)]);
            }
        }
    }
    hash_map
        .into_values()
        .flat_map(|lenses| {
            lenses
                .into_iter()
                .enumerate()
                .map(|(i, (key, lens))| (hash(key.as_bytes()) + 1) * (i + 1) * lens)
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "1320".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "145".to_string())
    }
}
