use aoc2023::*;

fn main() {
    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::init_from_env(env);

    println!("Advent of Code 2023");

    day1::day1();
    day2::day2();
}
