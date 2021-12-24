use advent_2021::blank_lines;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const INPUT_PATH: &str = "inputs/day24.txt";

type Part1 = Vec<i64>;
enum Symbol<'a> {
    Num(i64),
    Sym(&'a str),
}
impl<'a> Symbol<'a> {
    fn value(&self, registers: &HashMap<&'a str, i64>) -> i64 {
        match self {
            Symbol::Num(ret) => *ret,
            Symbol::Sym(reg) => registers[reg],
        }
    }
}
enum Instructions<'a> {
    Input(&'a str),
    Add(&'a str, Symbol<'a>),
    Mul(&'a str, Symbol<'a>),
    Div(&'a str, Symbol<'a>),
    Mod(&'a str, Symbol<'a>),
    Eql(&'a str, Symbol<'a>),
}

impl<'a> Instructions<'a> {
    fn from_str(s: &'a str) -> Result<Self, ()> {
        let input = s.split(' ').collect_vec();
        use Instructions::*;
        let s = if input.len() > 2 {
            if input[2].chars().all(|c| c.is_alphabetic()) {
                Symbol::Sym(input[2])
            } else {
                Symbol::Num(input[2].parse().unwrap())
            }
        } else {
            Symbol::Num(0)
        };
        match input[0] {
            "inp" => Ok(Input(input[1])),
            "add" => Ok(Add(input[1], s)),
            "mul" => Ok(Mul(input[1], s)),
            "div" => Ok(Div(input[1], s)),
            "mod" => Ok(Mod(input[1], s)),
            "eql" => Ok(Eql(input[1], s)),
            _ => Err(()),
        }
    }
}

struct Alu<'a> {
    registers: HashMap<&'a str, i64>,
}

impl<'a> Alu<'a> {
    fn new_with_z(z: i64) -> Self {
        Alu {
            registers: vec![("w", 0), ("x", 0), ("y", 0), ("z", z)]
                .into_iter()
                .collect(),
        }
    }
    fn golden_inputs(memo: &HashMap<(usize, i64), Vec<(u8, i64)>>) -> Vec<i64> {
        let starting_point = (13, 0);
        fn inner(
            (round, z_state): (usize, i64),
            reverse_lookup: &HashMap<(usize, i64), Vec<(u8, i64)>>,
        ) -> Vec<i64> {
            if round == 0 {
                return reverse_lookup
                    .get(&(0, z_state))
                    .map(|v| v.iter().map(|(digit, _zstate)| *digit as i64))
                    .unwrap()
                    .collect_vec();
            }
            let previous_points = reverse_lookup.get(&(round, z_state)).unwrap();
            previous_points
                .iter()
                .flat_map(|(digit, previous_z_state)| {
                    let others = inner((round - 1, *previous_z_state), reverse_lookup);
                    others.into_iter().map(|j| j * 10 + *digit as i64)
                })
                .collect_vec()
        }
        inner(starting_point, &memo)
    }
    fn golden_path(
        instructions: &[Vec<Instructions<'a>>],
    ) -> HashMap<(usize, i64), Vec<(u8, i64)>> {
        let mut memo: HashMap<(usize, i64), Vec<(u8, i64)>> = HashMap::new();
        let mut z_possibilities = vec![0].into_iter().collect::<HashSet<_>>();
        for (count, block) in instructions.iter().enumerate() {
            let mut new_possibilities = HashSet::new();

            for z_value in z_possibilities {
                for input in 1_u8..=9 {
                    let mut alu = Alu::new_with_z(z_value);
                    let input_i = std::iter::once(input as i64);
                    alu.run(&block, input_i);
                    let resulting_z = alu.registers["z"];
                    memo.entry((count, resulting_z))
                        .or_insert(vec![])
                        .push((input, z_value));
                    new_possibilities.insert(resulting_z);
                }
            }

            z_possibilities = new_possibilities;
        }
        memo
    }

    fn run<T>(&mut self, instructions: &[Instructions<'a>], input: T)
    where
        T: IntoIterator<Item = i64>,
    {
        let mut input = input.into_iter();
        use Instructions::*;
        for instruction in instructions {
            match instruction {
                Input(dest) => {
                    let v = input.next().unwrap();
                    self.registers.insert(dest, v);
                }
                Add(s, d) => {
                    let d = d.value(&self.registers);
                    let sum = self.registers[s] + d;
                    self.registers.insert(s, sum);
                }
                Mul(s, d) => {
                    let d = d.value(&self.registers);
                    let sum = self.registers[s] * d;
                    self.registers.insert(s, sum);
                }
                Div(s, d) => {
                    let d = d.value(&self.registers);
                    let sum = self.registers[s] / d;
                    self.registers.insert(s, sum);
                }
                Mod(s, d) => {
                    let d = d.value(&self.registers);
                    let sum = self.registers[s] % d;
                    self.registers.insert(s, sum);
                }
                Eql(s, d) => {
                    let d = d.value(&self.registers);
                    let sum = self.registers[s] == d;
                    self.registers.insert(s, if sum { 1 } else { 0 });
                }
            }
        }
    }
}

fn parse(input: &str) -> Vec<Vec<Instructions>> {
    blank_lines(input)
        .into_iter()
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|line| Instructions::from_str(line))
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        })
        .collect()
}

fn part1(input: &str) -> Part1 {
    let instructions = parse(input);
    let memo = Alu::golden_path(&instructions);
    let mut inputs = Alu::golden_inputs(&memo);
    inputs.sort();
    inputs
}

#[test]
fn tpart1_and_2() {
    let input = std::fs::read_to_string(&INPUT_PATH).unwrap();
    let result = part1(&input);
    assert_eq!(result[result.len() - 1], 79197919993985);
    assert_eq!(result[0], 13191913571211)
}
