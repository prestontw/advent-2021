use std::str::FromStr;

use advent_2021::regex;
use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"forward 5
down 5
forward 8
up 3
down 8
forward 2"#;

type Part1 = usize;
enum Directions {
    Forward,
    Down,
    Up,
}

impl FromStr for Directions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => return Ok(Self::Forward),
            "up" => return Ok(Self::Up),
            "down" => return Ok(Self::Down),
            _ => return Err(()),
        }
    }
}

struct Move {
    direction: Directions,
    distance: usize,
}

fn parse(input: &str) -> Vec<Move> {
    let re = regex!(r"(\w+) (\d+)");
    re.captures_iter(input)
        .map(|capture| Move {
            direction: capture[1].parse().unwrap(),
            distance: capture[2].parse().unwrap(),
        })
        .collect()
}

fn part1(input: &str) -> Part1 {
    let input = parse(input);
    let coordinates = input
        .into_iter()
        .fold((0, 0), |acc, current| match current.direction {
            Directions::Up => (acc.0, acc.1 - current.distance),
            Directions::Forward => (acc.0 + current.distance, acc.1),
            Directions::Down => (acc.0, acc.1 + current.distance),
        });
    coordinates.0 * coordinates.1
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 150)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    assert_eq!(part1(&input), 1804520)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = usize;

fn part2(input: &str) -> Part2 {
    let input = parse(input);
    let coordinates = input
        .into_iter()
        .fold((0, 0, 0), |acc, current| match current.direction {
            Directions::Up => (acc.0, acc.1, acc.2 - current.distance),
            Directions::Forward => (
                acc.0 + current.distance,
                acc.1 + (current.distance * acc.2),
                acc.2,
            ),
            Directions::Down => (acc.0, acc.1, acc.2 + current.distance),
        });
    coordinates.0 * coordinates.1
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 900)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day2.txt").unwrap();
    assert_eq!(part2(&input), 1971095320)
}
