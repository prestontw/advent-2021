fn main() {
    println!("Hello, world!");
}

use advent_2021::{counts, regex};
const SAMPLE: &str = r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#;

type Part1 = usize;

fn parse(input: &str) -> Vec<String> {
    let re = regex!(r"(\d+)");
    re.captures_iter(input)
        .map(|capture| capture[0].to_string())
        .collect()
}

fn part1(input: &str) -> Part1 {
    let input = parse(input);
    let len = input[0].len();
    let mut ret = String::new();
    for index in 0..len {
        let counts = counts(input.iter().map(|line| line.chars().nth(index).unwrap()));
        let ones = counts[&'1'];
        let zeros = counts[&'0'];
        ret += if ones > zeros { "1" } else { "0" };
    }
    let gamma_rate = usize::from_str_radix(&ret, 2).unwrap();
    let max = 2_usize.pow(len as u32) - 1;
    gamma_rate * (max - gamma_rate)
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 22 * 9)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    assert_eq!(part1(&input), 2261546)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let input = parse(input);
    let filter = |mut candidates: Vec<String>, f: &dyn Fn(usize, usize) -> char| {
        let mut index = 0;
        while candidates.len() != 1 {
            let counts = counts(
                candidates
                    .iter()
                    .map(|line| line.chars().nth(index).unwrap()),
            );
            let ones = counts[&'1'];
            let zeros = counts[&'0'];

            let desired = f(ones, zeros);
            candidates = candidates
                .into_iter()
                .filter(|line| line.chars().nth(index).unwrap() == desired)
                .collect();
            index += 1;
        }
        usize::from_str_radix(&candidates[0], 2).unwrap()
    };
    let o2 = filter(input.clone(), &|ones, zeros| {
        if ones >= zeros {
            '1'
        } else {
            '0'
        }
    });
    let co2 = filter(input.clone(), &|ones, zeros| {
        if ones < zeros {
            '1'
        } else {
            '0'
        }
    });
    o2 * co2
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 23 * 10)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day3.txt").unwrap();
    assert_eq!(part2(&input), 6775520)
}
