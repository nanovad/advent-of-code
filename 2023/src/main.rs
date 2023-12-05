use aoc2023::day1;

fn main() {
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();

    println!("Advent of Code 2023");

    day1::day1();
}
