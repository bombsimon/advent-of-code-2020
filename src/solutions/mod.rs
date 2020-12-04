mod day01;
mod day02;
mod day03;
mod day04;

pub fn solution_for(day: i32) {
    println!("Showing solution for day {}\n", day);

    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        d => panic!(format!("Day {} not implemented", d)),
    }
}
