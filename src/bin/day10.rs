use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"#;

type Part1 = usize;

fn parse(input: &str) -> Vec<usize> {
    use advent_2021::regex;
    let re = regex!(r"(\d+)");
    re.captures_iter(input)
        .map(|capture| capture[0].parse().unwrap())
        .collect()
}

fn part1(input: &str) -> Part1 {
    let bounty: HashMap<char, usize> =
        HashMap::from_iter(vec![(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let pairs: HashMap<char, char> =
        HashMap::from_iter(vec![(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let mut score = 0;
    for line in input.lines() {
        let mut stack = vec![];
        for char in line.chars() {
            if bounty.contains_key(&char) {
                if stack.last().unwrap() == &pairs[&char] {
                    stack.pop();
                } else {
                    score += bounty[&char];
                    break;
                }
            } else {
                stack.push(char)
            }
        }
    }
    score
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 6 + 57 + 1197 + 25137)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();
    assert_eq!(part1(&input), 392097)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let pairs: HashMap<char, char> =
        HashMap::from_iter(vec![(')', '('), (']', '['), ('}', '{'), ('>', '<')]);
    let translate: HashMap<char, char> =
        HashMap::from_iter(vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]);
    let bounty: HashMap<char, usize> =
        HashMap::from_iter(vec![(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut incomplete_lines = input
        .lines()
        .filter_map(|line| {
            let mut stack = vec![];
            for char in line.chars() {
                if bounty.contains_key(&char) {
                    if dbg!(&stack).last().unwrap() == &pairs[&char] {
                        stack.pop();
                    } else {
                        return None;
                    }
                } else {
                    stack.push(char)
                }
            }
            Some(stack)
        })
        .map(|mut stack| {
            stack.reverse();
            stack
                .into_iter()
                .fold(0, |acc, char| acc * 5 + bounty[&translate[&char]])
        })
        .collect_vec();
    incomplete_lines.sort();

    incomplete_lines[incomplete_lines.len() / 2]
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 288957)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day10.txt").unwrap();
    assert_eq!(part2(&input), Part2::default())
}
