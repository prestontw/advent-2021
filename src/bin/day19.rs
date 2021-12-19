use std::collections::HashSet;

use advent_2021::{blank_lines, manhattan_distance3d};
use itertools::Itertools;
use nalgebra::{Matrix3, Vector3};

fn main() {
    println!("Hello, world!");
}

const SAMPLE: &str = r#"--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14"#;

type Part1 = usize;

#[derive(Hash, PartialEq, Eq, Clone)]
struct PointCloud(Vec<(i64, i64, i64)>);

impl PointCloud {
    fn as_distances(&self) -> Vec<Vec<f64>> {
        let mut ret = vec![];
        for point in &self.0 {
            let mut row = vec![];
            for other in &self.0 {
                row.push(distance3d(point, other));
            }
            ret.push(row)
        }
        ret
    }
}

fn are_intersecting(d1s: &Vec<Vec<f64>>, d2s: &Vec<Vec<f64>>) -> bool {
    // if at least twelve rows in d1s have at least 12 distances in a row in d2s
    d1s.iter()
        .filter(|row| {
            for other_row in d2s {
                //check if row has 12 of the same in other_row
                let overlap = same_point(row, other_row);
                if overlap {
                    return true;
                }
            }
            false
        })
        .count()
        >= 12
}

#[test]
fn example_intersecting() {
    let points = parse(&SAMPLE);
    let p1 = &points[0];
    let p2 = &points[1];
    let d1 = p1.as_distances();
    let d2 = p2.as_distances();
    assert!(are_intersecting(&d1, &d2));

    let p4 = &points[4];
    let d4 = p4.as_distances();
    assert!(are_intersecting(&d2, &d4))
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum Angle {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}
impl Angle {
    fn cos(&self) -> i64 {
        match self {
            Self::Zero => 1,
            Self::Ninety | Self::TwoSeventy => 0,
            Self::OneEighty => -1,
        }
    }
    fn sin(&self) -> i64 {
        match self {
            Self::Zero | Self::OneEighty => 0,
            Self::Ninety => 1,
            Self::TwoSeventy => -1,
        }
    }
}
fn general_rotation_matrix(alpha: Angle, beta: Angle, gamma: Angle) -> Matrix3<i64> {
    let rot_mat = [
        alpha.cos() * beta.cos(),
        alpha.sin() * beta.cos(),
        -beta.sin(),
        alpha.cos() * beta.sin() * gamma.sin() - alpha.sin() * gamma.cos(),
        alpha.sin() * beta.sin() * gamma.sin() + alpha.cos() * gamma.cos(),
        beta.cos() * gamma.sin(),
        alpha.cos() * beta.sin() * gamma.cos() + alpha.sin() * gamma.sin(),
        alpha.sin() * beta.sin() * gamma.cos() - alpha.cos() * gamma.sin(),
        beta.cos() * gamma.cos(),
    ];

    Matrix3::from_iterator(rot_mat)
}

fn rotation_between(v1: &Vector3<i64>, v2: &Vector3<i64>) -> Matrix3<i64> {
    // for each rotation of angles, see if it gets v1 to v2
    use Angle::*;
    let possible_angles = [Zero, Ninety, OneEighty, TwoSeventy];
    for alpha in possible_angles {
        for beta in possible_angles {
            for gamma in possible_angles {
                let potential = general_rotation_matrix(alpha, beta, gamma);
                if &(potential * v1) == v2 {
                    return potential;
                }
            }
        }
    }
    panic!("couldn't find rotation")
}

type Pos = (i64, i64, i64);
fn same_point(row: &Vec<f64>, other_row: &Vec<f64>) -> bool {
    row.iter().filter(|point| other_row.contains(point)).count() >= 12
}
/// Assuming that these intersect, find two analogous points
fn corresponding_points(points1: &PointCloud, points2: &PointCloud) -> ((Pos, Pos), (Pos, Pos)) {
    let diffs1 = points1.as_distances();
    let diffs2 = points2.as_distances();

    let (p11index, p12index, p21index, p22index) = {
        let mut p1index = 0;
        let mut p2index = 0;
        'outer: for (p1_index, p1_row) in diffs1.iter().enumerate() {
            for (p2_index, p2_row) in diffs2.iter().enumerate() {
                if same_point(p1_row, p2_row) {
                    p1index = p1_index;
                    p2index = p2_index;
                    break 'outer;
                }
            }
        }
        let p1 = (p1index, p2index);
        let mut p1index = 0;
        let mut p2index = 0;
        'outer2: for (p1_index, p1_row) in diffs1.iter().enumerate() {
            if p1_index == p1.0 {
                continue;
            }
            for (p2_index, p2_row) in diffs2.iter().enumerate() {
                if same_point(p1_row, p2_row) {
                    p1index = p1_index;
                    p2index = p2_index;
                    break 'outer2;
                }
            }
        }
        (p1.0, p1index, p1.1, p2index)
    };
    (
        (points1.0[p11index], points1.0[p12index]),
        (points2.0[p21index], points2.0[p22index]),
    )
}

