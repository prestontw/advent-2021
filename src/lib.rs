use std::collections::HashMap;
use std::hash::Hash;

use regex::Match;

/// # Examples
/// ```
/// # use advent_2021::blank_lines;
/// let input = r#"hello
/// there
///
/// anyone
///
/// there?"#;
/// assert_eq!(blank_lines(input), vec![vec!["hello", "there"], vec!["anyone"], vec!["there?"]]);
/// ```
pub fn blank_lines(s: &str) -> Vec<Vec<&str>> {
    let (mut agg, final_group) =
        s.lines()
            .fold((Vec::new(), Vec::new()), |(mut agg, mut grouped), line| {
                if line.is_empty() {
                    agg.push(grouped);
                    (agg, Vec::new())
                } else {
                    grouped.push(line);
                    (agg, grouped)
                }
            });
    agg.push(final_group);
    agg
}

pub fn commas(s: &str) -> Vec<&str> {
    s.split(',').collect()
}
pub fn lines(s: &str) -> Vec<&str> {
    s.lines().collect()
}

/// # Examples
/// ```
/// # use advent_2021::manhattan_distance;
/// assert_eq!(manhattan_distance(-6, 3), 9);
/// ```
pub fn manhattan_distance<I: num_traits::Num + num_traits::Signed>(x: I, y: I) -> I {
    x.abs() + y.abs()
}

pub fn manhattan_distance3d<I: num_traits::Num + num_traits::Signed>(
    (a, b, c): (I, I, I),
    (x, y, z): (I, I, I),
) -> I {
    (a - x).abs() + (b - y).abs() + (c - z).abs()
}

pub fn abs_diff(n1: u32, n2: u32) -> u32 {
    n1.max(n2) - n1.min(n2)
}

pub fn counts<I, E>(i: I) -> HashMap<E, usize>
where
    I: IntoIterator<Item = E>,
    E: Clone + Hash + Eq,
{
    i.into_iter().fold(HashMap::new(), |mut counts, item| {
        let count = counts.entry(item).or_insert(0);
        *count += 1;
        counts
    })
}

/// # Examples
/// ```
/// # use advent_2021::digits;
/// assert_eq!(digits("134"), vec![1, 3, 4]);
/// ```
pub fn digits(num: &str) -> Vec<u32> {
    num.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

pub fn number_digits(num: usize) -> Vec<u32> {
    digits(&num.to_string())
}

/// # Examples
/// ```
/// # use advent_2021::number_to_binary;
/// assert_eq!(number_to_binary(1), vec![1]);
/// assert_eq!(number_to_binary(2), vec![1, 0]);
/// ```
pub fn number_to_binary(num: usize) -> Vec<u32> {
    format!("{:b}", num)
        .chars()
        .map(|c| c.to_digit(2).unwrap())
        .collect()
}

/// # Examples
/// ```
/// # use advent_2021::{number_to_binary, pad_vectors};
/// assert_eq!(pad_vectors(&[number_to_binary(1), number_to_binary(2)], 0), vec![vec![0, 1], vec![1, 0]]);
/// ```
pub fn pad_vectors<T: Clone>(vs: &[Vec<T>], padding: T) -> Vec<Vec<T>> {
    let max_length = vs.iter().map(|v| v.len()).max().unwrap();
    vs.iter()
        .map(|v| {
            let line = v.clone();
            let mut prefix = vec![padding.clone(); max_length - line.len()];
            prefix.extend_from_slice(&line);
            prefix
        })
        .collect()
}

// could add lines, commas, spaces, memoize, regex things next

/// # Examples
/// ```
/// # use advent_2021::extract_values;
/// let re = regex::Regex::new(r"(\w*) (\w*) bags contain (.*)\.").unwrap();
/// assert_eq!(extract_values(&re, "muted tomato bags contain 1 bright brown bag."),
/// vec!["muted", "tomato", "1 bright brown bag"]);
/// ```
pub fn extract_values<'source>(re: &regex::Regex, s: &'source str) -> Vec<&'source str> {
    re.captures_iter(s)
        .flat_map(|capture| capture.iter().skip(1).collect::<Vec<_>>())
        .filter_map(|c: Option<Match>| c.map(|c| c.as_str()))
        .collect()
}

#[macro_export]
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}
