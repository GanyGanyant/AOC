fn main() {
    let input = include_str!("../day9.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .flat_map(|mut seq| {
            let len = seq.len();
            let mut depth = 1;
            for _ in 0..len - 1 {
                for i in 0..len - depth {
                    seq[i] = seq[i + 1] - seq[i];
                }
                depth += 1;
            }
            seq
        })
        .sum::<i32>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .rev()
                .collect::<Vec<i32>>()
        })
        .flat_map(|mut seq| {
            let len = seq.len();
            let mut depth = 1;
            for _ in 0..len - 1 {
                for i in 0..len - depth {
                    seq[i] = seq[i + 1] - seq[i];
                }
                depth += 1;
            }
            seq
        })
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "114".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "2".to_string())
    }
}
