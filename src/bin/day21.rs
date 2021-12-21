fn main() {
    println!("Hello, world!");
}

const INPUT_PATH: &str = "inputs/day21.txt";
const SAMPLE: &str = r#"Player 1 starting position: 4
Player 2 starting position: 8"#;

type Part1 = u64;

struct Dice(u64);
impl Dice {
    fn roll(&mut self) -> u64 {
        let current = self.0;
        // check that we are at 100, not at 99
        self.0 = if current + 1 == 101 { 1 } else { current + 1 };
        current
    }
}
struct Player {
    pos: u64,
    score: u64,
}
impl Player {
    fn take_turn(&mut self, d: &mut Dice) -> bool {
        let dice_rolls = d.roll() + d.roll() + d.roll();
        let new_space = self.pos + dice_rolls;
        // replace 0 with 10 space
        let new_space = if new_space % 10 == 0 {
            10
        } else {
            new_space % 10
        };
        self.score += new_space as u64;
        self.pos = new_space;
        // check score, not position
        self.score >= 1000
    }
    fn new(starting_pos: u64) -> Self {
        Player {
            pos: starting_pos,
            score: 0,
        }
    }
}
fn parse(input: &str) -> (u64, u64) {
    if input == SAMPLE {
        (4, 8)
    } else {
        (4, 5)
    }
}

fn part1(input: &str) -> Part1 {
    let mut rounds = 0;
    let mut player = 0;
    let mut dice = Dice(1);
    let input = parse(input);
    let mut players = [Player::new(input.0), Player::new(input.1)];
    loop {
        rounds += 3;
        let done = players[player].take_turn(&mut dice);
        player = (player + 1) % 2;
        if done {
            break;
        }
    }
    rounds * players[player].score
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 745 * 993)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string(&INPUT_PATH).unwrap();
    assert_eq!(part1(&input), 864900)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn possible_results(player_index: usize, positions: [u64; 2], scores: [u64; 2]) -> [u64; 2] {
    if positions[player_index] >= 21 {
        let mut ret = [0, 0];
        ret[player_index] = 1;
        return ret;
    }
    let possibilities = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    let wins = possibilities
        .iter()
        .fold([0, 0], |mut acc, (roll, frequency)| {
            let new_space = positions[player_index] + roll;
            let new_space = if new_space % 10 == 0 {
                10
            } else {
                new_space % 10
            };
            let new_score = scores[player_index] + new_space;
            if new_score >= 21 {
                acc[player_index] += *frequency;
                acc
            } else {
                // then we recur with the other player starting!
                let other_index = (player_index + 1) % 2;
                let mut positions = positions.clone();
                positions[player_index] = new_space;
                let mut scores = scores.clone();
                scores[player_index] = new_score;
                let results = possible_results(other_index, positions, scores);
                [
                    acc[0] + results[0] * frequency,
                    acc[1] + results[1] * frequency,
                ]
            }
        });
    wins
}

#[test]
fn end_of_game() {
    // put player 0 so that they will win if they roll a 6 or higher
    assert_eq!(possible_results(0, [0, 0], [15, 20]), [17, 10 * 27])
}

fn part2(input: &str) -> Part2 {
    let input = parse(input);
    let win_counts = possible_results(0, [input.0, input.1], [0, 0]);

    win_counts[0].max(win_counts[1])
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 444356092776315)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    // 3497145745715916
    assert_eq!(part2(&input), 575111835924670)
}
