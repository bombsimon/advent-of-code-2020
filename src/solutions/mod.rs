mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day05_alt;
mod day06;
mod day07;

pub fn solution_for(day: i32) {
    println!("Showing solution for day {}\n", day);

    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => {
            println!("Solution #1");
            day05::solve();

            println!("\nSolution #2");
            day05_alt::solve();
        }
        6 => day06::solve(),
        7 => day07::solve(),
        d => panic!(format!("Day {} not implemented", d)),
    }
}
