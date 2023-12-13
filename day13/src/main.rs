fn main() {
    let input = include_str!("../day13.txt");
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

fn part1(input: &str) -> String {
    let images = input
        .split("\n\n")
        .map(|images| {
            images
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    'next: for image in images {
        'down: for i in 1..image.len() {
            let mut depth = 1;
            if image[i] == image[i - 1] {
                while let (Some(top), Some(bot)) = (image.get(i - depth - 1), image.get(i + depth))
                {
                    if top != bot {
                        continue 'down;
                    }
                    depth += 1;
                }
                total += i * 100;
                continue 'next;
            }
        }
        let image = transpose(image);
        'down: for i in 1..image.len() {
            let mut depth = 1;
            if image[i] == image[i - 1] {
                while let (Some(top), Some(bot)) = (image.get(i - depth - 1), image.get(i + depth))
                {
                    if top != bot {
                        continue 'down;
                    }
                    depth += 1;
                }
                total += i;
                continue 'next;
            }
        }
    }
    total.to_string()
}

fn part2(input: &str) -> String {
    let images = input
        .split("\n\n")
        .map(|images| {
            images
                .lines()
                .map(|line| line.as_bytes().to_vec())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut total = 0;
    'next: for image in images {
        'down: for i in 1..image.len() {
            let mut depth = 1;
            let mut smudge = false;
            for (a, b) in image[i].iter().zip(image[i - 1].iter()) {
                if a != b {
                    if smudge {
                        continue 'down;
                    }
                    smudge = true;
                }
            }
            while let (Some(top), Some(bot)) = (image.get(i - depth - 1), image.get(i + depth)) {
                for (a, b) in top.iter().zip(bot.iter()) {
                    if a != b {
                        if smudge {
                            continue 'down;
                        }
                        smudge = true;
                    }
                }
                depth += 1;
            }
            if smudge {
                total += i * 100;
                continue 'next;
            }
        }
        let image = transpose(image);
        'down: for i in 1..image.len() {
            let mut depth = 1;
            let mut smudge = false;
            for (a, b) in image[i].iter().zip(image[i - 1].iter()) {
                if a != b {
                    if smudge {
                        continue 'down;
                    }
                    smudge = true;
                }
            }
            while let (Some(top), Some(bot)) = (image.get(i - depth - 1), image.get(i + depth)) {
                for (a, b) in top.iter().zip(bot.iter()) {
                    if a != b {
                        if smudge {
                            continue 'down;
                        }
                        smudge = true;
                    }
                }
                depth += 1;
            }
            if smudge {
                total += i;
                continue 'next;
            }
        }
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "405".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "400".to_string())
    }
}
