use std::collections::HashMap;

fn main() {
    let input = include_str!("../day7.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

type Card = (Vec<u8>, u32);

fn card_types(input: &str) -> Vec<Vec<Card>> {
    let mut types = vec![vec![]; 7];
    input.lines().for_each(|line| {
        let (cards, bid_amount) = line.split_once(' ').unwrap();
        let bid = bid_amount.parse::<u32>().unwrap();
        let mut labels = HashMap::new();
        let letters = cards.as_bytes().to_vec();
        letters.iter().for_each(|c| {
            if let Some((_, v)) = labels.get_key_value(&c) {
                labels.insert(c, v + 1);
            } else {
                labels.insert(c, 1u32);
            }
        });
        let mut kinds = labels.into_values().collect::<Vec<u32>>();
        kinds.sort();
        kinds.reverse();
        let first = kinds[0];
        match first {
            5 => types[6].push((letters, bid)),
            4 => types[5].push((letters, bid)),
            1 => types[0].push((letters, bid)),
            _ => {
                let second = kinds[1];
                match (first, second) {
                    (3, 2) => types[4].push((letters, bid)),
                    (3, 1) => types[3].push((letters, bid)),
                    (2, 2) => types[2].push((letters, bid)),
                    (2, 1) => types[1].push((letters, bid)),
                    _ => panic!("2 value match fucked up"),
                };
            }
        };
    });
    types
}

fn sort_carts(cards: Vec<Card>, level: usize) -> Vec<u32> {
    if level == 5 {
        return cards.into_iter().map(|(_, bid)| bid).collect();
    }
    let mut types = vec![vec![]; 13];
    cards.into_iter().for_each(|(card, bid)| {
        let label = card[level];
        match label {
            b'2' => types[0].push((card, bid)),
            b'3' => types[1].push((card, bid)),
            b'4' => types[2].push((card, bid)),
            b'5' => types[3].push((card, bid)),
            b'6' => types[4].push((card, bid)),
            b'7' => types[5].push((card, bid)),
            b'8' => types[6].push((card, bid)),
            b'9' => types[7].push((card, bid)),
            b'T' => types[8].push((card, bid)),
            b'J' => types[9].push((card, bid)),
            b'Q' => types[10].push((card, bid)),
            b'K' => types[11].push((card, bid)),
            b'A' => types[12].push((card, bid)),
            c => panic!("wrong char: {c}"),
        };
    });
    types
        .into_iter()
        .flat_map(|cards| sort_carts(cards, level + 1))
        .collect()
}

fn part1(input: &str) -> String {
    let types = card_types(input);
    types
        .into_iter()
        .flat_map(|cards| sort_carts(cards, 0))
        .enumerate()
        .map(|(i, bid)| bid * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

fn card_types2(input: &str) -> Vec<Vec<Card>> {
    let mut types = vec![vec![]; 7];
    input.lines().for_each(|line| {
        let (cards, bid_amount) = line.split_once(' ').unwrap();
        let bid = bid_amount.parse::<u32>().unwrap();
        let mut labels = HashMap::new();
        let letters = cards.as_bytes().to_vec();
        let mut joker = 0u32;
        letters.iter().for_each(|c| {
            if *c == b'J' {
                joker += 1;
            } else if let Some((_, v)) = labels.get_key_value(&c) {
                labels.insert(c, v + 1);
            } else {
                labels.insert(c, 1u32);
            }
        });
        let mut kinds = labels.into_values().collect::<Vec<u32>>();
        kinds.sort();
        kinds.reverse();
        let first = if let Some(n) = kinds.first() {
            n + joker
        } else {
            joker
        };
        match first {
            5 => types[6].push((letters, bid)),
            4 => types[5].push((letters, bid)),
            1 => types[0].push((letters, bid)),
            _ => {
                let second = kinds[1];
                match (first, second) {
                    (3, 2) => types[4].push((letters, bid)),
                    (3, 1) => types[3].push((letters, bid)),
                    (2, 2) => types[2].push((letters, bid)),
                    (2, 1) => types[1].push((letters, bid)),
                    _ => panic!("2 value match fucked up"),
                };
            }
        };
    });
    types
}

fn sort_carts2(cards: Vec<Card>, level: usize) -> Vec<u32> {
    if level == 5 {
        return cards.into_iter().map(|(_, bid)| bid).collect();
    }
    let mut types = vec![vec![]; 13];
    cards.into_iter().for_each(|(card, bid)| {
        let label = card[level];
        match label {
            b'J' => types[0].push((card, bid)),
            b'2' => types[1].push((card, bid)),
            b'3' => types[2].push((card, bid)),
            b'4' => types[3].push((card, bid)),
            b'5' => types[4].push((card, bid)),
            b'6' => types[5].push((card, bid)),
            b'7' => types[6].push((card, bid)),
            b'8' => types[7].push((card, bid)),
            b'9' => types[8].push((card, bid)),
            b'T' => types[9].push((card, bid)),
            b'Q' => types[10].push((card, bid)),
            b'K' => types[11].push((card, bid)),
            b'A' => types[12].push((card, bid)),
            c => panic!("wrong char: {c}"),
        };
    });
    types
        .into_iter()
        .flat_map(|cards| sort_carts2(cards, level + 1))
        .collect()
}
fn part2(input: &str) -> String {
    let types = card_types2(input);
    types
        .into_iter()
        .flat_map(|cards| sort_carts2(cards, 0))
        .enumerate()
        .map(|(i, bid)| bid * (i as u32 + 1))
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "6440".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "5905".to_string())
    }
}
