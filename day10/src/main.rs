fn main() {
    let input = include_str!("../day10.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Clone, Copy)]
enum Dir {
    Left,
    Right,
    Top,
    Bottom,
}
impl Dir {
    fn coords(&self) -> (isize, isize) {
        match self {
            Self::Left => (0, -1),
            Self::Right => (0, 1),
            Self::Top => (-1, 0),
            Self::Bottom => (1, 0),
        }
    }

    fn can_walk(&self, pipe: Pipe) -> bool {
        matches!(
            (self, pipe),
            (Self::Left, Pipe::UpR | Pipe::DnR | Pipe::LR)
                | (Self::Right, Pipe::UpL | Pipe::DnL | Pipe::LR)
                | (Self::Top, Pipe::DnL | Pipe::DnR | Pipe::UpDn)
                | (Self::Bottom, Pipe::UpR | Pipe::UpL | Pipe::UpDn)
        )
    }

    fn walk(&mut self, pipe: Pipe) {
        *self = match pipe {
            Pipe::DnR => match self {
                Dir::Left => Self::Bottom,
                Dir::Right => panic!(),
                Dir::Top => Self::Right,
                Dir::Bottom => panic!(),
            },
            Pipe::DnL => match self {
                Dir::Left => panic!(),
                Dir::Right => Self::Bottom,
                Dir::Top => Self::Left,
                Dir::Bottom => panic!(),
            },
            Pipe::UpR => match self {
                Dir::Left => Self::Top,
                Dir::Right => panic!(),
                Dir::Top => panic!(),
                Dir::Bottom => Self::Right,
            },
            Pipe::UpL => match self {
                Dir::Left => panic!(),
                Dir::Right => Self::Top,
                Dir::Top => panic!(),
                Dir::Bottom => Self::Left,
            },
            Pipe::UpDn => match self {
                Dir::Left => panic!(),
                Dir::Right => panic!(),
                Dir::Top => Self::Top,
                Dir::Bottom => Self::Bottom,
            },
            Pipe::LR => match self {
                Dir::Left => Self::Left,
                Dir::Right => Self::Right,
                Dir::Top => panic!(),
                Dir::Bottom => panic!(),
            },
            Pipe::S => panic!("Starting point"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
    UpL,
    UpR,
    DnL,
    DnR,
    UpDn,
    LR,
    S,
}

impl TryFrom<u8> for Pipe {
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'J' => Ok(Self::UpL),
            b'L' => Ok(Self::UpR),
            b'7' => Ok(Self::DnL),
            b'F' => Ok(Self::DnR),
            b'|' => Ok(Self::UpDn),
            b'-' => Ok(Self::LR),
            b'S' => Ok(Self::S),
            x => Err(format!("Invalid pipe: {x}")),
        }
    }

    type Error = String;
}
#[derive(Debug)]
struct Walker {
    dir: Dir,
    x: usize,
    y: usize,
}

impl Walker {
    fn new(dir: Dir, x: usize, y: usize) -> Self {
        Walker { dir, x, y }
    }

    fn walk(&mut self, pipes: &[Box<[u8]>]) {
        let (i, j) = self.dir.coords();
        self.x = (self.x as isize + i) as usize;
        self.y = (self.y as isize + j) as usize;
        let pipe = Pipe::try_from(pipes[self.x][self.y]).unwrap();
        if let Pipe::S = pipe {
            return;
        }
        self.dir.walk(pipe);
    }
}

type Pipes = Box<[Box<[u8]>]>;
fn find_animal(input: &str) -> (Pipes, Option<(usize, usize)>) {
    let mut animal = None;
    let pipes = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            if let Some(j) = line.find('S') {
                animal = Some((i, j));
            }

            line.bytes().collect::<Box<[u8]>>()
        })
        .collect::<Pipes>();
    (pipes, animal)
}

fn start_directions(pipes: &[Box<[u8]>], start: (usize, usize)) -> (Dir, Dir) {
    let mut dirs = [Dir::Left, Dir::Right, Dir::Top, Dir::Bottom]
        .into_iter()
        .filter(|direction| {
            let (i, j) = direction.coords();
            let (x, y) = (
                (start.0 as isize + i) as usize,
                (start.1 as isize + j) as usize,
            );
            if !(0..pipes.len()).contains(&x) || !(0..pipes[0].len()).contains(&y) {
                return false;
            }

            if let Ok(pipe) = Pipe::try_from(pipes[x][y]) {
                if direction.can_walk(pipe) {
                    return true;
                }
            };
            false
        });
    (dirs.next().unwrap(), dirs.next().unwrap())
}

fn part1(input: &str) -> String {
    let (pipes, animal) = find_animal(input);
    let (x, y) = animal.expect("Start not found");
    let (a, b) = start_directions(pipes.as_ref(), (x, y));
    let mut steps = 0u32;
    let mut a = Walker::new(a, x, y);
    let mut b = Walker::new(b, x, y);
    loop {
        steps += 1;
        a.walk(pipes.as_ref());
        b.walk(pipes.as_ref());

        if a.x == b.x && a.y == b.y {
            break;
        }
    }
    steps.to_string()
}

fn part2(input: &str) -> String {
    let (mut pipes, animal) = find_animal(input);
    let (x, y) = animal.expect("Start not found");
    let (a, b) = start_directions(pipes.as_ref(), (x, y));
    let mut path = pipes.clone();
    let mut w = Walker::new(a, x, y);
    loop {
        w.walk(pipes.as_ref());
        path[w.x][w.y] = b'P';
        if w.x == x && w.y == y {
            break;
        }
    }

    pipes[x][y] = match (a, b) {
        (Dir::Left, Dir::Right) | (Dir::Right, Dir::Left) => b'-',
        (Dir::Top, Dir::Right) | (Dir::Right, Dir::Top) => b'L',
        (Dir::Bottom, Dir::Right) | (Dir::Right, Dir::Bottom) => b'F',
        (Dir::Left, Dir::Top) | (Dir::Top, Dir::Left) => b'J',
        (Dir::Left, Dir::Bottom) | (Dir::Bottom, Dir::Left) => b'7',
        (Dir::Top, Dir::Bottom) | (Dir::Bottom, Dir::Top) => b'|',
        _ => panic!("smth idk"),
    };
    let mut total = 0u32;
    for (i, line) in pipes.iter().enumerate() {
        let mut inside = false;
        let mut enter = 0u8;
        for (j, byte) in line.iter().enumerate() {
            if path[i][j] == b'P' {
                if matches!(byte, b'F' | b'L') {
                    enter = *byte;
                } else if *byte == b'|' || matches!((enter, byte), (b'F', b'J') | (b'L', b'7')) {
                    inside = !inside;
                }
                continue;
            }
            if inside {
                total += 1;
            }
        }
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT1: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
    static INPUT2: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test1() {
        assert_eq!(part1(INPUT1), "8".to_string())
    }

    #[test]
    fn test2() {
        assert_eq!(part2(INPUT2), "10".to_string())
    }
}
