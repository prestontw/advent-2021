use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {
    println!("Hello, world!");
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Node {
    Start,
    End,
    Small(String),
    Big(String),
}
impl Default for Node {
    fn default() -> Self {
        Self::Small("".into())
    }
}
impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "start" => Self::Start,
            "end" => Self::End,
            other => {
                if other.chars().next().unwrap().is_uppercase() {
                    Self::Big(other.to_string())
                } else {
                    Self::Small(other.to_string())
                }
            }
        })
    }
}
const SAMPLE: &str = r#"start-A
start-b
A-c
A-b
b-d
A-end
b-end"#;

type Part1 = usize;

fn parse(input: &str) -> HashMap<Node, HashSet<Node>> {
    use advent_2021::regex;
    let re = regex!(r"(\w+)-(\w+)");
    let mut ret = HashMap::new();
    re.captures_iter(input)
        .map(|capture| (capture[1].parse().unwrap(), capture[2].parse().unwrap()))
        .for_each(|(l, r): (Node, Node)| {
            let ends = ret.entry(l.clone()).or_insert_with(HashSet::new);
            ends.insert(r.clone());
            let start = ret.entry(r).or_insert_with(HashSet::new);
            start.insert(l);
        });
    ret
}

fn part1(input: &str) -> Part1 {
    let input = parse(input);

    fn dfs(
        current: &Node,
        path: &Vec<Node>,
        visited: &HashSet<Node>,
        g: &HashMap<Node, HashSet<Node>>,
    ) -> Vec<Vec<Node>> {
        let unvisited_neighbors = g[current].difference(visited);
        if matches!(current, &Node::End) {
            return vec![path.clone()];
        }
        let mut ret = vec![];
        for neighbor in unvisited_neighbors {
            let mut newp = path.clone();
            newp.push(neighbor.clone());
            if matches!(neighbor, &Node::Small(_)) {
                let mut newv = visited.clone();
                newv.insert(neighbor.clone());
                ret.append(&mut dfs(neighbor, &newp, &newv, g));
            } else {
                ret.append(&mut dfs(&neighbor, &newp, visited, g));
            };
        }
        ret
    }
    dfs(
        &Node::Start,
        &vec![],
        &HashSet::from_iter(vec![Node::Start]),
        &input,
    )
    .into_iter()
    .filter(|v| v.last() == Some(&Node::End))
    .count()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 10)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day12.txt").unwrap();
    assert_eq!(part1(&input), 4167)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let input = parse(input);

    fn dfs(
        current: &Node,
        path: &Vec<Node>,
        visited: &HashMap<Node, u8>,
        g: &HashMap<Node, HashSet<Node>>,
    ) -> HashSet<Vec<Node>> {
        let potential_duplicates = visited.values().any(|v| v >= &2);
        let min_value = if potential_duplicates { 1 } else { 2 };
        let unvisited_neighbors = g[current]
            .iter()
            .filter(|n| visited.get(n).unwrap_or(&0) < &min_value);
        if matches!(current, &Node::End) {
            return HashSet::from_iter(vec![path.clone()]);
        }
        let mut ret = HashSet::new();
        for neighbor in unvisited_neighbors {
            if matches!(neighbor, &Node::Start) {
                continue;
            }
            let mut newp = path.clone();
            newp.push(neighbor.clone());
            if matches!(neighbor, &Node::Small(_)) {
                let mut newv = visited.clone();
                let count = newv.entry(neighbor.clone()).or_default();
                *count += 1;
                ret.extend(dfs(neighbor, &newp, &newv, g));
            } else {
                ret.extend(dfs(&neighbor, &newp, visited, g));
            };
        }
        ret
    }
    let resutl = dfs(&Node::Start, &vec![], &HashMap::new(), &input);
    resutl
        .into_iter()
        .filter(|v| v.last() == Some(&Node::End))
        .count()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 36)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day12.txt").unwrap();
    assert_eq!(part2(&input), 98441)
}
