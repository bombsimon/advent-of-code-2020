use crate::input;

pub fn solve() {
    let x = input::file_for_day(15).join("\n");

    println!("Solution part 1: {}", part_one(x.clone()));
    println!("Solution part 2: {}", part_two(x.clone()));
}

fn part_one(input: String) -> i32 {
    let mut register: [i32; 2048] = [0; 2048];
    let mut speak_map: [(i32, i32); 2048] = [(0, 0); 2048];
    let mut history = Vec::new();

    input
        .lines()
        .filter(|&c| c != "")
        .next()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<i32>().unwrap())
        .enumerate()
        .for_each(|(i, v)| {
            register[v as usize] += 1;
            speak_map[v as usize] = (speak_map[i as usize].1, (i + 1) as i32);
            history.push(v as usize);
        });

    for i in history.len() + 1..2020 + 1usize {
        let last_spoken = history[history.len() - 1];
        let to_speak: usize;

        match register[last_spoken] {
            1 => to_speak = 0,
            _ => {
                let (spoken_1, spoken_2) = speak_map[last_spoken];
                to_speak = (spoken_2 - spoken_1) as usize;
            }
        }

        register[to_speak] += 1;
        speak_map[to_speak] = (speak_map[to_speak].1, i as i32);
        history.push(to_speak);
    }

    history[history.len() - 1] as i32
}

fn part_two(input: String) -> i64 {
    use std::collections::HashMap;

    let mut register: HashMap<usize, usize> = HashMap::new();
    let mut speak_map: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut history = Vec::new();

    input
        .lines()
        .filter(|&c| c != "")
        .next()
        .unwrap()
        .split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .enumerate()
        .for_each(|(i, v)| {
            let old_register = match register.get(&v) {
                Some(m) => *m,
                None => 0,
            };

            let old_speak_map = match speak_map.get(&v) {
                Some(m) => m.1,
                None => 0,
            };

            register.insert(v, old_register + 1);
            speak_map.insert(v, (old_speak_map, i + 1));
            history.push(v as usize);
        });

    for i in history.len() + 1..30000000 + 1usize {
        let last_spoken = history[history.len() - 1];
        let to_speak: usize;

        match register.get(&last_spoken) {
            Some(1) => to_speak = 0,
            _ => {
                let (spoken_1, spoken_2) = speak_map.get(&last_spoken).unwrap();
                to_speak = spoken_2 - spoken_1;
            }
        }

        let old_register = match register.get(&to_speak) {
            Some(m) => *m,
            None => 0,
        };

        let old_speak_map = match speak_map.get(&to_speak) {
            Some(m) => m.1,
            None => 0,
        };

        register.insert(to_speak, old_register + 1);
        speak_map.insert(to_speak, (old_speak_map, i));
        history.push(to_speak);
    }

    history[history.len() - 1] as i64
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        let cases = vec![
            ("0,3,6", 436),
            ("1,3,2", 1),
            ("2,1,3", 10),
            ("1,2,3", 27),
            ("2,3,1", 78),
            ("3,2,1", 438),
            ("3,1,2", 1836),
        ];

        for tc in cases {
            assert_eq!(super::part_one(tc.0.to_string()), tc.1);
        }
    }

    #[test]
    fn part_two() {
        let cases = vec![
            ("0,3,6", 175594),
            ("1,3,2", 2578),
            ("2,1,3", 3544142),
            ("1,2,3", 261214),
            ("2,3,1", 6895259),
            ("3,2,1", 18),
            ("3,1,2", 362),
        ];

        for tc in cases {
            assert_eq!(super::part_two(tc.0.to_string()), tc.1);
        }
    }
}
