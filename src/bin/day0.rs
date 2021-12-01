fn main() {
    println!("Hello, world!");
}

const sample: &'static str = r#"1721
979
366
299
675
1456"#;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

fn part1(input: &str) -> Option<u32> {
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

fn part2(input: &str) -> Option<u32> {
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
    let input = sample;
    assert_eq!(part1(&input), Some(514579))
}

#[test]
fn tpart2_sample() {
    let input = sample;
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
