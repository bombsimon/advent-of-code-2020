use crate::input;

use std::collections::HashMap;

#[derive(Debug, Default)]
struct StateMachine {
    acc: i32,
    pos: usize,
    program: Vec<(String, i32)>,
    original_program: Vec<(String, i32)>,
    visited: HashMap<i32, bool>,
}

impl StateMachine {
    fn new(input: String) -> StateMachine {
        let p: Vec<(String, i32)> = input
            .lines()
            .filter(|&c| c != "")
            .map(|l| {
                let mut row = l.split_whitespace();
                let instruction = row.next().unwrap();
                let step = row.next().unwrap().parse::<i32>().unwrap();

                (instruction.to_string(), step)
            })
            .collect();

        StateMachine {
            acc: 0,
            pos: 0,
            original_program: p.clone(),
            program: p,
            visited: HashMap::new(),
        }
    }

    fn run(&mut self) {
        let mut i = 0;

        while !self.visited.contains_key(&i) && i < self.program.len() as i32 {
            let (ins, step) = &self.program[i as usize];
            self.visited.insert(i, true);

            match ins.as_str() {
                "nop" => (),
                "acc" => self.acc += step,
                "jmp" => i += step - 1,
                x => panic!(format!("Unexpected instruction: {}", x)),
            }

            i += 1;
        }

        self.pos = i as usize;
    }

    fn change_instruction(&mut self, from: String, to: String, after: usize) {
        self.program = self.original_program.clone();
        self.acc = 0;
        self.visited = HashMap::new();

        let mut seen = 1usize;

        for (ins, _) in &mut self.program {
            if ins == &from {
                if seen == after {
                    *ins = to;
                    break;
                } else {
                    seen += 1;
                }
            }
        }
    }

    fn did_complete(&self) -> bool {
        self.pos >= self.program.len()
    }
}

pub fn solve() {
    let x = input::file_for_day(8).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i64 {
    let mut sm = StateMachine::new(input);
    sm.run();

    sm.acc as i64
}

fn part_two(input: String) -> i64 {
    let mut sm = StateMachine::new(input);
    let mut tries = 1;

    sm.run();

    while !sm.did_complete() {
        sm.change_instruction("jmp".to_string(), "nop".to_string(), tries);
        sm.run();
        tries += 1;
    }

    sm.acc as i64
}

#[cfg(test)]
mod tests {
    static TEST_INPUT: &str = r#"
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"#;

    static SOLUTION_ONE: i64 = 5;
    static SOLUTION_TWO: i64 = 8;

    #[test]
    fn part_one() {
        assert_eq!(super::part_one(TEST_INPUT.to_string()), SOLUTION_ONE);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::part_two(TEST_INPUT.to_string()), SOLUTION_TWO);
    }
}
