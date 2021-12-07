fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#""#;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;
static MEMO_DATA: Lazy<Mutex<HashMap<usize, usize>>> = Lazy::new(|| {
    let hm = HashMap::new();
    Mutex::new(hm)
});

type Part1 = usize;

fn parse(input: &str) -> Vec<u32> {
    static RE: once_cell::sync::OnceCell<HashMap<(), ()>> = once_cell::sync::OnceCell::new();
    RE.get_or_init(|| {
        println!("initializing");
        HashMap::new()
    });
    input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn fib(n: usize) -> usize {
    if n == 1 || n == 0 {
        return 1;
    }
    if let Some(ret) = MEMO_DATA.lock().unwrap().get(&n) {
        return *ret;
    }
    println!("calculating for {}", n);
    let new = fib(n - 1) + fib(n - 2);
    MEMO_DATA.lock().unwrap().insert(n, new);
    new
}

fn part1(_input: &str) -> Part1 {
    fib(5);
    fib(5);
    fib(3);
    Part1::default()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 0)
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
