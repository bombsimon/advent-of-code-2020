use crate::input;

pub fn solve() {
    let x = input::file_for_day(9).join("\n");

    println!("Solution part 1: {}", part_one(x.clone(), 25));
    println!("Solution part 2: {}", part_two(x.clone(), 25));
}

fn sum_in_list(n: i64, list: &[i64]) -> bool {
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[i] + list[j] == n {
                return true;
            }
        }
    }

    false
}

fn sum_in_sequence(target: i64, list: &[i64]) -> &[i64] {
    for i in 0..list.len() {
        let mut sum: i64 = list[i];

        for j in i + 1..list.len() {
            sum += list[j];

            if sum == target {
                return &list[i..j];
            }

            if sum > target {
                break;
            }
        }
    }

    &[0, 0]
}

fn part_one(input: String, preamble: usize) -> i64 {
    let numbers: Vec<i64> = input
        .lines()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    for i in preamble..numbers.len() {
        let n = numbers[i];

        if !sum_in_list(n, &numbers[i - preamble..i]) {
            return n;
        }
    }

    -1
}

fn part_two(input: String, preamble: usize) -> i64 {
    let numbers: Vec<i64> = input
        .lines()
        .filter_map(|x| x.parse::<i64>().ok())
        .collect();

    for i in preamble..numbers.len() {
        let sum = numbers[i];
        let subset = &numbers[i - preamble..i];

        if !sum_in_list(sum, subset) {
            let seq = sum_in_sequence(sum, &numbers);

            let (min, max) = seq
                .iter()
                .fold((sum, 0), |(min, max), &n| (min.min(n), max.max(n)));

            return min + max;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r#"
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
"#;

    static SOLUTION_ONE: i64 = 127;
    static SOLUTION_TWO: i64 = 62;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string(), 5), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string(), 5), SOLUTION_TWO);
    }
}
