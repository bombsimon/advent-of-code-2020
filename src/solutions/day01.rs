use crate::input;

pub fn solve() {
    let x = input::file_for_day_as_int(1);

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(vec: Vec<i32>) -> i32 {
    for (i, _) in vec.iter().enumerate() {
        for j in i..vec.len() {
            let i_val = vec.get(i).unwrap();
            let j_val = vec.get(j).unwrap();

            if i_val + j_val == 2020 {
                return i_val * j_val;
            }
        }
    }

    -1
}

fn part_two(vec: Vec<i32>) -> i32 {
    for (i, _) in vec.iter().enumerate() {
        for j in i..vec.len() {
            for k in j..vec.len() {
                let i_val = vec.get(i).unwrap();
                let j_val = vec.get(j).unwrap();
                let k_val = vec.get(k).unwrap();

                if i_val + j_val + k_val == 2020 {
                    return i_val * j_val * k_val;
                }
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    static SOLUTION_ONE: i32 = 514579;
    static SOLUTION_TWO: i32 = 241861950;
    static TEST_INPUT: &str = r#"
1721
979
366
299
675
1456
"#;

    #[test]
    fn part_one() {
        let x = input::string_vec_to_int_vec(TEST_INPUT.split("\n").collect::<Vec<_>>().as_ref());
        assert_eq!(super::part_one(x), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        let x = input::string_vec_to_int_vec(TEST_INPUT.split("\n").collect::<Vec<_>>().as_ref());
        assert_eq!(super::part_two(x), SOLUTION_TWO);
    }
}
