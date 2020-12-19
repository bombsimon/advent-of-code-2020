use crate::input;

use std::collections::HashMap;

pub fn solve() {
    let x = input::file_for_day(18).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i64 {
    input.lines().filter(|&c| c != "").fold(0, |acc, item| {
        acc + parse_expression(item.to_string(), false)
    })
}

fn part_two(input: String) -> i64 {
    input.lines().filter(|&c| c != "").fold(0, |acc, item| {
        acc + parse_expression(item.to_string(), true)
    })
}

fn parse_expression(exp: String, addition_first: bool) -> i64 {
    let mut parentheses: Vec<usize> = Vec::with_capacity(exp.len());
    let mut parentheses_map: HashMap<usize, usize> = HashMap::new();

    for (i, c) in exp.chars().enumerate() {
        match c {
            '(' => parentheses.push(i),
            ')' => {
                let matching_parentheses = parentheses[parentheses.len() - 1];
                parentheses = parentheses[..parentheses.len() - 1].to_vec();
                parentheses_map.insert(matching_parentheses, i);
            }
            _ => continue,
        }
    }

    simplify(exp, 0usize, &parentheses_map, addition_first)
        .parse::<i64>()
        .unwrap()
}

fn simplify(
    to_simplify: String,
    parentheses_shift: usize,
    parentheses_map: &HashMap<usize, usize>,
    addition_first: bool,
) -> String {
    let mut s = String::new();
    let mut i = 0;
    let chars = to_simplify.chars().collect::<Vec<_>>();

    while i < to_simplify.len() {
        let c = chars[i];

        match c {
            '(' => {
                // Since this method is called recursive, we need to know how much of the string
                // length is cut off so we can properly find parenthesis and their matching friend
                // in the lookup map.
                let parentheses_i = i + parentheses_shift;
                let block = &chars[i + 1..parentheses_map[&parentheses_i] - parentheses_shift]
                    .iter()
                    .collect::<String>();

                let simplified = simplify(
                    block.to_string(),
                    parentheses_shift + i + 1,
                    parentheses_map,
                    addition_first,
                );

                s.push_str(&simplified);
                i = parentheses_map[&parentheses_i] - parentheses_shift;
            }
            ')' => (),
            _ => s.push(c),
        }

        i += 1;
    }

    if addition_first {
        with_addition_precedence(s).to_string()
    } else {
        left_to_right(s).to_string()
    }
}

fn with_addition_precedence(exp: String) -> i64 {
    let mut tokens = exp
        .split_whitespace()
        .map(|c| c.to_string())
        .collect::<Vec<_>>();

    let mut i = 0;
    while i < tokens.len() {
        if tokens.len() < 3 || i >= tokens.len() - 2 {
            break;
        }

        let token = tokens[i].parse::<i64>().unwrap();
        let op = &tokens[i + 1];
        let next = tokens[i + 2].parse::<i64>().unwrap();

        match op.as_str() {
            "+" => {
                let mut n = tokens[..i].to_vec();
                n.push((token + next).to_string());
                n.extend_from_slice(&tokens[i + 3..]);
                tokens = n.clone();
            }
            _ => i += 2,
        }
    }

    left_to_right(tokens.join(" "))
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

        if tokens.len() < 3 || i >= tokens.len() - 2 {
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
        let cases = vec![
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];

        for (tc, result) in cases {
            assert_eq!(super::part_two(tc.to_string()), result);
        }
    }
}
