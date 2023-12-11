fn main() {
    let input = include_str!("../day11.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let image = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Box<_>>();
    let mut lines = Vec::new();
    let mut columns = (0..image[0].len()).collect::<Vec<_>>();
    let galaxies = image
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            if !line.contains(&b'#') {
                lines.push(i);
                return vec![];
            }
            line.iter()
                .enumerate()
                .filter_map(|(j, c)| {
                    if *c == b'#' {
                        columns.retain(|col| *col != j);
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    // let mut gals = galaxies.iter();
    for (n, (i, j)) in galaxies.iter().enumerate() {
        for (x, y) in galaxies.iter().skip(n) {
            let left = std::cmp::min(y, j);
            let right = std::cmp::max(y, j);
            sum += x - i + right - left;
            for l in lines.iter() {
                if (*i..*x).contains(l) {
                    sum += 1;
                }
            }
            for col in columns.iter() {
                if (*left..*right).contains(col) {
                    sum += 1;
                }
            }
        }
    }
    sum.to_string()
}

fn part2(input: &str) -> String {
    let image = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Box<_>>();
    let mut lines = Vec::new();
    let mut columns = (0..image[0].len()).collect::<Vec<_>>();
    let galaxies = image
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            if !line.contains(&b'#') {
                lines.push(i);
                return vec![];
            }
            line.iter()
                .enumerate()
                .filter_map(|(j, c)| {
                    if *c == b'#' {
                        columns.retain(|col| *col != j);
                        Some((i, j))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    // let mut gals = galaxies.iter();
    for (n, (i, j)) in galaxies.iter().enumerate() {
        for (x, y) in galaxies.iter().skip(n) {
            let left = std::cmp::min(y, j);
            let right = std::cmp::max(y, j);
            sum += x - i + right - left;
            for l in lines.iter() {
                if (*i..*x).contains(l) {
                    sum += 999999;
                }
            }
            for col in columns.iter() {
                if (*left..*right).contains(col) {
                    sum += 999999;
                }
            }
        }
    }
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    static INPUT2: &str = "";

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "374".to_string())
    }

    //#[test]
    //fn test2() {
    //    assert_eq!(part2(INPUT2), "".to_string())
    //}
}
