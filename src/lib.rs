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

/// # Examples
/// ```
/// # use advent_2021::manhattan_distance;
/// assert_eq!(manhattan_distance(-6, 3), 9);
/// ```
pub fn manhattan_distance<I: num_traits::Num + num_traits::Signed>(x: I, y: I) -> I {
    x.abs() + y.abs()
}

/// # Examples
/// ```
/// # use advent_2021::digits;
/// assert_eq!(digits("134"), vec![1, 3, 4]);
/// ```
pub fn digits(num: &str) -> Vec<u32> {
    num.chars().map(|c| c.to_digit(10).unwrap()).collect()
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