fn translate_coordinates(
    reference_point: (i64, i64, i64),
    analogous_point: (i64, i64, i64),
    rotation_between: &Matrix3<i64>,
    points: Vec<(i64, i64, i64)>,
) -> (Vec<(i64, i64, i64)>, Pos) {
    let reference_point = Vector3::new(reference_point.0, reference_point.1, reference_point.2);
    let analogous_point = Vector3::new(analogous_point.0, analogous_point.1, analogous_point.2);
    let updated_points = points
        .into_iter()
        .map(|point| {
            let point = Vector3::new(point.0, point.1, point.2);
            let diff = point - analogous_point;
            let translated = rotation_between * diff;
            let result = reference_point + translated;
            (result.x, result.y, result.z)
        })
        .collect();

    let scanner_pos = Vector3::new(0, 0, 0);
    let diff = scanner_pos - analogous_point;
    let translated = rotation_between * diff;
    let result = reference_point + translated;
    (updated_points, (result.x, result.y, result.z))
}

#[test]
fn example_absolute_points() {
    use nalgebra::Vector3;
    let s0v0 = Vector3::new(-618, -824, -621);
    let s0v1 = Vector3::new(-447, -329, 318);
    let s1v0 = Vector3::new(686, 422, 578);
    let s1v1 = Vector3::new(515, 917, -361);

    let s0diff = s0v0 - s0v1;
    let s1diff = s1v0 - s1v1;

    let rotation = rotation_between(&s0diff, &s1diff);

    assert_eq!(
        rotation * (Vector3::new(-537, -823, -458,) - s0v1),
        Vector3::new(605, 423, 415,) - s1v1
    )
}

fn distance3d(p1: &(i64, i64, i64), p2: &(i64, i64, i64)) -> f64 {
    let xdist = (p1.0 - p2.0) as f64;
    let ydist = (p1.1 - p2.1) as f64;
    let zdist = (p1.2 - p2.2) as f64;
    (xdist * xdist + ydist * ydist + zdist * zdist).sqrt()
}

