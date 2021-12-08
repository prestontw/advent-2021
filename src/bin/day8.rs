use itertools::Itertools;
use std::collections::HashSet;
fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

type Part1 = usize;

fn parse(input: &str) -> Vec<(Vec<&str>, Vec<&str>)> {
    input
        .lines()
        .map(|line| {
            let mut it = line.split('|');
            let first = it.next().unwrap();
            let second = it.next().unwrap();
            (
                first.split_ascii_whitespace().collect(),
                second.split_ascii_whitespace().collect(),
            )
        })
        .collect()
    // re.captures_iter(input)
    //     .map(|capture| {
    //         let pieces = capture.len() - 1;
    //         (0..pieces)
    //             .map(move |piece| capture[piece].trim().to_string())
    //             .collect()
    //     })
    //     .collect()
}

fn part1(input: &str) -> Part1 {
    let input = parse(input);
    input
        .into_iter()
        .map(|(_, line)| {
            line.into_iter()
                .filter(|word| {
                    let len = (dbg!(word)).len();
                    len == 2 || len == 3 || len == 4 || len == 7
                })
                .count()
        })
        .sum()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 26)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
    assert_eq!(part1(&input), 375)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn is_nine(four: &HashSet<char>, potential_nine: &HashSet<char>) -> bool {
    four.difference(potential_nine).count() == 0
}

fn is_five(six: &HashSet<char>, potential_five: &HashSet<char>) -> bool {
    potential_five.difference(six).count() == 0
}

fn is_three(seven: &HashSet<char>, potential_three: &HashSet<char>) -> bool {
    seven.difference(potential_three).count() == 0
}

fn is_zero(seven: &HashSet<char>, potential: &HashSet<char>) -> bool {
    seven.difference(potential).count() == 0
}

fn part2(input: &str) -> Part2 {
    let input = parse(input);
    input
        .into_iter()
        .map(|(clues, digits)| {
            let clues = clues
                .into_iter()
                .map(|word| HashSet::from_iter(word.chars()))
                .collect::<Vec<_>>();
            let one = (clues.iter().find(|word| word.len() == 2).unwrap());
            let eight = clues.iter().find(|word| word.len() == 7).unwrap();
            let seven = clues.iter().find(|word| word.len() == 3).unwrap();
            let four = clues.iter().find(|word| word.len() == 4).unwrap();

            let nine = clues
                .iter()
                .find(|potential| potential.len() == 6 && is_nine(four, potential))
                .unwrap();
            let zero = clues
                .iter()
                .find(|&word| word.len() == 6 && word != nine && is_zero(seven, word))
                .unwrap();
            let six = clues
                .iter()
                .find(|&potential| potential.len() == 6 && potential != zero && potential != nine)
                .unwrap();

            let three = clues
                .iter()
                .find(|word| word.len() == 5 && is_three(seven, word))
                .unwrap();
            let five = clues
                .iter()
                .find(|word| word.len() == 5 && is_five(six, word))
                .unwrap();
            let two = clues
                .iter()
                .find(|&potential| potential.len() == 5 && potential != three && potential != five)
                .unwrap();

            let lookup = dbg!(vec![
                zero, one, two, three, four, five, six, seven, eight, nine
            ]);

            let num = digits
                .into_iter()
                .map(|word| {
                    let hs: HashSet<_> = dbg!(word.chars().collect());
                    lookup
                        .iter()
                        .enumerate()
                        .filter_map(|(i, &num)| {
                            if num == &hs {
                                Some(i.to_string())
                            } else {
                                None
                            }
                        })
                        .next()
                        .unwrap()
                })
                .collect::<String>();
            (dbg!(num)).parse::<usize>().unwrap()
        })
        .sum()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 61229)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day8.txt").unwrap();
    assert_eq!(part2(&input), 1019355)
}
