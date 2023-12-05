fn main() {
    let input = include_str!("../day5.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> String {
    let mut maps = input.split("\n\n");
    let seeds = maps
        .next()
        .map(|seed| {
            seed.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect::<Vec<u64>>()
        })
        .unwrap();
    lowest_location(maps, seeds)
}

fn part2(input: &str) -> String {
    let mut maps = input.split("\n\n");
    let mut seeds: Vec<u64> = maps
        .next()
        .map(|seed| {
            let data = seed
                .split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .collect::<Vec<u64>>();
            let size = data.len() / 2;
            (0..size)
                .flat_map(|i| (data[i * 2]..data[i * 2] + data[i * 2 + 1]).collect::<Vec<u64>>())
                .collect()
        })
        .unwrap();
    lowest_location(maps, seeds)
}

fn lowest_location<'a>(maps: impl IntoIterator<Item = &'a str>, mut seeds: Vec<u64>) -> String {
    maps.into_iter().for_each(|map| {
        let mut new_seeds = Vec::new();
        map.lines().skip(1).for_each(|line| {
            let nums = line
                .split_whitespace()
                .filter_map(|num| num.parse::<u64>().ok())
                .take(3)
                .collect::<Vec<u64>>();
            let (to, from, range) = (nums[0], nums[1], nums[2]);
            let change = to as i64 - from as i64;
            seeds = seeds
                .iter_mut()
                .filter_map(|seed| {
                    if (from..from + range).contains(seed) {
                        new_seeds.push((*seed as i64 + change) as u64);
                        None
                    } else {
                        Some(*seed)
                    }
                })
                .collect();
        });
        seeds.append(&mut new_seeds);
    });
    seeds.sort();
    seeds.first().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    static INPUT2: &str = INPUT1;

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "35".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "46".to_string())
    }
}