fn tuple_to_vector((x, y, z): (i64, i64, i64)) -> Vector3<i64> {
    Vector3::new(x, y, z)
}
fn parse(input: &str) -> Vec<PointCloud> {
    let scanners = blank_lines(input);
    scanners
        .into_iter()
        .map(|chunk| {
            PointCloud(
                chunk[1..]
                    .into_iter()
                    .map(|line| {
                        line.split(',')
                            .map(|chunk| chunk.parse::<i64>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn part1(input: &str) -> Part1 {
    let mut input = parse(input);

    let mut unseen = input.split_off(1);
    let mut points = input[0].0.iter().cloned().collect::<HashSet<_>>();
    let mut seen = input;

    while !unseen.is_empty() {
        let (unseen_index, seen_index) = unseen
            .iter()
            .enumerate()
            .find_map(|(index, pc)| {
                let dist = pc.as_distances();
                for (other_index, other) in seen.iter().enumerate() {
                    let odist = other.as_distances();
                    if are_intersecting(&dist, &odist) {
                        return Some((index, other_index));
                    }
                }
                None
            })
            .unwrap();
        let to_add = unseen.remove(unseen_index);
        let seen_with_points = &seen[seen_index];
        let ((reference_seen, other_seen), (analagous_unseen, other_unseen)) =
            corresponding_points(seen_with_points, &to_add);
        let rotation_between = {
            let reference_seen = Vector3::new(reference_seen.0, reference_seen.1, reference_seen.2);
            let other_seen = tuple_to_vector(other_seen);
            let seen_diff = other_seen - reference_seen;

            let analogous_unseen = tuple_to_vector(analagous_unseen);
            let other_unseen = tuple_to_vector(other_unseen);
            let unseen_diff = other_unseen - analogous_unseen;

            rotation_between(&unseen_diff, &seen_diff)
        };
        let (translated, _) = translate_coordinates(
            reference_seen,
            analagous_unseen,
            &rotation_between,
            to_add.0.clone(),
        );
        for p in &translated {
            points.insert(*p);
        }

        // add *translated* to seen
        seen.push(PointCloud(translated));
    }
    points.len()
}

#[test]
fn tpart1_sample() {
    assert_eq!(part1(&SAMPLE), 79)
}

#[test]
fn tpart1() {
    let input = std::fs::read_to_string("inputs/day19.txt").unwrap();
    assert_ne!(part1(&input), 607);
    assert_eq!(part1(&input), 445)
}

////////////////////////////////////////////////
///  start part 2
////////////////////////////////////////////////
type Part2 = i64;

fn part2(input: &str) -> Part2 {
    let mut input = parse(input);

    let mut unseen = input.split_off(1);
    let mut scanner_positions = vec![(0, 0, 0)].into_iter().collect::<HashSet<_>>();
    let mut seen = input;

    while !unseen.is_empty() {
        let (unseen_index, seen_index) = unseen
            .iter()
            .enumerate()
            .find_map(|(index, pc)| {
                let dist = pc.as_distances();
                for (other_index, other) in seen.iter().enumerate() {
                    let odist = other.as_distances();
                    if are_intersecting(&dist, &odist) {
                        return Some((index, other_index));
                    }
                }
                None
            })
            .unwrap();
        let to_add = unseen.remove(unseen_index);
        let seen_with_points = &seen[seen_index];
        let ((reference_seen, other_seen), (analagous_unseen, other_unseen)) =
            corresponding_points(seen_with_points, &to_add);
        let rotation_between = {
            let reference_seen = Vector3::new(reference_seen.0, reference_seen.1, reference_seen.2);
            let other_seen = tuple_to_vector(other_seen);
            let seen_diff = other_seen - reference_seen;

            let analogous_unseen = tuple_to_vector(analagous_unseen);
            let other_unseen = tuple_to_vector(other_unseen);
            let unseen_diff = other_unseen - analogous_unseen;

            rotation_between(&unseen_diff, &seen_diff)
        };
        let (translated, scanner_pos) = translate_coordinates(
            reference_seen,
            analagous_unseen,
            &rotation_between,
            to_add.0.clone(),
        );
        scanner_positions.insert(scanner_pos);

        // add *translated* to seen
        seen.push(PointCloud(translated));
    }
    scanner_positions
        .iter()
        .cartesian_product(scanner_positions.iter())
        .map(|(p1, p2)| manhattan_distance3d(*p1, *p2))
        .max()
        .unwrap()
}

#[test]
fn tpart2_sample() {
    assert_eq!(part2(&SAMPLE), 3621)
}

#[test]
fn tpart2() {
    let input = std::fs::read_to_string("inputs/day19.txt").unwrap();
    assert_eq!(part2(&input), 13225)
}
