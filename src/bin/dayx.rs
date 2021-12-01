fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#""#;

type Part1 = u32;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
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
    let input = std::fs::read_to_string("inputs/day0.txt").unwrap();
    assert_eq!(part1(&input), Part1::default())
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = u32;

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
    let input = std::fs::read_to_string("inputs/day0.txt").unwrap();
    assert_eq!(part2(&input), Part2::default())
}
