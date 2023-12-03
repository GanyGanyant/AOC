use std::cmp::{max, min};

fn main() {
    let input = include_str!("../day3.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}
#[derive(Debug)]
struct Part {
    index: (usize, usize),
    len: usize,
    value: u32,
}

enum Parter {
    Start(usize),
    None,
}

impl Parter {
    fn eval(
        &mut self,
        row: usize,
        end: usize,
        evaluator: impl Fn(Part) -> Option<Part>,
    ) -> Result<Option<Part>, &str> {
        let ans;
        match self {
            Parter::Start(i) => {
                ans = Ok(evaluator(Part {
                    index: (row, *i),
                    len: end - *i,
                    value: 0,
                }))
            }
            Parter::None => ans = Err("No part to eval"),
        };
        *self = Self::None;
        ans
    }
}

fn part1(input: &str) -> String {
    let engine = input.lines().collect::<Vec<_>>();
    let mut parts = Vec::new();
    let len = engine[0].len();
    let size = engine.len();
    let e = |mut part: Part| {
        let top = max(0, part.index.0 as isize - 1) as usize;
        let bottom = min(size - 1, part.index.0 + 1);
        let left = max(0, part.index.1 as isize - 1) as usize;
        let right = min(len - 1, part.index.1 + part.len);
        let value = engine[part.index.0][part.index.1..part.index.1 + part.len]
            .to_string()
            .parse::<u32>()
            .unwrap();

        let mut label = false;
        engine[top..=bottom].iter().for_each(|line| {
            line[left..=right].chars().for_each(|c| {
                if !c.is_ascii_digit() && c != '.' {
                    label = true;
                }
            });
        });
        if !label {
            return None;
        }
        part.value = value;
        Some(part)
    };
    for (i, line) in engine.iter().enumerate() {
        let mut parter = Parter::None;
        for (j, c) in line.char_indices() {
            match parter {
                Parter::Start(_) => {
                    if !c.is_ascii_digit() {
                        if let Ok(Some(part)) = parter.eval(i, j, e) {
                            parts.push(part);
                        };
                    }
                }
                Parter::None => {
                    if c.is_ascii_digit() {
                        parter = Parter::Start(j);
                    }
                }
            }
        }
        if let Ok(Some(part)) = parter.eval(i, len, e) {
            parts.push(part);
        };
    }
    parts
        .into_iter()
        .fold(0, |acc, part| acc + part.value)
        .to_string()
}

struct Gear(usize, usize);

fn part2(input: &str) -> String {
    let engine = input.lines().collect::<Vec<_>>();
    let len = engine[0].len();
    let size = engine.len();

    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.chars().enumerate().filter_map(move |(j, c)| {
                if c == '*' {
                    return Some(Gear(i, j));
                }
                None
            })
        })
        .filter_map(|Gear(i, j)| {
            let top = max(0, i as isize - 1) as usize;
            let bottom = min(size - 1, i + 1);

            let left = max(0, j as isize - 1) as usize;
            let right = min(len - 1, j + 1);

            let mut nums: Vec<_> = Vec::new();

            // functions
            let find_start = |row: usize, index: usize| {
                let mut start = index;
                while let Some(digit) = engine[row].chars().nth(start - 1) {
                    if digit.is_ascii_digit() {
                        start -= 1;
                    } else {
                        break;
                    }
                }
                start
            };
            let find_end = |row: usize, index: usize| {
                let mut end = index;
                while let Some(digit) = engine[row].chars().nth(end + 1) {
                    if digit.is_ascii_digit() {
                        end += 1;
                    } else {
                        break;
                    }
                }
                end
            };

            let find = |row: usize, index: usize| find_start(row, index)..=find_end(row, index);
            // end of functions

            for index in [top, i, bottom] {
                if engine[index].chars().nth(j).unwrap().is_ascii_digit() {
                    nums.push(
                        engine[index]
                            .get(find(index, j))
                            .unwrap()
                            .parse::<u32>()
                            .ok(),
                    );
                } else {
                    nums.push(
                        engine[index]
                            .get(find_start(index, left)..=left)
                            .unwrap_or("")
                            .parse::<u32>()
                            .ok(),
                    );
                    nums.push(
                        engine[index]
                            .get(right..=find_end(index, right))
                            .unwrap_or("")
                            .parse::<u32>()
                            .ok(),
                    );
                }
            }
            let numbers = nums.into_iter().flatten().collect::<Vec<u32>>();
            if numbers.len() == 2 {
                return Some(numbers[0] * numbers[1]);
            }
            None
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "4361".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "467835".to_string())
    }
}
