fn main() {
    println!("Hello, world!");
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum SnailNumber {
    Number(usize),
    Pair(Box<SnailNumber>, Box<SnailNumber>),
}

impl SnailNumber {
    fn magnitude(&self) -> usize {
        match self {
            SnailNumber::Number(ret) => *ret,
            SnailNumber::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
    fn is_number(&self) -> bool {
        match self {
            &SnailNumber::Number(_) => true,
            _ => false,
        }
    }
    fn add(self, other: SnailNumber) -> Self {
        fn inner(sn: SnailNumber, other: SnailNumber) -> SnailNumber {
            SnailNumber::Pair(Box::new(sn), Box::new(other))
        }
        let mut n = inner(self, other);
        loop {
            match n.explode() {
                Ok(unchanged) => match unchanged.split() {
                    Ok(unchanged) => {
                        n = unchanged;
                        break;
                    }
                    Err(new) => {
                        n = new;
                    }
                },
                Err(new) => {
                    n = new;
                }
            }
        }
        n
    }
    fn split(self) -> Result<Self, Self> {
        match self {
            SnailNumber::Number(n) if n >= 10 => Err(SnailNumber::Pair(
                Box::new(SnailNumber::Number(n / 2)),
                Box::new(SnailNumber::Number((n + 1) / 2)),
            )),
            SnailNumber::Number(n) => Ok(SnailNumber::Number(n)),
            SnailNumber::Pair(l, r) => {
                let l = l.split();
                if let Ok(l) = l {
                    let r = r.split();
                    if let Ok(r) = r {
                        Ok(SnailNumber::Pair(l.into(), r.into()))
                    } else {
                        Err(SnailNumber::Pair(l.into(), r.unwrap_err().into()))
                    }
                } else {
                    Err(SnailNumber::Pair(l.unwrap_err().into(), r.into()))
                }
            }
        }
    }
    fn add_to_rightmost(self, val: usize) -> Self {
        match self {
            SnailNumber::Number(x) => SnailNumber::Number(x + val),
            SnailNumber::Pair(untouched_l, r) => {
                SnailNumber::Pair(untouched_l, r.add_to_rightmost(val).into())
            }
        }
    }
    fn add_to_leftmost(self, val: usize) -> Self {
        match self {
            SnailNumber::Number(x) => SnailNumber::Number(x + val),
            SnailNumber::Pair(l, untouched_r) => {
                SnailNumber::Pair(l.add_to_leftmost(val).into(), untouched_r)
            }
        }
    }
    fn explode(self) -> Result<Self, Self> {
        fn inner(
            sn: SnailNumber,
            depth: usize,
        ) -> Result<SnailNumber, (usize, SnailNumber, usize)> {
            use SnailNumber::*;
            match (sn, depth) {
                (Number(n), _) => Ok(Number(n)),
                (Pair(l, r), x) if x >= 4 && l.is_number() && r.is_number() => {
                    // it's time to explode!
                    match (*l, *r) {
                        (Number(l), Number(r)) => Err((l, Number(0), r)),
                        _ => panic!("we know this is a number"),
                    }
                }
                (Pair(l, r), d) => {
                    let l = inner(*l, d + 1);
                    match l {
                        Ok(sn) => {
                            let r = inner(*r, d + 1);
                            match r {
                                Ok(rsn) => Ok(Pair(sn.into(), rsn.into())),
                                Err((lval, rsn, rval)) => {
                                    // can add this lval to our lovely sn
                                    let newl = sn.add_to_rightmost(lval);
                                    let nn = Pair(newl.into(), rsn.into());
                                    // and return rval for later use
                                    Err((0, nn, rval))
                                }
                            }
                        }
                        Err((lval, lsn, rval)) => {
                            // can add rval to leftmost of r
                            let newr = r.add_to_leftmost(rval);
                            let nn = Pair(lsn.into(), newr.into());
                            Err((lval, nn, 0))
                        }
                    }
                }
            }
        }
        inner(self, 0).map_err(|(_l, sn, _r)| sn)
    }
}
#[test]
fn example_explode() {
    // from `[[[[[9,8],1],2],3],4] becomes [[[[0,9],2],3],4]`
    use SnailNumber::*;
    let initial = Pair(
        Pair(
            Pair(
                Pair(
                    Pair(Number(9).into(), Number(8).into()).into(),
                    Number(1).into(),
                )
                .into(),
                Number(2).into(),
            )
            .into(),
            Number(3).into(),
        )
        .into(),
        Number(4).into(),
    );
    let expected = Pair(
        Pair(
            Pair(
                Pair(Number(0).into(), Number(9).into()).into(),
                Number(2).into(),
            )
            .into(),
            Number(3).into(),
        )
        .into(),
        Number(4).into(),
    );
    assert_eq!(initial.explode(), Err(expected))
}

#[test]
fn example_reduce() {
    // [[[[4,3],4],4],[7,[[8,4],9]]] + [1,1]
    let result = parse("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]");
    let after = result
        .into_iter()
        .reduce(|acc, elem| acc.add(elem))
        .unwrap();
    assert_eq!(after, parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")[0])
}
const SAMPLE: &str = r#"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"#;

type Part1 = usize;

fn value_to_sn(v: &serde_json::Value) -> SnailNumber {
    use serde_json::Value;
    match v {
        Value::Number(n) => {
            SnailNumber::Number(n.as_u64().and_then(|n| n.try_into().ok()).unwrap())
        }
        Value::Array(a) => {
            SnailNumber::Pair(Box::new(value_to_sn(&a[0])), Box::new(value_to_sn(&a[1])))
        }
        _ => panic!("json error"),
    }
}
fn parse(input: &str) -> Vec<SnailNumber> {
    use serde_json::Value;
    input
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .map(|v: Value| value_to_sn(&v))
        .collect()
}

fn part1(input: &str) -> Part1 {
    let input = parse(input);
    input
        .into_iter()
        .reduce(|acc, element| acc.add(element))
        .map(|sn| sn.magnitude())
        .unwrap()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 4140)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day18.txt").unwrap();
    assert_eq!(part1(&input), 4469)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let input = parse(input);
    let mut amax = None;
    for i1 in &input {
        for i2 in &input {
            if i1 == i2 {
                continue;
            }
            let max = i1
                .clone()
                .add(i2.clone())
                .magnitude()
                .max(i2.clone().add(i1.clone()).magnitude());
            amax = amax
                .map(|current: usize| current.max(max))
                .or_else(|| Some(max));
        }
    }
    amax.unwrap()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 3993)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day18.txt").unwrap();
    assert_eq!(part2(&input), 4770)
}
