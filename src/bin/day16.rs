use itertools::Itertools;
use nom::{
    bytes::complete::take,
    multi::{many1, many_m_n},
    IResult,
};
fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"A0016C880162017C3686B18A3D4780"#;

type Part1 = usize;

enum Packet {
    Literal {
        packet_version: u8,
        // packet type 4
        groups: Vec<u8>,
    },
    Operator {
        packet_version: u8,
        packet_type: Operator,
        subpackets: Vec<Packet>,
    },
}
enum Operator {
    Sum,
    Product,
    Min,
    Max,
    GT,
    LT,
    Equal,
}
impl Operator {
    fn from_slice(i: &[u8]) -> Self {
        match i {
            [0, 0, 0] => Self::Sum,
            [0, 0, 1] => Self::Product,
            [0, 1, 0] => Self::Min,
            [0, 1, 1] => Self::Max,
            [1, 0, 1] => Self::GT,
            [1, 1, 0] => Self::LT,
            [1, 1, 1] => Self::Equal,
            _ => panic! {"hello"},
        }
    }
}
fn packet(i: &[u8]) -> IResult<&[u8], Packet> {
    let (rem, version) = take(3_u8)(i)?;
    let version = u8::from_str_radix(
        &version.iter().map(|d| d.to_string()).collect::<String>(),
        2,
    )
    .unwrap();
    let (rem, packet_type) = take(3_u8)(rem)?;
    let (ret, rem) = if packet_type == &vec![1, 0, 0] {
        // it's a literal
        let (groups, rem) = {
            let mut groups = vec![];
            let mut rem = rem;
            while !rem.is_empty() && rem[0] == 1 {
                let (orem, group) = take(5_u8)(rem)?;
                groups.extend_from_slice(&group[1..]);
                rem = orem;
            }
            let (orem, group) = take(5_u8)(rem)?;
            rem = orem;
            groups.extend_from_slice(&group[1..]);
            (groups, rem)
        };
        (
            Packet::Literal {
                packet_version: version,
                groups,
            },
            rem,
        )
    } else {
        // it's an operator
        let (rem, len_type) = take(1_u8)(rem)?;

        let len_type = if len_type[0] == 0 {
            LengthTypeId::TotalLength
        } else {
            LengthTypeId::NumberSubPackets
        };
        let (rem, subpackets) = match len_type {
            LengthTypeId::NumberSubPackets => {
                let (rem, number_sub) = take(11_u8)(rem)?;
                let number_sub = u16::from_str_radix(
                    &number_sub.iter().map(|d| d.to_string()).collect::<String>(),
                    2,
                )
                .unwrap();
                many_m_n(number_sub.into(), number_sub.into(), packet)(rem)?
            }
            LengthTypeId::TotalLength => {
                let (rem, length) = take(15_u8)(rem)?;
                let length = u16::from_str_radix(
                    &length.iter().map(|d| d.to_string()).collect::<String>(),
                    2,
                )
                .unwrap();
                let (rem, bits) = take(length)(rem)?;
                let (empty_rem, ret) = many1(packet)(bits)?;
                debug_assert!(empty_rem.is_empty());
                (rem, ret)
            }
        };
        (
            Packet::Operator {
                packet_version: version,
                packet_type: Operator::from_slice(packet_type),
                subpackets,
            },
            rem,
        )
    };
    Ok((rem, ret))
}
impl Packet {
    fn value(&self) -> usize {
        use Packet::*;
        match self {
            Literal { groups, .. } => {
                usize::from_str_radix(&groups.iter().map(|d| d.to_string()).collect::<String>(), 2)
                    .unwrap()
            }
            Operator {
                packet_type,
                subpackets,
                ..
            } => {
                use self::Operator::*;
                match packet_type {
                    Sum => subpackets.iter().map(|sp| sp.value()).sum(),
                    Product => subpackets.iter().map(|sp| sp.value()).product(),
                    Min => subpackets.iter().map(|sp| sp.value()).min().unwrap(),
                    Max => subpackets.iter().map(|sp| sp.value()).max().unwrap(),
                    GT => {
                        if subpackets[0].value() > subpackets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    LT => {
                        if subpackets[0].value() < subpackets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                    Equal => {
                        if subpackets[0].value() == subpackets[1].value() {
                            1
                        } else {
                            0
                        }
                    }
                }
            }
        }
    }
    fn version_number(&self) -> usize {
        use Packet::*;
        match self {
            Literal { packet_version, .. } => *packet_version as usize,
            Operator {
                packet_version,
                subpackets,
                ..
            } => {
                *packet_version as usize
                    + subpackets
                        .iter()
                        .map(|sp| sp.version_number())
                        .sum::<usize>()
            }
        }
    }
}
enum LengthTypeId {
    TotalLength,
    NumberSubPackets,
}

fn parse(input: &str) -> Vec<u8> {
    input
        .bytes()
        .map(|b| if b >= b'A' { b - b'A' + 10 } else { b - b'0' })
        .collect()
}

fn to_bits(input: &[u8]) -> Vec<u8> {
    input
        .iter()
        .map(|b| format!("{:04b}", b))
        .collect::<String>()
        .as_bytes()
        .iter()
        .map(|b| b - b'0')
        .collect_vec()
}

#[test]
fn simple_to_bits() {
    let i = "D2FE28";
    assert_eq!(
        to_bits(&parse(i)),
        vec![1, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0,]
    )
}

fn part1(input: &str) -> Part1 {
    let input = to_bits(&parse(input));
    let (_, packets): (_, Vec<Packet>) = many1(packet)(&input).unwrap();
    packets.into_iter().map(|p| p.version_number()).sum()
}

#[test]
fn tpart1_sample1() {
    assert_eq!(part1("D2FE28"), 6)
}
#[test]
fn tpart1_sample2() {
    assert_eq!(part1("38006F45291200"), 1 + 6 + 2)
}
#[test]
fn tpart1_sample3() {
    assert_eq!(part1("EE00D40C823060"), 7 + 2 + 4 + 1)
}
#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 31)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day16.txt").unwrap();
    assert_eq!(part1(&input), 873)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let input = to_bits(&parse(input));
    let (_, packets): (_, Vec<Packet>) = many1(packet)(&input).unwrap();
    debug_assert!(packets.len() == 1);
    packets[0].value()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2("CE00C43D881120"), 9)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day16.txt").unwrap();
    assert_eq!(part2(&input), 402817863665)
}
