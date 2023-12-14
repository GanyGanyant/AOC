use std::{cmp::Reverse, collections::VecDeque};

fn main() {
    let input = include_str!("../day14.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}
fn tilt<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut v = transpose(v);
    v.iter_mut().for_each(|l| l.reverse());
    v
}

fn part1(input: &str) -> String {
    let rocks = transpose(input.lines().map(|line| line.as_bytes().to_vec()).collect());
    let mut total = 0;
    let length = rocks[0].len();
    for row in rocks.into_iter() {
        let mut beam = 0;
        let mut count = 0;
        for (i, stone) in row.into_iter().enumerate() {
            match stone {
                b'#' => {
                    beam = i + 1;
                    count = 0;
                }
                b'O' => {
                    total += length - beam - count;
                    count += 1;
                }
                _ => (),
            }
        }
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let mut rocks = transpose(input.lines().map(|line| line.as_bytes().to_vec()).collect());
    let mut cache = VecDeque::new();
    let cycles = 1_000_000_000;
    let mut n = 0;
    while n < cycles {
        for roll in 0..4 {
            rocks = transpose(rocks);
            for row in rocks.iter_mut() {
                for group in row.split_mut(|n| *n == b'#') {
                    match roll {
                        0 | 1 => group.sort_unstable_by_key(|x| Reverse(*x)),
                        2 | 3 => group.sort_unstable(),
                        _ => panic!(),
                    }
                }
            }
        }
        if let Some(idx) = cache.iter().position(|p| *p == rocks) {
            let repeat_length = cache.len() - idx;
            n = cycles - ((cycles - n) % repeat_length);
        }
        cache.push_back(rocks.clone());
        if cache.len() > 50 {
            cache.pop_front();
        }
        n += 1;
    }
    let mut total = 0;
    let length = rocks[0].len();
    for row in rocks.into_iter() {
        let mut beam = 0;
        let mut count = 0;
        for (i, stone) in row.into_iter().enumerate() {
            match stone {
                b'#' => {
                    beam = i + 1;
                    count = 0;
                }
                b'O' => {
                    total += length - beam - count;
                    count += 1;
                }
                _ => (),
            }
        }
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "136".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "64".to_string())
    }
}
