use advent_2021::abs_diff;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"16,1,2,0,4,2,7,1,2,14"#;

type Part1 = u32;

fn parse(input: &str) -> Vec<u32> {
    use advent_2021::regex;
    let re = regex!(r"(\d+)");
    re.captures_iter(input)
        .map(|capture| capture[0].parse().unwrap())
        .collect()
}

fn part1(input: &str) -> Part1 {
    let input = parse(input);
    let max = *input.iter().max().unwrap();
    let mut min = u32::MAX;
    for aligned in 0..max {
        let cur_min = input.iter().map(|pos| abs_diff(*pos, aligned)).sum();
        if cur_min < min {
            min = cur_min
        }
    }
    min
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 37)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    assert_eq!(part1(&input), 348996)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let input = parse(input);
    let max = *input.iter().max().unwrap();
    let mut min = u32::MAX;
    for aligned in 0..max {
        let cur_min = input
            .iter()
            .map(|pos| {
                let distance = abs_diff(*pos, aligned);
                (distance * (distance + 1)) / 2
            })
            .sum();
        if cur_min < min {
            min = cur_min
        }
    }
    min
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 168)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day7.txt").unwrap();
    assert_eq!(part2(&input), 98231647)
}
