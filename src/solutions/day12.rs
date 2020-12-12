use crate::{grid::Direction, input};

#[derive(Clone, Debug)]
struct Walker {
    instructions: Vec<(Direction, usize)>,
    instructions_ptr: usize,
    facing: Direction,
    start_x: usize,
    start_y: usize,
    x: usize,
    y: usize,
}

impl Walker {
    fn new(instructions: Vec<(Direction, usize)>) -> Self {
        Walker {
            instructions,
            instructions_ptr: 0usize,
            facing: Direction::East,
            start_x: 1000,
            start_y: 1000,
            x: 1000,
            y: 1000,
        }
    }

    #[allow(dead_code)]
    fn debug_and_wait(&self) {
        println!("{}", self);
        print!("PRESS ENTER TO CONTINUE");

        use std::io::{stdin, stdout, Write};

        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");
    }

    fn next(&mut self) {
        let (ins, len) = self.instructions[self.instructions_ptr];
        let (new_x, new_y) = match ins {
            Direction::Unknown('F') => self.facing.next_from(self.x, self.y, len),
            Direction::Unknown('L') => {
                // Just turn right and another 180 degrees
                if len != 180 {
                    self.facing = self.facing.turn_deegrees(180);
                }

                self.facing = self.facing.turn_deegrees(len);
                self.instructions_ptr += 1;
                return;
            }
            Direction::Unknown('R') => {
                self.facing = self.facing.turn_deegrees(len);
                self.instructions_ptr += 1;
                return;
            }
            direction => direction.next_from(self.x, self.y, len as usize),
        };

        self.x = new_x;
        self.y = new_y;
        self.instructions_ptr += 1;
    }

    fn done(&self) -> bool {
        self.instructions_ptr == self.instructions.len()
    }

    #[allow(dead_code)]
    fn manhattan(&self) -> i32 {
        (self.start_x as i32 - self.x as i32).abs() + (self.start_y as i32 - self.y as i32).abs()
    }
}

impl std::fmt::Display for Walker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "at x: {}, y: {}, facing: {:?}",
            self.x, self.y, self.facing
        )
    }
}

pub fn solve() {
    let x = input::file_for_day(12).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i32 {
    let sequence = input
        .lines()
        .filter(|&c| c != "")
        .map(|l| {
            let ins = l.chars().next().unwrap();
            let len = l.get(1..).unwrap().parse::<usize>().unwrap();

            (Direction::from(ins), len)
        })
        .collect::<Vec<(Direction, usize)>>();

    let mut walker = Walker::new(sequence);

    while !walker.done() {
        walker.next();
    }

    walker.manhattan()
}

fn part_two(_input: String) -> i32 {
    -1
}

#[cfg(test)]
mod tests {
    static TEST_INPUT_ONE: &str = r#"
F10
N3
F7
R90
F11"#;
    static TEST_INPUT_TWO: &str = r#""#;

    static SOLUTION_ONE: i32 = 25;
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
