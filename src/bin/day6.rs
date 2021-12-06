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
        for index in (0..starting_pos) {
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
    let input = std::fs::read_to_string("inputs/day6.txt").unwrap();
    assert_eq!(part2(&input), Part2::default())
}
