fn main() {
    let input = include_str!("../day4.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (wins, list) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
            let nums = list
                .split_whitespace()
                .filter_map(|n| n.parse::<u32>().ok())
                .collect::<Vec<_>>();
            let mut points = 0;
            wins.split_whitespace().for_each(|n| {
                if let Ok(num) = n.parse::<u32>() {
                    if nums.contains(&num) {
                        points += 1;
                    }
                }
            });
            if points == 0 {
                return None;
            }
            Some(2u32.pow(points - 1))
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let cards = input.lines().collect::<Vec<_>>();
    let size = cards.len();
    let mut wins = vec![0usize; size];
    cards.into_iter().enumerate().for_each(|(index, card)| {
        let (win, list) = card.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let nums = list
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<Vec<_>>();
        let mut points = 0;
        win.split_whitespace().for_each(|n| {
            if let Ok(num) = n.parse::<u32>() {
                if nums.contains(&num) {
                    points += 1;
                }
            }
        });
        wins[index] = points;
    });
    let mut sum = 0u32;
    let mut calc = |index: usize| {
        fn calculate(sum: &mut u32, wins: &Vec<usize>, index: usize) {
            if let Some(points) = wins.get(index) {
                *sum += 1;
                for i in 0..*points {
                    calculate(sum, wins, index + i + 1);
                }
            }
        }
        calculate(&mut sum, &wins, index)
    };
    for i in 0..size {
        calc(i);
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "13".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "30".to_string())
    }
}
