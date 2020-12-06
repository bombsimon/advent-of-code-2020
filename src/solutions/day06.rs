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
            let mut x = l
                .chars()
                .filter(|&c| !c.is_whitespace())
                .collect::<Vec<_>>();
            x.sort();
            x.dedup();
            x.len() as i64
        })
        .sum()
}

fn part_two(input: String) -> i64 {
    input
        .split("\n\n")
        .map(|l| {
            let mut all_rows = l.lines().filter(|&c| c != "");
            let mut shared_responses = all_rows.next().unwrap().chars().collect::<Vec<_>>();

            for row in all_rows {
                shared_responses.retain(|&a| row.contains(a));
            }

            shared_responses.len() as i64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r#"
abc

a
b
c

ab
ac

a
a
a
a

b
"#;

    static SOLUTION_ONE: i64 = 11;
    static SOLUTION_TWO: i64 = 6;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string()), SOLUTION_TWO);
    }
}
