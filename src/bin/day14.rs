use std::collections::HashMap;

use advent_2021::{blank_lines, counts};
use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

type Part1 = usize;

fn parse(input: &str) -> (&str, HashMap<(char, char), char>) {
    use advent_2021::regex;
    let chunks = blank_lines(&input);
    let starting_template = chunks[0][0];

    let re = regex!(r"(\w+) -> (\w+)");
    let substitutions = &chunks[1];
    let substitutions = substitutions
        .into_iter()
        .map(|line| {
            re.captures(line)
                .map(|capture| {
                    (
                        (
                            capture[1].chars().nth(0).unwrap(),
                            capture[1].chars().nth(1).unwrap(),
                        ),
                        capture[2].chars().next().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();
    (starting_template, substitutions)
}

#[derive(Clone)]
struct Polymer {
    adjacent_pairs: HashMap<(char, char), usize>,
    counts: HashMap<char, usize>,
}

impl Polymer {
    fn new(starting_template: &str) -> Self {
        let counts = counts(starting_template.chars());
        let pairs = advent_2021::counts(starting_template.chars().tuple_windows());
        Polymer {
            counts,
            adjacent_pairs: pairs,
        }
    }
    fn next(&self, subsitutions: &HashMap<(char, char), char>) -> Self {
        let mut next = HashMap::new();
        let mut next_counts = self.counts.clone();
        for ((c1, c2), count) in &self.adjacent_pairs {
            let btwn = subsitutions[&(*c1, *c2)];
            *next_counts.entry(btwn).or_insert(0) += count;
            *next.entry((*c1, btwn)).or_insert(0) += count;
            *next.entry((btwn, *c2)).or_insert(0) += count;
        }

        Polymer {
            counts: next_counts,
            adjacent_pairs: next,
        }
    }
    fn next_n(&self, subsitutions: &HashMap<(char, char), char>, n: usize) -> Self {
        let mut p = self.clone();
        for _ in 0..n {
            p = p.next(subsitutions);
        }
        p
    }
    fn counts(self) -> HashMap<char, usize> {
        self.counts
    }
}

fn part1(input: &str) -> Part1 {
    let (starting, subs) = parse(input);
    let p = Polymer::new(starting);
    let p = p.next_n(&subs, 10);
    let counts = p.counts();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 1588)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day14.txt").unwrap();
    assert_eq!(part1(&input), 4244)
}

////////////////////////////////////////////////
///  start part 2
/// can keep track of adjacent pair counts and character counts
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let (starting, subs) = parse(input);
    let mut p = Polymer::new(starting);
    for _ in 0..40 {
        p = p.next(&subs)
    }
    let counts = p.counts();
    counts.values().max().unwrap() - counts.values().min().unwrap()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 2188189693529)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day14.txt").unwrap();
    assert_eq!(part2(&input), 4807056953866)
}
