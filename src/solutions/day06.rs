use std::collections::BTreeMap;

use crate::input;

pub fn solve() {
    let x = input::file_for_day(6).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i64 {
    input
        .split("\n\n")
        .map(|l| {
            let mut s = l
                .split_whitespace()
                .collect::<String>()
                .chars()
                .collect::<Vec<_>>();
            s.sort();
            s.dedup();
            s.len() as i64
        })
        .fold(0, |acc, i| acc + i)
}

fn part_two(input: String) -> i64 {
    input
        .split("\n\n")
        .map(|l| {
            let mut all_yeses = 0;
            let rows = l.split_whitespace().collect::<Vec<_>>();
            let mut counts: BTreeMap<char, usize> = BTreeMap::new();

            for row in rows.iter() {
                for c in row.chars() {
                    *counts.entry(c.into()).or_insert(0) += 1;
                }
            }

            for (_, count) in counts.iter() {
                if *count == rows.len() {
                    all_yeses += 1;
                }
            }

            all_yeses
        })
        .fold(0, |acc, i| acc + i)
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_ONE: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;
    static TEST_INPUT_TWO: &str = r#"abc

a
b
c

ab
ac

a
a
a
a

b"#;

    static SOLUTION_ONE: i64 = 11;
    static SOLUTION_TWO: i64 = 6;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT_ONE.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT_TWO.to_string()), SOLUTION_TWO);
    }
}
