use crate::input;

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Content {
    color: String,
    amount: i32,
}

pub fn solve() {
    let x = input::file_for_day(7).join("\n");
    let input = parse_input(x);

    println!("Solution part 1: {}", part_one(&input));
    println!("Solution part 2: {}", part_two(&input));
}

fn parse_input(input: String) -> HashMap<String, Vec<Content>> {
    let mut b = HashMap::new();

    for l in input.lines().filter(|&c| c != "") {
        // #0     |            | #1
        // <color> bags contain <n <color> bag(s)>[, n <color> bag(s)>]
        let mut parts = l.split(" bags contain ");
        let outer_bag_color = parts.next().unwrap().to_string();
        let contents = parts.next().unwrap();

        let mut map_content: Vec<Content> = Vec::new();

        if contents == "no other bags." {
            b.insert(outer_bag_color, map_content);
            continue;
        }

        // Iterate over each group of bags.
        for content in contents.split(",") {
            let mut content_parts = content.split_whitespace();
            let amount = content_parts.next().unwrap().parse::<i32>().unwrap();

            let content_color = content_parts.take(2).collect::<Vec<_>>().join(" ");

            map_content.push(Content {
                color: content_color,
                amount,
            });
        }

        b.insert(outer_bag_color, map_content);
    }

    b
}

fn part_one(input: &HashMap<String, Vec<Content>>) -> i64 {
    // Recursive function to follow a bags content until "shiny gold" is found.
    fn has_gold(contents: &Vec<Content>, b: &HashMap<String, Vec<Content>>) -> bool {
        for content in contents {
            if content.color == "shiny gold" {
                return true;
            }

            match b.get(&content.color) {
                Some(c) => {
                    if has_gold(&c, b) {
                        return true;
                    }
                }
                None => continue,
            };
        }

        false
    };

    // Each color and it's content represents one line in the file (no duplicates) so iterate over
    // them and do a recursive check for "shiny gold".
    input.iter().fold(
        0,
        |acc, (_, v)| if has_gold(&v, input) { acc + 1 } else { acc },
    )
}

fn part_two(input: &HashMap<String, Vec<Content>>) -> i64 {
    // Recursive function to the number of bags found as content until no more bags are found.
    fn get_count(contents: &Vec<Content>, b: &HashMap<String, Vec<Content>>) -> i32 {
        let mut sum = 0;

        for content in contents {
            sum += content.amount;

            match b.get(&content.color) {
                Some(c) => {
                    sum += get_count(&c, b) * content.amount;
                }
                None => continue,
            };
        }

        sum
    };

    let gold_content = input
        .get(&"shiny gold".to_string())
        .expect("no shiny gold in input");

    get_count(&gold_content, input) as i64
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_ONE: &str = r#"
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#;

    static TEST_INPUT_TWO: &str = r#"
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#;

    static SOLUTION_ONE: i64 = 4;
    static SOLUTION_TWO: i64 = 126;

    #[test]
    fn part_one() {
        let input = super::parse_input(TEST_INPUT_ONE.to_string());
        assert_eq!(super::part_one(&input), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let input = super::parse_input(TEST_INPUT_TWO.to_string());
        assert_eq!(super::part_two(&input), SOLUTION_TWO);
    }
}
