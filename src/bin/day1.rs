fn main() {
    println!("Hello, world!");
}

fn part1(input: &str) -> usize {
    let list = input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    list.windows(2).filter(|w| w[0] < w[1]).count()
}

fn part2(input: &str) -> usize {
    let list = input
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    list.windows(3)
        .map(|s| s.iter().sum::<u32>())
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

#[test]
fn tpart1_sample() {
    let input = r#"199
200
208
210
200
207
240
269
260
263"#;
    assert_eq!(part1(&input), 7)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    assert_eq!(part1(&input), 1583)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day1.txt").unwrap();
    assert_ne!(1706, part2(&input));
    assert_eq!(1627, part2(&input));
}
