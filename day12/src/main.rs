fn main() {
    let input = include_str!("../day12.txt");
    println!("Part 1: {}", part1(input));
    // println!("Part 2: {}", part2(input));
}

struct Line {
    groups: Vec<Vec<u8>>,
    group_sizes: Vec<usize>,
}

impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let (records, values) = value.split_once(' ').unwrap();
        let groups = records
            .split('.')
            .map(|g| g.as_bytes().to_vec())
            .collect::<Vec<_>>();
        let group_sizes = values
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        Self {
            groups,
            group_sizes,
        }
    }
}

impl Line {
    fn arrangments(mut self) -> usize {
        println!("A:{:?}", self.groups);
        {
            let lowest = self.group_sizes.iter().min().unwrap();
            self.groups.retain(|group| group.len() >= *lowest)
        }
        {
            let first = self.group_sizes.first().unwrap();
            let last = self.group_sizes.last().unwrap();
            self.groups = self
                .groups
                .into_iter()
                .skip_while(|group| group.len() < *first)
                .collect();
            self.groups.reverse();
            self.groups = self
                .groups
                .into_iter()
                .skip_while(|group| group.len() < *last)
                .collect();
            self.groups.reverse();
        }

        println!("B:{:?}", self.groups);
        println!("S:{:?}", self.group_sizes);
        // if self.groups.len() == self.group_sizes.len() {
        //     let mut sizes = self.group_sizes.into_iter();
        //     return self
        //         .groups
        //         .into_iter()
        //         .map(|group| group.combinations(sizes.next().unwrap()))
        //         .sum();
        // }
        0
    }
}

fn part1(input: &str) -> String {
    input
        .lines()
        .map(|line| Line::from(line).arrangments())
        .sum::<usize>()
        .to_string()
}

fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    static INPUT2: &str = "";

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "21".to_string())
    }

    //#[test]
    //fn test2() {
    //    assert_eq!(part2(INPUT2), "".to_string())
    //}
}
