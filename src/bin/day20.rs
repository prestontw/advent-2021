use std::{collections::HashMap, str::FromStr};

use advent_2021::blank_lines;
use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#;

type Part1 = usize;

#[derive(Clone, Copy)]
enum Pixel {
    Light, // #
    Dark,  // .
}
impl FromStr for Pixel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Pixel::Light),
            "." => Ok(Pixel::Dark),
            _ => Err(()),
        }
    }
}
impl ToString for Pixel {
    fn to_string(&self) -> String {
        match self {
            Self::Light => "#".to_string(),
            Self::Dark => ".".to_string(),
        }
    }
}
impl Pixel {
    fn to_binary(&self) -> u32 {
        match self {
            Pixel::Light => 1,
            Pixel::Dark => 0,
        }
    }
    fn flip(&self) -> Self {
        match self {
            Pixel::Light => Self::Dark,
            Pixel::Dark => Self::Light,
        }
    }
}
fn binary_to_decimal(u: &[u32]) -> usize {
    usize::from_str_radix(&u.iter().map(|u| u.to_string()).collect::<String>(), 2).unwrap()
}
enum BackgroundBehavior {
    Stable,
    Flip(Pixel),
}
impl BackgroundBehavior {
    fn background_pixel(&self) -> Pixel {
        match self {
            &BackgroundBehavior::Stable => Pixel::Dark,
            &BackgroundBehavior::Flip(current) => current,
        }
    }
    fn next(&self) -> Self {
        match self {
            &BackgroundBehavior::Stable => Self::Stable,
            &BackgroundBehavior::Flip(current) => Self::Flip(current.flip()),
        }
    }
}
struct Grid {
    grid: HashMap<(i64, i64), Pixel>,
    background_behavior: BackgroundBehavior,
}

impl Grid {
    fn new(grid: HashMap<(i64, i64), Pixel>, first_instruction: &str) -> Self {
        let background_behavior = match first_instruction {
            "." => BackgroundBehavior::Stable,
            _ => BackgroundBehavior::Flip(Pixel::Dark),
        };
        Self {
            grid,
            background_behavior,
        }
    }
    fn upper_left(&self) -> (i64, i64) {
        let left = *self.grid.keys().map(|(x, _y)| x).min().unwrap();
        let upper = *self.grid.keys().map(|(_x, y)| y).min().unwrap();
        (left, upper)
    }
    fn lower_right(&self) -> (i64, i64) {
        let right = *self.grid.keys().map(|(x, _y)| x).max().unwrap();
        let lower = *self.grid.keys().map(|(_x, y)| y).max().unwrap();
        (right, lower)
    }

    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let mut ret = "".into();

        let upper_left = self.upper_left();
        let lower_right = self.lower_right();
        let background = self.background_behavior.background_pixel();

        for x in (upper_left.0 - 1)..(lower_right.0 + 2) {
            let mut row = "\n".to_string();
            for y in (upper_left.1 - 1)..(lower_right.1 + 2) {
                let c = self.grid.get(&(x, y)).unwrap_or(&background);
                row += &c.to_string();
            }
            ret += &row[..];
        }
        ret
    }
    fn next(&self, instructions: &Vec<Pixel>) -> Self {
        let upper_left = self.upper_left();
        let lower_right = self.lower_right();

        let mut ret = HashMap::new();
        // plus 2 is important since points above are inclusive
        for x in (upper_left.0 - 4)..(lower_right.0 + 5) {
            for y in (upper_left.1 - 4)..(lower_right.1 + 5) {
                let positions = [
                    (x - 1, y - 1),
                    (x - 1, y),
                    (x - 1, y + 1),
                    (x, y - 1),
                    (x, y),
                    (x, y + 1),
                    (x + 1, y - 1),
                    (x + 1, y),
                    (x + 1, y + 1),
                ];
                let raw = positions
                    .iter()
                    .map(|pos| {
                        self.grid
                            .get(pos)
                            .unwrap_or(&self.background_behavior.background_pixel())
                            .to_binary()
                    })
                    .collect_vec();
                let index = binary_to_decimal(&raw);
                ret.insert((x, y), instructions[index]);
            }
        }

        Grid {
            grid: ret,
            background_behavior: self.background_behavior.next(),
        }
    }
    fn count_lit(&self) -> usize {
        self.grid
            .values()
            .filter(|v| matches!(v, Pixel::Light))
            .count()
    }
}

fn parse(input: &str) -> (Vec<Pixel>, Grid) {
    let first = &input[..1];
    let input = blank_lines(input);
    let instructions = input[0]
        .iter()
        .flat_map(|s| s.chars().map(|c| c.to_string().parse().unwrap()))
        .collect::<Vec<Pixel>>();
    let mut grid = HashMap::new();
    input[1].iter().enumerate().for_each(|(line_index, line)| {
        line.chars().enumerate().for_each(|(char_index, c)| {
            grid.insert(
                (line_index as i64, char_index as i64),
                c.to_string().parse().unwrap(),
            );
        });
    });
    (instructions, Grid::new(grid, first))
}

fn part1(input: &str) -> Part1 {
    let (instructions, grid) = parse(input);
    // println!("{}", grid.to_string());
    grid.next(&instructions).next(&instructions).count_lit()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 35)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day20.txt").unwrap();
    let result = part1(&input);
    assert_ne!(result, 5249);
    assert_ne!(result, 5447);
    assert_ne!(result, 5969);
    assert_ne!(result, 7142);
    assert_eq!(result, 5354)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let (instructions, mut grid) = parse(input);
    for _time in 0..50 {
        grid = grid.next(&instructions);
    }
    grid.count_lit()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 3351)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day20.txt").unwrap();
    let result = part2(&input);
    assert_ne!(result, 223943);
    assert_eq!(result, 18269)
}
