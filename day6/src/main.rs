fn main() {
    let input = include_str!("../day6.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut lines = input
        .lines()
        .map(|line| {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();
    let (distance, time) = (lines.pop().unwrap(), lines.pop().unwrap());
    calculate(distance, time)
}

fn part2(input: &str) -> String {
    let mut lines = input
        .lines()
        .map(|line| {
            let mut ans = "".to_string();
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .for_each(|num| ans.push_str(num));
            vec![ans.parse::<u64>().unwrap()]
        })
        .collect::<Vec<Vec<u64>>>();
    let (distance, time) = (lines.pop().unwrap(), lines.pop().unwrap());
    calculate(distance, time)
}

fn calculate(distance: Vec<u64>, time: Vec<u64>) -> String {
    distance
        .into_iter()
        .zip(time)
        .map(|(d, t)| {
            let discriminant = f64::sqrt(t as f64 * t as f64 - 4.0 * d as f64);
            let mut left = t as f64 / 2.0 - discriminant / 2.0;
            let mut right = t as f64 / 2.0 + discriminant / 2.0;
            left += 0.0001;
            right -= 0.0001;
            left = left.floor();
            right = right.floor();
            right as u64 - left as u64
        })
        .product::<u64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "Time:      7  15   30
Distance:  9  40  200";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "288".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "71503".to_string())
    }
}
