use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Octopus {
    Flashed,
    Count(u32),
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Actions {
    Increment(usize, usize),
    Flash,
}
struct Grid {
    inner: [[Octopus; 10]; 10],
}
impl Grid {
    pub fn new(input: &str) -> Self {
        let mut initial = [[Octopus::Flashed; 10]; 10];
        for (li, line) in input.lines().enumerate() {
            for (ci, char) in line.char_indices() {
                initial[li][ci] = Octopus::Count(char.to_digit(10).unwrap());
            }
        }
        Grid { inner: initial }
    }
    pub fn next_generation(&mut self) -> Vec<Actions> {
        let mut starting_actions = (0..10)
            .cartesian_product(0..10)
            .map(|(one, two)| Actions::Increment(one, two))
            .collect_vec();
        let ret = self.inner(&mut starting_actions, vec![]);
        for i in 0..10 {
            for j in 0..10 {
                if self.inner[i][j] == Octopus::Flashed {
                    self.inner[i][j] = Octopus::Count(0)
                }
            }
        }
        ret
    }

    fn inner(&mut self, queue: &mut Vec<Actions>, mut actions: Vec<Actions>) -> Vec<Actions> {
        if queue.is_empty() {
            return actions;
        }
        let current_action = queue.pop().unwrap();
        match current_action {
            Actions::Flash => {
                actions.push(current_action);
            }
            Actions::Increment(ri, ci) => match &mut self.inner[ri][ci] {
                Octopus::Flashed => (),
                Octopus::Count(current) => {
                    *current += 1;
                    if current > &mut 9 {
                        self.inner[ri][ci] = Octopus::Flashed;
                        queue.push(Actions::Flash);
                        for (neighbor_x, neighbor_y) in neighbors(ri, ci) {
                            queue.push(Actions::Increment(neighbor_x, neighbor_y))
                        }
                    }
                }
            },
        }
        self.inner(queue, actions)
    }
}
type Part1 = usize;

fn neighbors(row_index: usize, col_index: usize) -> Vec<(usize, usize)> {
    let row_start = if row_index == 0 { 0 } else { row_index - 1 };
    let row_end = if row_index == 9 { 9 } else { row_index + 1 };
    let col_start = if col_index == 0 { 0 } else { col_index - 1 };
    let col_end = if col_index == 9 { 9 } else { col_index + 1 };
    (row_start..=row_end)
        .cartesian_product(col_start..=col_end)
        .filter(|(x, y)| x != &row_index || y != &col_index)
        .collect()
}
#[test]
fn test_neighbors() {
    assert_eq!(neighbors(0, 0), vec![(0, 1), (1, 0), (1, 1)]);
}

fn part1(input: &str) -> Part1 {
    let mut grid = Grid::new(input);
    (0..100)
        .flat_map(|_| grid.next_generation())
        .filter(|action| action == &Actions::Flash)
        .count()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 1656)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day11.txt").unwrap();
    assert_eq!(part1(&input), 1625)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let mut grid = Grid::new(input);
    (1..)
        .find(|_day| grid.next_generation().len() == 100)
        .unwrap()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 195)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day11.txt").unwrap();
    assert_eq!(part2(&input), 244)
}
