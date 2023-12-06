use aho_corasick::AhoCorasickBuilder;
use log::info;
use std::{fs::File, io::Read, ops::Add};

pub fn day1() {
    info!("Day 1");
    let mut f = File::open("input/day1.txt").unwrap();
    let mut buf = String::new();

    f.read_to_string(&mut buf).unwrap();

    let mut part1_accu: u64 = 0;
    let mut part2_accu: u64 = 0;

    let number_words = &[
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];
    let number_digits = &["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let ac = AhoCorasickBuilder::new()
        .match_kind(aho_corasick::MatchKind::Standard)
        .build(number_words)
        .unwrap();

    for line in buf.split("\n") {
        let s = String::from(line);

        // Part 1: Only include actual digits in the calculation.
        let part1_sfl = sum_first_last(&s).unwrap_or(0u64);
        part1_accu = part1_accu.add(part1_sfl);


        // Part 2: Include words representing digits as well.

        // The Aho-Corasick automaton searches the input string for overlapping patterns, which in our case, are words
        // representing digits. The input string is each line of the input file. By replacing those digit words with
        // their numerical representations, we can build a string of only digits that will work with the generic
        // sum_first_last function. Digits already existing in the input string are replaced with themselves, to ensure
        // that the original digits are not lost in the destructive transformation. A trick with modulo allows
        // number_digits to be smaller than number_words by wrapping the originating match's index.
        let dig = ac
            .find_overlapping_iter(&s)
            .map(|f| number_digits[f.pattern().as_usize() % 9])
            .collect::<Vec<&str>>()
            .concat();
        let part2_sfl = sum_first_last(&dig).unwrap_or(0u64);
        part2_accu = part2_accu.add(part2_sfl);
    }

    log::info!("Part 1: {}", part1_accu);
    log::info!("Part 2: {}", part2_accu);
    log::info!("");
}

fn sum_first_last(s: &String) -> Option<u64> {
    // Get the index of the first and last digit in the input string.
    let first_idx = s.find(|c: char| c.is_digit(10));
    // If no digit was found searching forwards, no digit exists in the string.
    if first_idx.is_none() {
        return None;
    }
    let last_idx = s.rfind(|c: char| c.is_digit(10));

    // Get the first and last character, i.e. the digit itself, from its index in the string.
    let first = s.chars().nth(first_idx.unwrap()).unwrap();
    let last = s.chars().nth(last_idx.unwrap()).unwrap();

    // Combine them into one string.
    let mut combined = String::from(first);
    combined.push(last);

    // And parse that string into a u64.
    combined.parse::<u64>().ok()
}
