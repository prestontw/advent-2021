use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

type Part1 = usize;

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn part1(input: &str) -> Part1 {
    let grid = parse(input);
    let mut costs = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    costs[0][0] = 0;

    fn inner(
        x: usize,
        y: usize,
        grid: &Vec<Vec<usize>>,
        costs: &mut Vec<Vec<usize>>,
        queue: &mut Vec<(usize, usize)>,
    ) {
        if costs.get(x).and_then(|row| row.get(y)).is_none() {
            return;
        }
        if (x, y) == (0, 0) {
            return;
        }
        let entering_cost = grid[x][y];
        let min = if x == 0 {
            costs[x][y - 1]
                .min(
                    *costs
                        .get(x + 1)
                        .and_then(|row| row.get(y))
                        .unwrap_or(&usize::MAX),
                )
                .min(*costs[x].get(y + 1).unwrap_or(&usize::MAX))
        } else if y == 0 {
            costs[x - 1][y]
                .min(
                    *costs
                        .get(x + 1)
                        .and_then(|row| row.get(y))
                        .unwrap_or(&usize::MAX),
                )
                .min(*costs[x].get(y + 1).unwrap_or(&usize::MAX))
        } else {
            costs[x - 1][y]
                .min(costs[x][y - 1])
                .min(
                    *costs
                        .get(x + 1)
                        .and_then(|row| row.get(y))
                        .unwrap_or(&usize::MAX),
                )
                .min(*costs[x].get(y + 1).unwrap_or(&usize::MAX))
        };

        if min + entering_cost < costs[x][y] {
            costs[x][y] = min + entering_cost;
            queue.push((x + 1, y));
            queue.push((x, y + 1));
            if x > 0 {
                queue.push((x - 1, y))
            }
            if y > 0 {
                queue.push((x, y - 1))
            }
        }
    }
    let mut queue = vec![(0, 1), (1, 0)];
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        inner(x, y, &grid, &mut costs, &mut queue);
    }
    costs[costs.len() - 1][costs[0].len() - 1]
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 40)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day15.txt").unwrap();
    assert_ne!(part1(&input), 506);
    assert_eq!(part1(&input), 498)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn expand(initial: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut expanse = vec![vec![usize::MAX; initial[0].len() * 5]; initial.len() * 5];
    // fill out the rest of the top rows
    for (row_i, col_i) in (0..initial.len()).cartesian_product(0..initial[0].len() * 5) {
        // x is the same, y is the thing that is multiplied
        let divisor = initial[0].len();
        let offset = col_i % divisor;
        let reference = col_i / divisor;
        let new_value = initial[row_i][offset] + reference;
        let new_value = if new_value > 9 {
            new_value - 9
        } else {
            new_value
        };
        expanse[row_i][col_i] = new_value;
    }
    // now fill out the four remaining mega-rows
    for row_i in initial.len()..initial.len() * 5 {
        for col_i in 0..initial[0].len() * 5 {
            // y is the same, x is the thing that is multiplied
            let new_value = expanse[row_i - initial.len()][col_i] + 1;
            let new_value = if new_value > 9 {
                new_value - 9
            } else {
                new_value
            };
            expanse[row_i][col_i] = new_value;
        }
    }
    expanse
}
#[test]
fn test_expanding() {
    let simple = vec![vec![8]];
    assert_eq!(
        expand(&simple),
        vec![
            vec![8, 9, 1, 2, 3],
            vec![9, 1, 2, 3, 4],
            vec![1, 2, 3, 4, 5],
            vec![2, 3, 4, 5, 6],
            vec![3, 4, 5, 6, 7]
        ]
    )
}
fn part2(input: &str) -> Part2 {
    let grid = {
        let initial = parse(input);
        expand(&initial)
    };
    let mut costs = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    costs[0][0] = 0;

    fn inner(
        x: usize,
        y: usize,
        grid: &Vec<Vec<usize>>,
        costs: &mut Vec<Vec<usize>>,
        queue: &mut Vec<(usize, usize)>,
    ) {
        if costs.get(x).and_then(|row| row.get(y)).is_none() {
            return;
        }
        if (x, y) == (0, 0) {
            return;
        }
        let entering_cost = grid[x][y];
        let min = if x == 0 {
            costs[x][y - 1]
                .min(
                    *costs
                        .get(x + 1)
                        .and_then(|row| row.get(y))
                        .unwrap_or(&usize::MAX),
                )
                .min(*costs[x].get(y + 1).unwrap_or(&usize::MAX))
        } else if y == 0 {
            costs[x - 1][y]
                .min(
                    *costs
                        .get(x + 1)
                        .and_then(|row| row.get(y))
                        .unwrap_or(&usize::MAX),
                )
                .min(*costs[x].get(y + 1).unwrap_or(&usize::MAX))
        } else {
            costs[x - 1][y]
                .min(costs[x][y - 1])
                .min(
                    *costs
                        .get(x + 1)
                        .and_then(|row| row.get(y))
                        .unwrap_or(&usize::MAX),
                )
                .min(*costs[x].get(y + 1).unwrap_or(&usize::MAX))
        };

        if min + entering_cost < costs[x][y] {
            costs[x][y] = min + entering_cost;
            queue.push((x + 1, y));
            queue.push((x, y + 1));
            if x > 0 {
                queue.push((x - 1, y))
            }
            if y > 0 {
                queue.push((x, y - 1))
            }
        }
    }
    let mut queue = vec![(0, 1), (1, 0)];
    while !queue.is_empty() {
        let (x, y) = queue.pop().unwrap();
        inner(x, y, &grid, &mut costs, &mut queue);
    }
    costs[costs.len() - 1][costs[0].len() - 1]
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 315)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day15.txt").unwrap();
    assert_ne!(part2(&input), 1712);
    assert_eq!(part2(&input), 2901)
}
