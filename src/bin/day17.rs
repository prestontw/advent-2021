use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

type Part1 = (i64, i64, i64);

// target area: x=119..176, y=-141..-84

#[derive(Clone, Copy)]
struct ProbePositions {
    x_pos: i64,
    y_pos: i64,
    x_vel: i64,
    y_vel: i64,
    bottom_left: (i64, i64),
    upper_right: (i64, i64),
}
impl ProbePositions {
    fn new(xvel: i64, yvel: i64, bottom_left: (i64, i64), upper_right: (i64, i64)) -> Self {
        ProbePositions {
            x_pos: 0,
            y_pos: 0,
            x_vel: xvel,
            y_vel: yvel,
            bottom_left,
            upper_right,
        }
    }
}
impl Iterator for ProbePositions {
    type Item = (i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y_pos < self.bottom_left.1 || self.x_pos > self.upper_right.0 {
            return None;
        }
        self.x_pos += self.x_vel;
        self.y_pos += self.y_vel;
        if self.x_vel < 0 {
            self.x_vel += 1;
        } else if self.x_vel > 0 {
            self.x_vel -= 1;
        }
        self.y_vel -= 1;
        Some((self.x_pos, self.y_pos))
    }
}

fn part1([bottom_left, upper_right]: [(i64, i64); 2]) -> Part1 {
    let initial_vels = (0..1000).cartesian_product(0..1000);
    let highest = initial_vels
        .filter_map(|(x_vel, y_vel)| {
            let p = ProbePositions::new(x_vel, y_vel, bottom_left, upper_right);
            if p.into_iter().any(|(x, y)| {
                x >= bottom_left.0 && x <= upper_right.0 && y >= bottom_left.1 && y <= upper_right.1
            }) {
                (p.into_iter().map(|(_x, y)| y)).max()
            } else {
                None
            }
        })
        .max()
        .unwrap();
    (0, 0, highest)
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1([(20, -10), (30, -5)]), (0, 0, 45))
}

#[test]
fn tpart1() {
    let input = [(119, -141), (176, -84)];
    assert_eq!(part1(input), (0, 0, 9870))
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = usize;

fn part2([bottom_left, upper_right]: [(i64, i64); 2]) -> Part2 {
    let initial_vels = (0..1000).cartesian_product(-1000..1000);
    initial_vels
        .filter_map(|(x_vel, y_vel)| {
            let p = ProbePositions::new(x_vel, y_vel, bottom_left, upper_right);
            if p.into_iter().any(|(x, y)| {
                x >= bottom_left.0 && x <= upper_right.0 && y >= bottom_left.1 && y <= upper_right.1
            }) {
                (p.into_iter().map(|(_x, y)| y)).max()
            } else {
                None
            }
        })
        .count()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2([(20, -10), (30, -5)]), 112)
}

#[test]
fn tpart2() {
    let input = [(119, -141), (176, -84)];
    assert_eq!(part2(input), 5523)
}
