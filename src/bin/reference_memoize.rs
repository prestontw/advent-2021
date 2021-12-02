fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#""#;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
static MEMO_DATA: Lazy<Mutex<HashMap<i32, String>>> = Lazy::new(|| {
    let hm = HashMap::new();
    Mutex::new(hm)
});

type Part1 = usize;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> Part1 {
    let _input = parse(input);
    let data = MEMO_DATA.lock().unwrap().insert(3, "hellov".into());
    Part1::default()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), Part1::default())
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/dayx.txt").unwrap();
    assert_eq!(part1(&input), Part1::default())
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
    let input = std::fs::read_to_string("inputs/dayx.txt").unwrap();
    assert_eq!(part2(&input), Part2::default())
}
