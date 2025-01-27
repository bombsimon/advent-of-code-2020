use std::fs::File;
use std::io::{stdin, stdout, BufRead, BufReader, Write};

pub fn file_for_day(day: i32) -> Vec<String> {
    let input_file = format!("input/day{:02}", day);
    let f = File::open(input_file).unwrap();
    let r = BufReader::new(f);

    r.lines().filter_map(|l| l.ok()).collect()
}

pub fn file_for_day_as_int(day: i32) -> Vec<i32> {
    string_vec_to_int_vec(file_for_day(day).as_ref())
}

pub fn string_vec_to_int_vec<S: AsRef<str>>(vec: &[S]) -> Vec<i32> {
    vec.into_iter()
        .filter_map(|x| x.as_ref().parse::<i32>().ok())
        .collect()
}

#[allow(unused)]
pub fn print_and_wait(print: String) {
    let mut s = String::new();

    println!("{}", print);
    print!("❯ PRESS ENTER TO CONTINUE...");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
}

#[cfg(test)]
pub fn string_to_vec(s: &str, keep_empty_lines: bool) -> Vec<String> {
    s.split("\n")
        .filter(|&x| keep_empty_lines || x != "")
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
}
