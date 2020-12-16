use crate::input;

use std::collections::HashMap;

pub fn solve() {
    let x = input::file_for_day(16).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn parse_input(input: String) -> (Vec<Vec<(String, (i32, i32))>>, Vec<i32>, Vec<Vec<i32>>) {
    let ref mut x = input
        .split("\n\n")
        .map(|l| l.trim().split("\n").collect::<Vec<_>>());

    let rules_raw = x.next().unwrap();
    let rules = rules_raw
        .iter()
        .map(|c| {
            let rules = c.split(": ").collect::<Vec<_>>();
            let typ = rules[0].to_string();
            let range = rules[1]
                .split(" or ")
                .map(|n| {
                    let range = n
                        .split("-")
                        .map(|i| i.parse::<i32>().unwrap())
                        .collect::<Vec<_>>();

                    (typ.clone(), (range[0], range[1]))
                })
                .collect::<Vec<_>>();

            range
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

fn part_one(input: String) -> i64 {
    let (rules, _, nearby) = parse_input(input);
    let mut invalid_number = 0i64;

    for ticket in &nearby {
        'number: for number in ticket {
            for rule in &rules {
                for rule_range in rule {
                    if *number >= rule_range.1 .0 && *number <= rule_range.1 .1 {
                        continue 'number;
                    }
                }
            }

            invalid_number += *number as i64;
        }
    }

    invalid_number
}

fn rule_valid(rules: &[(String, (i32, i32))], tickets: &[Vec<i32>], candidate: usize) -> bool {
    'ticket: for ticket in tickets {
        let n = ticket[candidate];

        for rule_range in rules {
            if n >= rule_range.1 .0 && n <= rule_range.1 .1 {
                continue 'ticket;
            }
        }

        return false;
    }

    true
}

fn part_two(input: String) -> i64 {
    let (all_rules, mine, nearby) = parse_input(input);

    let mut filtered = Vec::new();

    'ticket: for ticket in &nearby {
        'number: for number in ticket {
            for rule in &all_rules {
                for rule_range in rule {
                    if *number >= rule_range.1 .0 && *number <= rule_range.1 .1 {
                        continue 'number;
                    }
                }
            }

            continue 'ticket;
        }

        filtered.push(ticket.clone());
    }

    filtered.push(mine.clone());

    let mut hr: HashMap<String, usize> = HashMap::new();
    let mut hn: HashMap<usize, String> = HashMap::new();

    while hr.len() < mine.len() {
        let mut ht: HashMap<usize, Vec<String>> = HashMap::new();

        for rules in all_rules.iter() {
            if hr.contains_key(&rules[0].0) {
                continue;
            }

            for i in 0..mine.len() {
                let mut vec = match ht.get(&i) {
                    Some(v) => v.clone(),
                    None => Vec::new(),
                };

                if hn.contains_key(&i) {
                    continue;
                }

                if rule_valid(&rules, filtered.as_slice(), i) {
                    vec.push(rules[0].0.clone());
                    ht.insert(i, vec);
                }
            }
        }

        for (i, rule) in &ht {
            if rule.len() == 1 {
                hn.insert(*i, rule[0].clone());
                hr.insert(rule[0].clone(), *i);
            }
        }
    }

    hr.iter().fold(1, |acc, (k, &i)| {
        if !k.as_str().starts_with("departure") {
            return acc;
        }
        acc * mine[i] as i64
    })
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

    static SOLUTION_ONE: i64 = 71;
    static SOLUTION_TWO: i64 = 1;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT_ONE.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT_TWO.to_string()), SOLUTION_TWO);
    }
}
