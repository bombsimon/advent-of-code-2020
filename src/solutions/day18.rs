use crate::input;

use std::collections::HashMap;

pub fn solve() {
    let x = input::file_for_day(18).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn parse_expression(exp: String) -> i64 {
    let mut parens: Vec<usize> = Vec::with_capacity(exp.len());
    let mut parens_map: HashMap<usize, usize> = HashMap::new();

    for (i, c) in exp.chars().enumerate() {
        match c {
            '(' => parens.push(i),
            ')' => {
                let matching_paren = parens[parens.len() - 1];
                parens = parens[..parens.len() - 1].to_vec();
                parens_map.insert(matching_paren, i);
            }
            _ => continue,
        }
    }

    simplify(exp, 0usize, &parens_map).parse::<i64>().unwrap()
}

fn simplify(to_simplify: String, paren_shift: usize, parens_map: &HashMap<usize, usize>) -> String {
    let mut s = String::new();
    let mut i = 0;
    let chars = to_simplify.chars().collect::<Vec<_>>();

    while i < to_simplify.len() {
        let c = chars[i];

        match c {
            '(' => {
                let paren_i = i + paren_shift; // Needed to find in map
                let block = &chars[i + 1..parens_map[&paren_i] - paren_shift]
                    .iter()
                    .collect::<String>();

                let simplified = simplify(block.to_string(), paren_shift + i + 1, parens_map);

                s.push_str(&simplified);
                i = parens_map[&paren_i] - paren_shift;
            }
            ')' => (),
            _ => s.push(c),
        }

        i += 1;
    }

    left_to_right(s).to_string()
}

fn left_to_right(exp: String) -> i64 {
    let tokens = exp.split_whitespace().collect::<Vec<_>>();
    let mut sum = 0i64;

    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i].parse::<i64>().unwrap();
        if i == 0 {
            sum = token;
        }

        if i >= tokens.len() - 2 {
            break;
        }

        let op = tokens[i + 1];
        let next = tokens[i + 2].parse::<i64>().unwrap();

        match op {
            "+" => sum += next,
            "*" => sum *= next,
            _ => unreachable!(),
        };

        i += 2;
    }

    sum
}

fn part_one(input: String) -> i64 {
    input
        .lines()
        .filter(|&c| c != "")
        .fold(0, |acc, item| acc + parse_expression(item.to_string()))
}

fn part_two(_input: String) -> i64 {
    -1
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let cases = vec![
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];

        for (tc, result) in cases {
            assert_eq!(super::part_one(tc.to_string()), result);
        }
    }

    #[test]
    fn part_two() {
        let cases: Vec<(String, i64)> = Vec::new();

        for (tc, result) in cases {
            assert_eq!(super::part_two(tc.to_string()), result);
        }
    }
}
