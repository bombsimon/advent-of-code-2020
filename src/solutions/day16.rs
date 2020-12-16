use crate::input;

pub fn solve() {
    let x = input::file_for_day(16).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn parse_input(input: String) -> (Vec<Vec<(i32, i32)>>, Vec<i32>, Vec<Vec<i32>>) {
    let ref mut x = input
        .split("\n\n")
        .map(|l| l.trim().split("\n").collect::<Vec<_>>());

    let rules_raw = x.next().unwrap();
    let rules = rules_raw
        .iter()
        .map(|c| {
            c.split(": ").collect::<Vec<_>>()[1]
                .split(" or ")
                .map(|n| {
                    let range = n
                        .split("-")
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

                    (range[0], range[1])
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mine_raw = &x.next().unwrap()[1];
    let mine = mine_raw
        .split(",")
        .map(|c| c.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let nearby_raw = &x.next().unwrap()[1..];
    let nearby = nearby_raw
        .iter()
        .map(|c| {
            c.split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, mine, nearby)
}

fn part_one(input: String) -> i32 {
    let (rules, _, nearby) = parse_input(input);
    let mut invalid_number = 0;

    for ticket in &nearby {
        'number: for number in ticket {
            for rule in &rules {
                for rule_range in rule {
                    if *number >= rule_range.0 && *number <= rule_range.1 {
                        continue 'number;
                    }
                }
            }

            invalid_number += number;
        }
    }

    invalid_number
}

fn part_two(_input: String) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_ONE: &str = r#"
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
"#;
    static TEST_INPUT_TWO: &str = r#"
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
"#;

    static SOLUTION_ONE: i32 = 71;
    static SOLUTION_TWO: i32 = -1;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT_ONE.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT_TWO.to_string()), SOLUTION_TWO);
    }
}
