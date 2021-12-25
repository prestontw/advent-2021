use std::{cmp::Reverse, collections::HashMap};

use itertools::Itertools;

fn main() {
    println!("Hello, world!");
}

const INPUT_PATH: &str = "inputs/day23.txt";
const SAMPLE: &str = r#"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########"#;

type Part1 = usize;

fn parse(input: &str) -> Vec<(Amphipod, usize, usize)> {
    input
        .lines()
        .skip(2)
        .take(2)
        .enumerate()
        .flat_map(|(index, line)| {
            line.char_indices().filter_map(move |(col_pos, c)| {
                if c != '#' && c != ' ' {
                    Some((Amphipod::from_char(c), index, col_pos))
                } else {
                    None
                }
            })
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn cost(&self) -> usize {
        match self {
            Self::A => 1,
            Self::B => 10,
            Self::C => 100,
            Self::D => 1000,
        }
    }
    fn from_char(c: char) -> Self {
        match c {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            _ => panic!("unexpected char"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct HallwaySpace(Option<Amphipod>);

#[derive(Clone, PartialEq, Eq, Hash)]
struct Board {
    leftmost: HallwaySpace,
    leftish: HallwaySpace,
    ab: HallwaySpace,
    bc: HallwaySpace,
    cd: HallwaySpace,
    rightish: HallwaySpace,
    rightmost: HallwaySpace,
    aroom: Room,
    broom: Room,
    croom: Room,
    droom: Room,
}

impl Board {
    fn is_done(&self) -> bool {
        self.aroom.is_done(Amphipod::A)
            && self.broom.is_done(Amphipod::B)
            && self.croom.is_done(Amphipod::C)
            && self.droom.is_done(Amphipod::D)
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Room {
    spaces: Vec<Option<Amphipod>>,
}
// can_receive, done
impl Room {
    fn all_same_terrain(&self, expected: Amphipod) -> bool {
        self.spaces
            .iter()
            .filter_map(|space| space.as_ref())
            .all(|a| a == &expected)
    }
    fn receive(&self, expected: Amphipod) -> Option<(usize, Self)> {
        if self.all_same_terrain(expected) {
            let mut spaces_clone = self.spaces.clone();
            for index in (0..self.spaces.len()).rev() {
                if spaces_clone[index].is_none() {
                    spaces_clone[index] = Some(expected);
                    return Some((
                        index + 1,
                        Room {
                            spaces: spaces_clone,
                        },
                    ));
                }
            }
            None
        } else {
            None
        }
    }
    fn is_done(&self, expected: Amphipod) -> bool {
        self.all_same_terrain(expected)
            && self.spaces.iter().filter(|space| space.is_some()).count() == self.spaces.len()
    }
    fn send(&self, expected: Amphipod) -> Option<(usize, Amphipod, Self)> {
        if self.is_done(expected) {
            return None;
        }
        // send the first thing in our space, along with how many spaces it needs to move to get into the hallway
        let (index, a) = self.spaces.iter().enumerate().find(|(_, a)| a.is_some())?;
        let mut new_spaces = self.spaces.clone();
        new_spaces[index] = None;

        Some((index + 1, a.unwrap(), Self { spaces: new_spaces }))
    }
    fn estimate(&self, expected: Amphipod) -> usize {
        if self.is_done(expected) {
            0
        } else {
            self.spaces
                .iter()
                .filter_map(|a| a.map(|a| if a == expected { 0 } else { a.cost() }))
                .sum()
        }
    }
}

impl Board {
    fn estimate(&self) -> usize {
        let Board {
            aroom,
            broom,
            croom,
            droom,
            leftmost,
            rightmost,
            leftish,
            rightish,
            ab,
            bc,
            cd,
        } = self;
        aroom.estimate(Amphipod::A)
            + broom.estimate(Amphipod::B)
            + croom.estimate(Amphipod::C)
            + droom.estimate(Amphipod::D)
            + leftmost.0.map(|a| a.cost()).unwrap_or_default()
            + leftish.0.map(|a| a.cost()).unwrap_or_default()
            + ab.0.map(|a| a.cost()).unwrap_or_default()
            + bc.0.map(|a| a.cost()).unwrap_or_default()
            + cd.0.map(|a| a.cost()).unwrap_or_default()
            + rightish.0.map(|a| a.cost()).unwrap_or_default()
            + rightmost.0.map(|a| a.cost()).unwrap_or_default()
    }
    fn new(input: &[(Amphipod, usize, usize)]) -> Self {
        let mut aroom = vec![None; input.len() / 4];
        let mut broom = vec![None; input.len() / 4];
        let mut croom = vec![None; input.len() / 4];
        let mut droom = vec![None; input.len() / 4];

        for (a, row, col) in input {
            match col {
                3 => aroom[*row] = Some(*a),
                5 => broom[*row] = Some(*a),
                7 => croom[*row] = Some(*a),
                9 => droom[*row] = Some(*a),
                _ => panic!("parse error"),
            }
        }

        Board {
            aroom: Room { spaces: aroom },
            broom: Room { spaces: broom },
            croom: Room { spaces: croom },
            droom: Room { spaces: droom },
            leftmost: HallwaySpace(None),
            leftish: HallwaySpace(None),
            ab: HallwaySpace(None),
            bc: HallwaySpace(None),
            cd: HallwaySpace(None),
            rightish: HallwaySpace(None),
            rightmost: HallwaySpace(None),
        }
    }
    fn next(&self) -> Vec<(usize, Self)> {
        let mut ret = vec![];
        // check all of the things in the hallways!
        if let HallwaySpace(Some(a)) = &self.leftmost {
            // check if there is something in leftmost and if it can make it to its destination
            match a {
                Amphipod::A => {
                    if self.leftish.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.aroom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 2),
                                Board {
                                    leftmost: HallwaySpace(None),
                                    aroom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::B => {
                    if self.leftish.0.is_none() && self.ab.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.broom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 4),
                                Board {
                                    leftmost: HallwaySpace(None),
                                    broom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::C => {
                    if self.leftish.0.is_none() && self.ab.0.is_none() && self.bc.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.croom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 6),
                                Board {
                                    leftmost: HallwaySpace(None),
                                    croom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::D => {
                    if self.leftish.0.is_none()
                        && self.ab.0.is_none()
                        && self.bc.0.is_none()
                        && self.cd.0.is_none()
                    {
                        if let Some((spaces_to_end, new_room)) = self.droom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 8),
                                Board {
                                    leftmost: HallwaySpace(None),
                                    droom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
            }
        }
        if let HallwaySpace(Some(a)) = &self.leftish {
            // check if there is something in leftish space and if it can make it to its destination
            match a {
                Amphipod::A => {
                    if let Some((spaces_to_end, new_room)) = self.aroom.receive(*a) {
                        ret.push((
                            a.cost() * (spaces_to_end + 1),
                            Board {
                                leftish: HallwaySpace(None),
                                aroom: new_room,
                                ..self.clone()
                            },
                        ));
                    }
                }
                Amphipod::B => {
                    if self.ab.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.broom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 3),
                                Board {
                                    leftish: HallwaySpace(None),
                                    broom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::C => {
                    if self.ab.0.is_none() && self.bc.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.croom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 5),
                                Board {
                                    leftish: HallwaySpace(None),
                                    croom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::D => {
                    if self.ab.0.is_none() && self.bc.0.is_none() && self.cd.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.droom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 7),
                                Board {
                                    leftish: HallwaySpace(None),
                                    droom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
            }
        }
        if let HallwaySpace(Some(a)) = &self.ab {
            // check if there is something in leftish space and if it can make it to its destination
            match a {
                Amphipod::A => {
                    if let Some((spaces_to_end, new_room)) = self.aroom.receive(*a) {
                        ret.push((
                            a.cost() * (spaces_to_end + 1),
                            Board {
                                ab: HallwaySpace(None),
                                aroom: new_room,
                                ..self.clone()
                            },
                        ));
                    }
                }
                Amphipod::B => {
                    if let Some((spaces_to_end, new_room)) = self.broom.receive(*a) {
                        ret.push((
                            a.cost() * (spaces_to_end + 1),
                            Board {
                                ab: HallwaySpace(None),
                                broom: new_room,
                                ..self.clone()
                            },
                        ));
                    }
                }
                Amphipod::C => {
                    if self.bc.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.croom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 3),
                                Board {
                                    ab: HallwaySpace(None),
                                    croom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::D => {
                    if self.bc.0.is_none() && self.cd.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.droom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 5),
                                Board {
                                    ab: HallwaySpace(None),
                                    droom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
            }
        }
        if let HallwaySpace(Some(a)) = &self.bc {
            // check if there is something in leftish space and if it can make it to its destination
            match a {
                Amphipod::A => {
                    if self.ab.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.aroom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 3),
                                Board {
                                    bc: HallwaySpace(None),
                                    aroom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::B => {
                    if let Some((spaces_to_end, new_room)) = self.broom.receive(*a) {
                        ret.push((
                            a.cost() * (spaces_to_end + 1),
                            Board {
                                bc: HallwaySpace(None),
                                broom: new_room,
                                ..self.clone()
                            },
                        ));
                    }
                }
                Amphipod::C => {
                    if let Some((spaces_to_end, new_room)) = self.croom.receive(*a) {
                        ret.push((
                            a.cost() * (spaces_to_end + 1),
                            Board {
                                bc: HallwaySpace(None),
                                croom: new_room,
                                ..self.clone()
                            },
                        ));
                    }
                }
                Amphipod::D => {
                    if self.cd.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.droom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 3),
                                Board {
                                    bc: HallwaySpace(None),
                                    droom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
            }
        }
        if let HallwaySpace(Some(a)) = &self.cd {
            // check if there is something in leftish space and if it can make it to its destination
            match a {
                Amphipod::A => {
                    if self.ab.0.is_none() && self.bc.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.aroom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 5),
                                Board {
                                    cd: HallwaySpace(None),
                                    aroom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::B => {
                    if self.bc.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.broom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 3),
                                Board {
                                    cd: HallwaySpace(None),
                                    broom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::C => {
                    if let Some((spaces_to_end, new_room)) = self.croom.receive(*a) {
                        ret.push((
                            a.cost() * (spaces_to_end + 1),
                            Board {
                                cd: HallwaySpace(None),
                                croom: new_room,
                                ..self.clone()
                            },
                        ));
                    }
                }
                Amphipod::D => {
                    if let Some((spaces_to_end, new_room)) = self.droom.receive(*a) {
                        ret.push((
                            a.cost() * (spaces_to_end + 1),
                            Board {
                                cd: HallwaySpace(None),
                                droom: new_room,
                                ..self.clone()
                            },
                        ));
                    }
                }
            }
        }
        if let HallwaySpace(Some(a)) = &self.rightish {
            // check if there is something in leftish space and if it can make it to its destination
            match a {
                Amphipod::A => {
                    if self.ab.0.is_none() && self.bc.0.is_none() && self.cd.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.aroom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 7),
                                Board {
                                    rightish: HallwaySpace(None),
                                    aroom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::B => {
                    if self.bc.0.is_none() && self.cd.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.broom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 5),
                                Board {
                                    rightish: HallwaySpace(None),
                                    broom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::C => {
                    if self.cd.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.croom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 3),
                                Board {
                                    rightish: HallwaySpace(None),
                                    croom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::D => {
                    if let Some((spaces_to_end, new_room)) = self.droom.receive(*a) {
                        ret.push((
                            a.cost() * (spaces_to_end + 1),
                            Board {
                                rightish: HallwaySpace(None),
                                droom: new_room,
                                ..self.clone()
                            },
                        ));
                    }
                }
            }
        }
        if let HallwaySpace(Some(a)) = &self.rightmost {
            // check if there is something in leftish space and if it can make it to its destination
            match a {
                Amphipod::A => {
                    if self.ab.0.is_none()
                        && self.bc.0.is_none()
                        && self.cd.0.is_none()
                        && self.rightish.0.is_none()
                    {
                        if let Some((spaces_to_end, new_room)) = self.aroom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 8),
                                Board {
                                    rightmost: HallwaySpace(None),
                                    aroom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::B => {
                    if self.bc.0.is_none() && self.cd.0.is_none() && self.rightish.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.broom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 6),
                                Board {
                                    rightmost: HallwaySpace(None),
                                    broom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::C => {
                    if self.cd.0.is_none() && self.rightish.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.croom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 4),
                                Board {
                                    rightmost: HallwaySpace(None),
                                    croom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
                Amphipod::D => {
                    if self.rightish.0.is_none() {
                        if let Some((spaces_to_end, new_room)) = self.droom.receive(*a) {
                            ret.push((
                                a.cost() * (spaces_to_end + 2),
                                Board {
                                    rightmost: HallwaySpace(None),
                                    droom: new_room,
                                    ..self.clone()
                                },
                            ));
                        }
                    }
                }
            }
        }
        // now check for the rooms to all of the different hallway positions
        if let Some((space_to_enter_hallway, a, new_room)) = self.aroom.send(Amphipod::A) {
            // check all of the hallway spaces to the left
            if self.leftish.0.is_none() {
                if self.leftmost.0.is_none() {
                    ret.push((
                        a.cost() * (space_to_enter_hallway + 2),
                        Board {
                            leftmost: HallwaySpace(Some(a)),
                            aroom: new_room.clone(),
                            ..self.clone()
                        },
                    ))
                }
                ret.push((
                    a.cost() * (space_to_enter_hallway + 1),
                    Board {
                        leftish: HallwaySpace(Some(a)),
                        aroom: new_room.clone(),
                        ..self.clone()
                    },
                ))
            }
            // check all of the hallway spaces to the right
            if self.ab.0.is_none() {
                if self.bc.0.is_none() {
                    if self.cd.0.is_none() {
                        if self.rightish.0.is_none() {
                            if self.rightmost.0.is_none() {
                                // we can move all the way to the right!
                                ret.push((
                                    a.cost() * (space_to_enter_hallway + 8),
                                    Board {
                                        rightmost: HallwaySpace(Some(a)),
                                        aroom: new_room.clone(),
                                        ..self.clone()
                                    },
                                ))
                            }
                            ret.push((
                                a.cost() * (space_to_enter_hallway + 7),
                                Board {
                                    rightish: HallwaySpace(Some(a)),
                                    aroom: new_room.clone(),
                                    ..self.clone()
                                },
                            ))
                        }
                        ret.push((
                            a.cost() * (space_to_enter_hallway + 5),
                            Board {
                                cd: HallwaySpace(Some(a)),
                                aroom: new_room.clone(),
                                ..self.clone()
                            },
                        ))
                    }
                    ret.push((
                        a.cost() * (space_to_enter_hallway + 3),
                        Board {
                            bc: HallwaySpace(Some(a)),
                            aroom: new_room.clone(),
                            ..self.clone()
                        },
                    ))
                }
                ret.push((
                    a.cost() * (space_to_enter_hallway + 1),
                    Board {
                        ab: HallwaySpace(Some(a)),
                        aroom: new_room.clone(),
                        ..self.clone()
                    },
                ))
            }
        }
        if let Some((space_to_enter_hallway, a, new_room)) = self.broom.send(Amphipod::B) {
            // check all of the hallway spaces to the left
            if self.ab.0.is_none() {
                if self.leftish.0.is_none() {
                    if self.leftmost.0.is_none() {
                        ret.push((
                            a.cost() * (space_to_enter_hallway + 4),
                            Board {
                                leftmost: HallwaySpace(Some(a)),
                                broom: new_room.clone(),
                                ..self.clone()
                            },
                        ))
                    }
                    ret.push((
                        a.cost() * (space_to_enter_hallway + 3),
                        Board {
                            leftish: HallwaySpace(Some(a)),
                            broom: new_room.clone(),
                            ..self.clone()
                        },
                    ))
                }
                ret.push((
                    a.cost() * (space_to_enter_hallway + 1),
                    Board {
                        ab: HallwaySpace(Some(a)),
                        broom: new_room.clone(),
                        ..self.clone()
                    },
                ))
            }
            // check all of the hallway spaces to the right
            if self.bc.0.is_none() {
                if self.cd.0.is_none() {
                    if self.rightish.0.is_none() {
                        if self.rightmost.0.is_none() {
                            // we can move all the way to the right!
                            ret.push((
                                a.cost() * (space_to_enter_hallway + 6),
                                Board {
                                    rightmost: HallwaySpace(Some(a)),
                                    broom: new_room.clone(),
                                    ..self.clone()
                                },
                            ))
                        }
                        ret.push((
                            a.cost() * (space_to_enter_hallway + 5),
                            Board {
                                rightish: HallwaySpace(Some(a)),
                                broom: new_room.clone(),
                                ..self.clone()
                            },
                        ))
                    }
                    ret.push((
                        a.cost() * (space_to_enter_hallway + 3),
                        Board {
                            cd: HallwaySpace(Some(a)),
                            broom: new_room.clone(),
                            ..self.clone()
                        },
                    ))
                }
                ret.push((
                    a.cost() * (space_to_enter_hallway + 1),
                    Board {
                        bc: HallwaySpace(Some(a)),
                        broom: new_room.clone(),
                        ..self.clone()
                    },
                ))
            }
        }
        if let Some((space_to_enter_hallway, a, new_room)) = self.croom.send(Amphipod::C) {
            // check all of the hallway spaces to the left
            if self.bc.0.is_none() {
                if self.ab.0.is_none() {
                    if self.leftish.0.is_none() {
                        if self.leftmost.0.is_none() {
                            ret.push((
                                a.cost() * (space_to_enter_hallway + 6),
                                Board {
                                    leftmost: HallwaySpace(Some(a)),
                                    croom: new_room.clone(),
                                    ..self.clone()
                                },
                            ))
                        }
                        ret.push((
                            a.cost() * (space_to_enter_hallway + 5),
                            Board {
                                leftish: HallwaySpace(Some(a)),
                                croom: new_room.clone(),
                                ..self.clone()
                            },
                        ))
                    }
                    ret.push((
                        a.cost() * (space_to_enter_hallway + 3),
                        Board {
                            ab: HallwaySpace(Some(a)),
                            croom: new_room.clone(),
                            ..self.clone()
                        },
                    ))
                }
                ret.push((
                    a.cost() * (space_to_enter_hallway + 1),
                    Board {
                        bc: HallwaySpace(Some(a)),
                        croom: new_room.clone(),
                        ..self.clone()
                    },
                ))
            }
            // check all of the hallway spaces to the right
            if self.cd.0.is_none() {
                if self.rightish.0.is_none() {
                    if self.rightmost.0.is_none() {
                        // we can move all the way to the right!
                        ret.push((
                            a.cost() * (space_to_enter_hallway + 4),
                            Board {
                                rightmost: HallwaySpace(Some(a)),
                                croom: new_room.clone(),
                                ..self.clone()
                            },
                        ))
                    }
                    ret.push((
                        a.cost() * (space_to_enter_hallway + 3),
                        Board {
                            rightish: HallwaySpace(Some(a)),
                            croom: new_room.clone(),
                            ..self.clone()
                        },
                    ))
                }
                ret.push((
                    a.cost() * (space_to_enter_hallway + 1),
                    Board {
                        cd: HallwaySpace(Some(a)),
                        croom: new_room.clone(),
                        ..self.clone()
                    },
                ))
            }
        }
        if let Some((space_to_enter_hallway, a, new_room)) = self.droom.send(Amphipod::D) {
            // check all of the hallway spaces to the left
            if self.cd.0.is_none() {
                if self.bc.0.is_none() {
                    if self.ab.0.is_none() {
                        if self.leftish.0.is_none() {
                            if self.leftmost.0.is_none() {
                                ret.push((
                                    a.cost() * (space_to_enter_hallway + 8),
                                    Board {
                                        leftmost: HallwaySpace(Some(a)),
                                        droom: new_room.clone(),
                                        ..self.clone()
                                    },
                                ))
                            }
                            ret.push((
                                a.cost() * (space_to_enter_hallway + 7),
                                Board {
                                    leftish: HallwaySpace(Some(a)),
                                    droom: new_room.clone(),
                                    ..self.clone()
                                },
                            ))
                        }
                        ret.push((
                            a.cost() * (space_to_enter_hallway + 5),
                            Board {
                                ab: HallwaySpace(Some(a)),
                                droom: new_room.clone(),
                                ..self.clone()
                            },
                        ))
                    }
                    ret.push((
                        a.cost() * (space_to_enter_hallway + 3),
                        Board {
                            bc: HallwaySpace(Some(a)),
                            droom: new_room.clone(),
                            ..self.clone()
                        },
                    ))
                }
                ret.push((
                    a.cost() * (space_to_enter_hallway + 1),
                    Board {
                        cd: HallwaySpace(Some(a)),
                        droom: new_room.clone(),
                        ..self.clone()
                    },
                ))
            }

            // check all of the hallway spaces to the right
            if self.rightish.0.is_none() {
                if self.rightmost.0.is_none() {
                    // we can move all the way to the right!
                    ret.push((
                        a.cost() * (space_to_enter_hallway + 2),
                        Board {
                            rightmost: HallwaySpace(Some(a)),
                            droom: new_room.clone(),
                            ..self.clone()
                        },
                    ))
                }
                ret.push((
                    a.cost() * (space_to_enter_hallway + 1),
                    Board {
                        rightish: HallwaySpace(Some(a)),
                        droom: new_room.clone(),
                        ..self.clone()
                    },
                ))
            }
        }
        ret
    }
}

fn a_star(start: Board) -> usize {
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    // This is usually implemented as a min-heap or priority queue rather than a hash-set.
    let mut openset = priority_queue::PriorityQueue::new();
    openset.push(start.clone(), Reverse(0));
    let mut gscore = HashMap::new();
    gscore.insert(start.clone(), 0);

    while !openset.is_empty() {
        let (current, _estimate) = openset.pop().unwrap();
        if current.is_done() {
            return gscore[&current];
        }

        for (cost, neighbor) in current.next() {
            let tentative_score = gscore[&current] + cost;
            if tentative_score < *gscore.entry(neighbor.clone()).or_insert(usize::MAX) {
                // This path to neighbor is better than any previous one. Record it!
                gscore.insert(neighbor.clone(), tentative_score);
                let estimated_cost = tentative_score + neighbor.estimate();
                if openset.get(&neighbor).is_some() {
                    openset.change_priority(&neighbor, Reverse(estimated_cost));
                } else {
                    openset.push(neighbor, Reverse(estimated_cost));
                }
            }
        }
    }
    panic!("never found target")
}
fn part1(input: &str) -> Part1 {
    let input = parse(input);
    let board = Board::new(&input);
    a_star(board)
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 12521)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string(&INPUT_PATH).unwrap();
    assert_eq!(part1(&input), 13495)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = Part1;

fn part2(input: &str) -> Part2 {
    let input = parse(input);
    let mut input = input
        .into_iter()
        .map(|(a, row, c)| {
            if row == 1 {
                (a, row + 2, c)
            } else {
                (a, row, c)
            }
        })
        .collect_vec();
    /*
     add
    #D#C#B#A#
    #D#B#A#C# */
    input.push((Amphipod::D, 1, 3));
    input.push((Amphipod::D, 2, 3));
    input.push((Amphipod::C, 1, 5));
    input.push((Amphipod::B, 2, 5));
    input.push((Amphipod::B, 1, 7));
    input.push((Amphipod::A, 2, 7));
    input.push((Amphipod::A, 1, 9));
    input.push((Amphipod::C, 2, 9));

    let board = Board::new(&input);
    a_star(board)
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 44169)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap();
    assert_eq!(part2(&input), 53767)
}
