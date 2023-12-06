use std::{fs::File, io::Read};

use log::{debug, info};
use regex::Regex;

const LIM_RED: u16 = 12;
const LIM_GREEN: u16 = 13;
const LIM_BLUE: u16 = 14;

pub fn day2() {
    info!("Day 2");
    let mut f = File::open("input/day2.txt").unwrap();
    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    let mut part1_accu: u64 = 0;
    let mut part2_accu: u64 = 0;

    // For getting the game ID itself as a capture group.
    let game_regex = Regex::new(r"^Game (\d+):").unwrap();
    // For getting the number of cubes and the name of those cubes in a set.
    let cube_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    for line in buf.trim_end().split("\n") {
        let game_captures = &game_regex.captures(line).unwrap();
        let game_id_str = &game_captures[1];
        debug!("Game ID str: {}", game_id_str);

        let game_id: u16 = game_id_str.parse::<u16>().unwrap();

        // Skip over the "Game n: " portion of the string and pass the rest to the game parser.
        let remainder = &line[game_captures[0].len() + 1..];
        debug!("Remainder: {}", remainder);

        let min_cubes = game_minimum_cubes(remainder, &cube_regex);
        let possible = game_possible(min_cubes);

        part1_accu += match possible {
            true => game_id as u64,
            false => 0u64,
        };

        part2_accu += (min_cubes.0 * min_cubes.1 * min_cubes.2) as u64;
    }
    info!("Part 1: {}", part1_accu);
    info!("Part 2: {}", part2_accu);
    info!("");
}

fn game_minimum_cubes(line: &str, cube_regex: &Regex) -> (u16, u16, u16) {
    // Split the line at semicolons. Each item in the split represents a set of draws.
    line.split(";")
         .map(|set| {
            // For each set of draws, get the number of cubes used in that draw.
            let mut cubes_in_set = (0, 0, 0);
            // Iterate over each cube within the draw, assigning the count of
            // those cubes to its respective place in the tuple.
            for cap in cube_regex.captures_iter(set) {
                debug!("cube and count capture: {:?}", cap);

                let cube_count = cap[1].parse::<u16>().unwrap();
                let cube_name = &cap[2];

                match cube_name {
                    "red" => cubes_in_set.0 = cube_count,
                    "green" => cubes_in_set.1 = cube_count,
                    "blue" => cubes_in_set.2 = cube_count,
                    _ => (),
                }
            }
            debug!("Cubes in set: {:?}", cubes_in_set);

            cubes_in_set
        })
        // Reduce the elements in the set to the greater of each individual cube count.
        // This gets the minimum number of cubes necessary to get the provided set.
        .reduce(|acc: (u16, u16, u16), el: (u16, u16, u16)| {
            (acc.0.max(el.0), acc.1.max(el.1), acc.2.max(el.2))
        })
        .unwrap()
}

fn game_possible(cubes: (u16, u16, u16)) -> bool {
    // Check to see if all portions of a cube tuple are within limits.
    cubes.0 <= LIM_RED && cubes.1 <= LIM_GREEN && cubes.2 <= LIM_BLUE
}
