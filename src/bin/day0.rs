//! This borrows a problem from a previous year
//! to make sure that the template file is hooked up correctly.

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"1721
979
366
299
675
1456"#;

type Part1 = Option<u32>;
type Part2 = Option<u32>;

fn parse(input: &str) -> Vec<u32> {
    use advent_2021::regex;
    let re = regex!(r"(\d+)");
    re.captures_iter(input)
        .map(|capture| capture[0].parse().unwrap())
        .collect()
}

fn part1(input: &str) -> Part1 {
    let list = parse(input);

    for i in &list {
        for j in &list {
            if i + j == 2020 {
                return Some(i * j);
            }
        }
    }
    None
}

fn part2(input: &str) -> Part2 {
    let list = parse(input);

    for i in &list {
        for j in &list {
            for k in &list {
                if i + j + k == 2020 {
                    return Some(i * j * k);
                }
            }
        }
    }
    None
}

#[test]
fn tpart1_sample() {
    let input = SAMPLE;
    assert_eq!(part1(&input), Some(514579))
}

#[test]
fn tpart2_sample() {
    let input = SAMPLE;
    assert_eq!(part2(&input), Some(241861950))
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day0.txt").unwrap();
    assert_eq!(part1(&input), Some(793524))
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day0.txt").unwrap();
    assert_eq!(part2(&input), Some(61515678))
}
