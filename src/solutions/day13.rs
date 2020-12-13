use crate::input;

pub fn solve() {
    let x = input::file_for_day(13).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone(), 10000000000));
}

fn part_one(input: String) -> i64 {
    let mut lines = input.lines().filter(|&c| c != "");

    let id = lines.next().unwrap().parse::<i64>().unwrap();
    let lines = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|l| l.parse::<i64>().ok())
        .collect::<Vec<_>>();

    for time in id..i64::MAX {
        for line in lines.clone() {
            if time % line == 0 {
                return line * (time - id);
            }
        }
    }

    -1
}

fn part_two(input: String, min_start: i64) -> i64 {
    let mut t = -1;

    let mut lines = input
        .lines()
        .filter(|&c| c != "")
        .last()
        .unwrap()
        .split(",")
        .map(|l| {
            t += 1;
            match l.parse::<i64>() {
                Ok(n) => (t, n),
                Err(_) => (t, 0),
            }
        })
        .filter(|&c| c.1 != 0)
        .collect::<Vec<_>>();

    lines.sort_by_key(|k| k.1);
    lines.reverse();

    let first = &lines[0];
    let tail = &lines[1..];

    let mut n = min_start - (min_start % first.1);
    'outer: loop {
        n += first.1;

        for (tdiff, line) in tail {
            if (n + (tdiff - first.0)) % *line != 0 {
                continue 'outer;
            }
        }

        return n - first.0;
    }
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r#"
939
7,13,x,x,59,x,31,19
"#;

    static SOLUTION_ONE: i64 = 295;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        use std::collections::HashMap;
        let mut cases: HashMap<&str, i64> = HashMap::new();

        cases.insert("1\n7,13,x,x,59,x,31,19\n", 1068781);
        cases.insert("1\n17,x,13,19\n", 3417);
        cases.insert("1\n67,7,59,61\n", 754018);
        cases.insert("1\n67,x,7,59,61\n", 779210);
        cases.insert("1\n67,7,x,59,61\n", 1261476);
        cases.insert("1\n1789,37,47,1889\n", 1202161486);

        for (tc, solution) in cases {
            assert_eq!(super::part_two(tc.to_string(), 1), solution);
        }
    }
}
