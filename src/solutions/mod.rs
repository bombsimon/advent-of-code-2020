mod day01;
mod day02;

pub fn solution_for(day: i32) {
    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        d => panic!(format!("Day {} not implemented", d)),
    }
}
