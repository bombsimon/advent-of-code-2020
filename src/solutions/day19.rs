use regex::Regex;
use std::collections::HashMap;

use crate::input;

pub fn solve() {
    let x = input::file_for_day(19).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i64 {
    let (rules, patterns) = parse_input(input, false);
    let re_str = parse_rule(0, &rules);
    let re = Regex::new(format!("^{}$", re_str).as_str()).unwrap();

    patterns.iter().fold(
        0,
        |acc, pattern| {
            if re.is_match(pattern) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn part_two(input: String) -> i64 {
    let (rules, patterns) = parse_input(input, true);
    let re_str = parse_rule(0, &rules);

    let re = Regex::new(format!("^{}$", re_str).as_str()).unwrap();

    patterns.iter().fold(
        0,
        |acc, pattern| {
            if re.is_match(pattern) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

fn parse_input(input: String, replace: bool) -> (HashMap<i32, String>, Vec<String>) {
    let mut rules: HashMap<i32, String> = HashMap::new();

    let segmets = input.trim().split("\n\n").collect::<Vec<_>>();
    segmets[0].lines().filter(|&c| c != "").for_each(|l| {
        let r = l.split(": ").collect::<Vec<_>>();
        let id = r[0].parse::<i32>().unwrap();
        let mut rule = r[1].replace("\"", "");

        if replace {
            match id {
                8 => rule = "42 #+#".to_string(),
                11 => {
                    let mut rules: Vec<String> = vec![];
                    for i in 1..5 {
                        rules.push(format!("42 #{{{}}}# 31 #{{{}}}#", i, i));
                    }

                    rule = rules.join(" | ");
                }
                _ => (),
            }
        }

        rules.insert(id, rule);
    });

    (
        rules,
        segmets[1]
            .lines()
            .map(|l| l.to_string())
            .collect::<Vec<_>>(),
    )
}

fn parse_rule(rule_id: i32, rules: &HashMap<i32, String>) -> String {
    let mut re = String::new();
    re.push_str("(");

    let ors = rules
        .get(&rule_id)
        .unwrap()
        .split(" | ")
        .collect::<Vec<_>>();

    for i in 0..ors.len() {
        let ids = ors[i].split_whitespace().collect::<Vec<_>>();

        for id in ids {
            if id.contains("#") {
                let chars = id.chars().collect::<Vec<_>>();
                let val = chars[1..chars.len() - 1].iter().collect::<String>();

                re.push_str(val.as_str());
            } else {
                match id.parse::<i32>() {
                    Ok(n) => re.push_str(parse_rule(n, rules).as_str()),
                    Err(_) => {
                        return id.to_string();
                    }
                }
            }
        }

        if ors.len() > 1 && i != ors.len() - 1 {
            re.push_str("|");
        }
    }

    re.push_str(")");

    re
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_ONE: &str = r#"
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;
    static TEST_INPUT_TWO: &str = r#"
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

    static SOLUTION_ONE: i64 = 2;
    static SOLUTION_TWO: i64 = 12;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT_ONE.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT_TWO.to_string()), SOLUTION_TWO);
    }
}
