mod day01;

pub fn solution_for(day: i32) {
    match day {
        1 => day01::solve(),
        d => panic!(format!("Day {} not implemented", d)),
    }
}
