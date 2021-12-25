use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const INPUT_PATH: &str = "inputs/day25.txt";
const SAMPLE: &str = r#"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>"#;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Space {
    East,
    South,
    Empty,
}

type Part1 = usize;

#[derive(PartialEq, Eq, Debug)]
struct Herd(Vec<Vec<Space>>);
fn parse(input: &str) -> Vec<Vec<Space>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '.' {
                        Space::Empty
                    } else if c == '>' {
                        Space::East
                    } else {
                        Space::South
                    }
                })
                .collect()
        })
        .collect()
}

impl Herd {
    // had to collect indices to move first rather than applying immediately
    fn next(&self) -> Herd {
        let mut ret = self.0.clone();
        let mut moves = vec![];
        for (row_index, row) in self.0.iter().enumerate() {
            for (index, piece) in row.iter().enumerate() {
                if !matches!(piece, Space::East) {
                    continue;
                }
                let next_index = (index + 1) % row.len();
                if matches!(row[next_index], Space::Empty) {
                    moves.push([(row_index, index), (row_index, next_index)]);
                }
            }
        }
        for [(row1, col1), (row2, col2)] in moves {
            ret[row1][col1] = Space::Empty;
            ret[row2][col2] = Space::East;
        }
        let mut moves = vec![];
        for col_index in 0..self.0[0].len() {
            for row_index in 0..self.0.len() {
                if !matches!(ret[row_index][col_index], Space::South) {
                    continue;
                }
                let next_index = (row_index + 1) % self.0.len();
                if matches!(ret[next_index][col_index], Space::Empty) {
                    moves.push([(row_index, col_index), (next_index, col_index)]);
                }
            }
        }
        for [(row1, col1), (row2, col2)] in moves {
            ret[row1][col1] = Space::Empty;
            ret[row2][col2] = Space::South;
        }
        Herd(ret)
    }
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|line| {
                line.iter()
                    .map(|s| match s {
                        Space::Empty => ".",
                        Space::East => ">",
                        Space::South => "v",
                    })
                    .collect::<String>()
            })
            .collect_vec()
            .join("\n")
    }
}

#[test]
fn one_step() {
    let initial = r"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
    let first = Herd(parse(initial));
    let next = r"....>.>v.>
v.v>.>v.v.
>v>>..>v..
>>v>v>.>.v
.>v.v...v.
v>>.>vvv..
..v...>>..
vv...>>vv.
>.v.v..v.v";
    let next = Herd(parse(next));
    println!("{}", first.next().to_string());
    assert_eq!(first.next().to_string(), next.to_string());
    assert_eq!(first.next(), next);
}

// east first, then south
fn part1(input: &str) -> Part1 {
    let input = parse(input);
    let mut previous = Herd(input);
    for day in 1.. {
        let next = previous.next();
        if next == previous {
            return day;
        }
        previous = next;
    }
    Part1::default()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 58)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string(&INPUT_PATH).unwrap();
    assert_eq!(part1(&input), 534)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let _input = parse(input);
    Part2::default()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), Part2::default())
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    assert_eq!(part2(&input), Part2::default())
}
