use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"3,4,3,1,2"#;

type Part1 = usize;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn part1(input: &str) -> Part1 {
    let mut input = parse(input);
    for _day in 0..80 {
        let starting_pos = input.len();
        for index in 0..starting_pos {
            if input[index] == 0 {
                input[index] = 6;
                input.push(8);
            } else {
                input[index] -= 1;
            }
        }
    }
    input.len()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 5934)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    assert_eq!(part1(&input), 390923)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

static MEMO_DATA: Lazy<Mutex<HashMap<Rabbit, usize>>> = Lazy::new(|| {
    let hm = HashMap::new();
    Mutex::new(hm)
});

#[derive(Hash, Eq, Clone, Copy, PartialEq, Debug)]
enum Rabbit {
    Mature(usize),
}
fn num_produced(r: &Rabbit) -> usize {
    if let Some(result) = MEMO_DATA.lock().unwrap().get(r) {
        return *result;
    }

    use Rabbit::*;
    let result = match r {
        Mature(age) => {
            (age / 7)
                + ((7..)
                    .step_by(7)
                    .take_while(|increment| increment + 2 <= *age)
                    .filter_map(|year_diff| age.checked_sub(year_diff + 2))
                    .map(|na| num_produced(&Rabbit::Mature(na)))
                    .sum::<usize>())
        }
    };

    MEMO_DATA.lock().unwrap().insert(r.clone(), result);

    result
}
fn part2(input: &str) -> Part2 {
    let input = parse(input);
    input.len()
        + input
            .into_iter()
            .map(|starting| Rabbit::Mature(256 + (7 - starting - 1) as usize))
            .map(|r| num_produced(&r))
            .sum::<Part2>()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 26984457539)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    assert_eq!(part2(&input), 1749945484935)
}
