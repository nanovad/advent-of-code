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

    let mut accu: u64 = 0;

    // For getting the game ID itself as a capture group.
    let game_regex = Regex::new(r"^Game (\d+):").unwrap();
    // For getting the number of cubes and the name of those cubes in a set.
    let cube_regex = Regex::new(r"(\d+) (\w+)").unwrap();

    for line in buf.trim_end().split("\n") {
        let game_captures = &game_regex.captures(line).unwrap();
        let game_id_str = &game_captures[1];
        debug!("Game ID str: {}", game_id_str);

        let game_id: u16 = game_id_str.parse::<u16>().unwrap();

        // Skip over the "Game n: " portion of the string and pass the rest to game_possible.
        let remainder = &line[game_captures[0].len() + 1..];
        debug!("Remainder: {}", remainder);

        accu += match game_possible(remainder, &cube_regex) {
            true => game_id as u64,
            false => 0u64,
        };
    }
    info!("Part 1: {}", accu);
    info!("");
}

fn game_possible(line: &str, cube_regex: &Regex) -> bool {
    debug!("Checking if game is possible (line={})", line);
    let sets = line.split(";");
    debug!("sets: {:?}", sets);
    for set in sets {
        for cap in cube_regex.captures_iter(set) {
            debug!("cube and count capture: {:?}", cap);

            let cube_n = cap[1].parse::<u16>().unwrap();
            let cube_name = &cap[2];

            let lim = match cube_name {
                "red" => LIM_RED,
                "green" => LIM_GREEN,
                "blue" => LIM_BLUE,
                _ => 0,
            };

            debug!("Limit for {}: {}", cube_name, lim);

            if cube_n > lim {
                debug!("Exceeds limit");
                return false;
            }
        }
    }

    debug!("Game works");
    true
}
