use crate::input;

pub fn solve() {
    let x = input::file_for_day(10).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i64 {
    let mut numbers: Vec<i64> = input
        .lines()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(numbers[numbers.len() - 1] + 3);

    let mut d1 = 0;
    let mut d3 = 0;

    for n in 1..numbers.len() {
        let current = numbers[n];
        let previous = numbers[n - 1];

        match current - previous {
            1 => d1 += 1,
            2 => (),
            3 => d3 += 1,
            x => panic!(
                "Unexpected diff: {} (prev: {}, cur: {}",
                x, current, previous
            ),
        };
    }

    d1 * d3
}

fn part_two(input: String) -> i64 {
    let mut numbers: Vec<i64> = input
        .lines()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(numbers[numbers.len() - 1] + 3);

    let mut register = vec![0; numbers.len()];
    register[0] = 1;

    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            if numbers[j] - numbers[i] > 3 {
                break;
            }

            register[j] += register[i];
        }
    }

    register.last().copied().unwrap()
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_ONE: &str = r#"
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
"#;

    static TEST_INPUT_TWO: &str = r#"
16
10
15
5
1
11
7
19
6
12
4
"#;

    static SOLUTION_ONE: i64 = 220;
    static SOLUTION_TWO: i64 = 8;
    static SOLUTION_TWO_LONG: i64 = 19208;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT_ONE.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT_TWO.to_string()), SOLUTION_TWO);
        assert_eq!(
            super::part_two(TEST_INPUT_ONE.to_string()),
            SOLUTION_TWO_LONG
        );
    }
}
