fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#""#;

type Part1 = usize;

fn parse(input: &str) -> Vec<Vec<&str>> {
    let re = {
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| {
            println!("hello from init"); // just prints once
            regex::Regex::new(r"(\w*) (\w*) bags contain (.*)\.").unwrap()
        })
    };
    input
        .lines()
        .map(|s| advent_2021::extract_values(re, s))
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> Part1 {
    let _input = parse(input);
    Part1::default()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), Part1::default())
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/reference_regex.txt").unwrap();
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
