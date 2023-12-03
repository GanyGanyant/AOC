fn main() {
    let input = include_str!("../day1.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let mut nums = line.chars().filter(|c| c.is_numeric());
            match nums.clone().count() {
                0 => 0,
                1 => {
                    let n = nums.next().unwrap();
                    format!("{n}{n}").parse::<u32>().unwrap()
                }
                _ => format!("{}{}", nums.next().unwrap(), nums.last().unwrap())
                    .parse::<u32>()
                    .unwrap(),
            }
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (mut value, mut index) = (0u32, usize::MAX);
            let (mut rvalue, mut rindex) = (0u32, 0usize);
            for (name, val) in NUMBERS {
                if let Some(a) = line.find(name) {
                    if a < index {
                        value = val;
                        index = a;
                    }
                }
                if let Some(b) = line.rfind(name) {
                    if b >= rindex {
                        rvalue = val;
                        rindex = b;
                    }
                }
            }
            format!("{value}{rvalue}").parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .to_string()
}

const NUMBERS: [(&str, u32); 19] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    static INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "142".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "281".to_string())
    }
}
