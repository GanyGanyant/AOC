fn main() {
    let input = include_str!("../day1.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

const R: u32 = 12;
const G: u32 = 13;
const B: u32 = 14;

fn part1(input: &str) -> String {
    input
        .lines()
        .filter_map(|line| {
            let (game, data) = line.split_once(": ").unwrap();
            let num = game.split_once(' ').unwrap().1.parse::<u32>().unwrap();
            if data
                .split("; ")
                .flat_map(move |set| {
                    set.split(", ").map(|pair| {
                        let (amount, color) = pair.split_once(' ').unwrap();
                        let n = amount.parse::<u32>().unwrap();
                        match color {
                            "red" => {
                                if n > R {
                                    return false;
                                }
                            }
                            "green" => {
                                if n > G {
                                    return false;
                                }
                            }
                            "blue" => {
                                if n > B {
                                    return false;
                                }
                            }
                            s => panic!("wrong color: {s}"),
                        }
                        true
                    })
                })
                .skip_while(|b| *b)
                .count()
                > 0
            {
                return None;
            }
            Some(num)
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            let (_, data) = line.split_once(": ").unwrap();
            let (mut r, mut g, mut b) = (0u32, 0u32, 0u32);
            let com = |amount, color: &mut u32| {
                if amount > *color {
                    *color = amount;
                }
            };
            data.split("; ").for_each(|set| {
                set.split(", ").for_each(|pair| {
                    let (amount, color) = pair.split_once(' ').unwrap();
                    let n = amount.parse::<u32>().unwrap();
                    match color {
                        "red" => com(n, &mut r),
                        "green" => com(n, &mut g),
                        "blue" => com(n, &mut b),
                        s => panic!("wrong color: {s}"),
                    };
                });
            });
            r * g * b
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "8".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "2286".to_string())
    }
}
