use std::collections::{HashMap, HashSet};

use advent_2021::blank_lines;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

type Part1 = u32;
#[derive(PartialEq, Eq, Clone, Copy)]
enum Cell {
    Called,
    Uncalled,
}
struct Grouping(HashMap<u32, Cell>);
impl Grouping {
    fn is_complete(&self) -> bool {
        self.0.values().all(|s| *s == Cell::Called)
    }
    fn mark(&mut self, num: u32) {
        if let Some(cell) = self.0.get_mut(&num) {
            *cell = Cell::Called;
        }
    }
}
struct BingoBoard {
    columns: Vec<Grouping>,
    rows: Vec<Grouping>,
}
impl BingoBoard {
    fn final_score(&self, just_called: u32) -> Option<u32> {
        self.columns
            .iter()
            .chain(self.rows.iter())
            .filter(|grouping| grouping.is_complete())
            .next()
            .map(|_grouping| {
                self.rows
                    .iter()
                    .flat_map(|grouping| {
                        grouping.0.iter().filter_map(|(val, done)| {
                            if *done == Cell::Uncalled {
                                Some(val)
                            } else {
                                None
                            }
                        })
                    })
                    .sum::<u32>()
                    * just_called as u32
            })
    }
    fn call(&mut self, number: u32) {
        for row in &mut self.rows {
            row.mark(number);
        }
        for col in &mut self.columns {
            col.mark(number);
        }
    }
}

fn parse(input: &str) -> (Vec<u32>, Vec<BingoBoard>) {
    let chunks = blank_lines(input);
    let numbers = chunks[0][0]
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let ret = chunks[1..]
        .into_iter()
        .map(|chunk| {
            let mut v: Vec<Vec<u32>> = Vec::with_capacity(chunk.len());
            for line in chunk {
                v.push(
                    line.split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect(),
                );
            }
            let rows = v
                .clone()
                .into_iter()
                .map(|row| Grouping(row.into_iter().map(|it| (it, Cell::Uncalled)).collect()))
                .collect();
            let columns = {
                let mut ret: Vec<Grouping> = vec![];
                for y in 0..5 {
                    let mut column = vec![];
                    for x in 0..5 {
                        column.push(v[x][y]);
                    }
                    ret.push(Grouping(
                        column.into_iter().map(|it| (it, Cell::Uncalled)).collect(),
                    ));
                }
                ret
            };
            BingoBoard { rows, columns }
        })
        .collect();
    (numbers, ret)
}

fn part1(input: &str) -> Part1 {
    let (items, mut boards) = parse(input);
    for num in items {
        for board in &mut boards {
            board.call(num);
            if let Some(score) = board.final_score(num) {
                return score;
            }
        }
    }
    Part1::default()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 188 * 24)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    assert_eq!(part1(&input), 12796)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let (items, mut boards) = parse(input);
    let mut finished_boards = HashSet::new();
    let total_boards = boards.len();
    for num in items {
        for (index, board) in boards.iter_mut().enumerate() {
            board.call(num);
            if let Some(score) = board.final_score(num) {
                finished_boards.insert(index);
                if finished_boards.len() == total_boards {
                    return score;
                }
            }
        }
    }
    Part1::default()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 148 * 13)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day4.txt").unwrap();
    assert_eq!(part2(&input), 18063)
}
