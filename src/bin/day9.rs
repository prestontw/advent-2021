use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

type Part1 = usize;

fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

const DIFFS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
fn part1(input: &str) -> Part1 {
    let input = parse(input);
    let mut mins = vec![];
    for (row_index, row) in input.iter().enumerate() {
        for (col_index, cell_value) in row.iter().enumerate() {
            let adjacent = |row_diff: &i32, col_diff: &i32| {
                let nr: usize = ((row_index as i32) + row_diff).try_into().ok()?;
                let nc: usize = ((col_index as i32) + col_diff).try_into().ok()?;
                input.get(nr).and_then(|row: &Vec<_>| row.get(nc))
            };
            let mut neighbors = DIFFS
                .iter()
                .filter_map(|(row_diff, col_diff)| adjacent(row_diff, col_diff));
            if neighbors.all(|adj_value| adj_value > cell_value) {
                mins.push(*cell_value as usize);
            }
        }
    }
    let len = mins.len();
    len + mins.into_iter().sum::<usize>()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 5 + 5 + 1 + 0 + 1 * 4)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day9.txt").unwrap();
    assert_eq!(part1(&input), 489)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

/// suffers if this is part of a cluster but the left most point, and not on the top row
fn part2(input: &str) -> Part2 {
    let input = parse(input);
    let mut basins = {
        let mut visited: HashMap<(_, _), usize> = HashMap::new();
        let mut stack: Vec<(_, _)> = (0..input.len())
            .cartesian_product(0..input[0].len())
            .collect();
        let mut initial = vec![vec![None; input[0].len()]; input.len()];
        let mut basin_number = 0;
        while !stack.is_empty() {
            let pos = stack.pop().unwrap();
            if visited.contains_key(&pos) || input[pos.0][pos.1] == 9 {
                continue;
            }
            let adjacent = |row_diff: &i32, col_diff: &i32| {
                let nr: usize = ((pos.0 as i32) + row_diff).try_into().ok()?;
                let nc: usize = ((pos.1 as i32) + col_diff).try_into().ok()?;
                input
                    .get(nr)
                    .and_then(|row: &Vec<_>| row.get(nc))
                    .map(|_| (nr, nc))
            };
            let mut neighbors: Vec<_> = DIFFS
                .iter()
                .filter_map(|(row_diff, col_diff)| adjacent(row_diff, col_diff))
                .collect();
            if let Some(index) = neighbors
                .iter()
                .filter_map(|(ri, ci)| initial[*ri][*ci])
                .next()
            {
                initial[pos.0][pos.1] = Some(index);
                visited.insert(pos, index);
            } else {
                basin_number += 1;
                initial[pos.0][pos.1] = Some(basin_number);
                visited.insert(pos, basin_number);
            }
            stack.append(&mut neighbors);
        }

        visited.into_values().counts().into_values().collect_vec()
    };
    basins.sort();
    basins.into_iter().rev().take(3).product()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 9 * 14 * 9)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day9.txt").unwrap();
    assert_eq!(part2(&input), 1056330)
}
