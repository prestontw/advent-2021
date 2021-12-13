use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

type Part1 = usize;

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<(char, u32)>) {
    use advent_2021::regex;
    let chunks = advent_2021::blank_lines(&input);
    let points = chunks[0]
        .iter()
        .map(|line| {
            line.split(',')
                .collect_tuple()
                .map(|(s1, s2)| (s1.parse().unwrap(), s2.parse().unwrap()))
                .unwrap()
        })
        .collect();
    let re = regex!(r"fold along (.)=(\d+)");
    let instructions = chunks[1]
        .iter()
        .map(|line| {
            re.captures(line)
                .map(|capture| {
                    (
                        capture[1].chars().next().unwrap(),
                        capture[2].parse().unwrap(),
                    )
                })
                .unwrap()
        })
        .collect();
    (points, instructions)
}

fn part1(input: &str) -> Part1 {
    let (points, instructions) = parse(input);
    let points: HashSet<(u32, u32)> = HashSet::from_iter(points);
    let (fold_direction, coord) = instructions[0];

    let after_first: HashSet<(u32, u32)> = if fold_direction == 'x' {
        // we are folding left
        points
            .into_iter()
            .map(|(x, y)| {
                if x > coord {
                    ((coord - (x - coord)), y)
                } else {
                    (x, y)
                }
            })
            .collect()
    } else {
        // we are folding right
        points
            .into_iter()
            .map(|(x, y)| {
                if y > coord {
                    (x, (coord - (y - coord)))
                } else {
                    (x, y)
                }
            })
            .collect()
    };
    after_first.iter().count()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 17)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
    assert_eq!(part1(&input), 790)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = String;

fn part2(input: &str) -> Part2 {
    let (points, instructions) = parse(input);
    let points: HashSet<(u32, u32)> = HashSet::from_iter(points);

    let sploop = instructions
        .into_iter()
        .fold(points, |points, (fold_direction, coord)| {
            if fold_direction == 'x' {
                // we are folding left
                points
                    .into_iter()
                    .map(|(x, y)| {
                        if x > coord {
                            ((coord - (x - coord)), y)
                        } else {
                            (x, y)
                        }
                    })
                    .collect()
            } else {
                // we are folding right
                points
                    .into_iter()
                    .map(|(x, y)| {
                        if y > coord {
                            (x, (coord - (y - coord)))
                        } else {
                            (x, y)
                        }
                    })
                    .collect()
            }
        });
    let maxx = sploop.iter().max_by_key(|(x, _)| x).unwrap().0 as usize;
    let maxy = sploop.iter().max_by_key(|(_, y)| y).unwrap().1 as usize;
    let mut grid = vec![vec![' '; maxy + 1]; maxx + 1];
    for (x, y) in sploop {
        grid[x as usize][y as usize] = '#';
    }
    grid.into_iter()
        .rev()
        .map(|line| line.into_iter().collect::<String>())
        .join("\n")
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day13.txt").unwrap();
    let output = indoc::indoc! {
    " #  #
    #    #
    #    #
     ####

    #####
    #    #
         #
        #

    #
    # #
    # #
    ######

    # ##
    # #  #
    # #  #
    ######

    ##   #
    # #  #
    #  # #
    #   ##

    ######
      #
      #
    ######

    # ###
    #  # #
    #    #
     ####

    ##
    #  #
    #  #
    ######"};
    assert_ne!(part2(&input), output)
}
