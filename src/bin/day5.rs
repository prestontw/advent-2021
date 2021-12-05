use std::collections::HashMap;

use advent_2021::abs_diff;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

type Part1 = usize;

fn parse(input: &str) -> Vec<((u32, u32), (u32, u32))> {
    use advent_2021::regex;
    let re = regex!(r"(\d+),(\d+) -> (\d+),(\d+)");
    re.captures_iter(input)
        .map(|capture| {
            (
                (capture[1].parse().unwrap(), capture[2].parse().unwrap()),
                (capture[3].parse().unwrap(), capture[4].parse().unwrap()),
            )
        })
        .collect()
}

fn horizontal_or_vertical(p1: &(u32, u32), p2: &(u32, u32)) -> bool {
    p1.0 == p2.0 || p1.1 == p2.1
}

fn part1(input: &str) -> Part1 {
    let input = parse(input)
        .into_iter()
        .filter(|(p1, p2)| horizontal_or_vertical(p1, p2))
        .collect::<Vec<((_, _), (_, _))>>();
    let mut frequencies = HashMap::new();
    for (p1, p2) in input {
        let (start, end) = (p1.0.min(p2.0), p1.0.max(p2.0));
        for x in start..=end {
            let (start, end) = (p1.1.min(p2.1), p1.1.max(p2.1));
            for y in start..=end {
                let count = frequencies.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
    }
    frequencies.values().filter(|&&v| v > 1).count()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 5)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    assert_eq!(part1(&input), 4993)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

struct Pos(u32);
impl Pos {
    fn step(&self, other: u32) -> Self {
        if self.0 == other {
            Pos(self.0)
        } else if self.0 == 0 || (abs_diff(self.0 + 1, other) < abs_diff(self.0 - 1, other)) {
            Pos(self.0 + 1)
        } else {
            Pos(self.0 - 1)
        }
    }
}

fn part2(input: &str) -> Part2 {
    let input = parse(input).into_iter().collect::<Vec<((_, _), (_, _))>>();
    let mut frequencies = HashMap::new();
    for (p1, p2) in input {
        let mut xstart = Pos(p1.0);
        let mut ystart = Pos(p1.1);
        while xstart.0 != p2.0 || ystart.0 != p2.1 {
            let count = frequencies.entry((xstart.0, ystart.0)).or_insert(0);
            *count += 1;
            xstart = xstart.step(p2.0);
            ystart = ystart.step(p2.1);
        }
        let count = frequencies.entry((xstart.0, ystart.0)).or_insert(0);
        *count += 1;
    }
    frequencies.values().filter(|&&v| v > 1).count()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 12)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day5.txt").unwrap();
    assert_eq!(part2(&input), 21101)
}
