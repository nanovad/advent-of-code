use std::{fs::File, io::Read, ops::Add};
use log::info;

pub fn day1() {
    info!("Day 1");
    let mut f = File::open("input/day1.txt").unwrap();
    let mut buf = String::new();

    let mut accu: u128 = 0;
    
    f.read_to_string(&mut buf).unwrap();
    for line in buf.split("\n") {
        let s = String::from(line);
        let first_idx = s.find(|c: char| c.is_numeric());
        let last_idx = s.rfind(|c: char| c.is_numeric());

        if first_idx.is_none() {
            continue;
        }

        let first = s.chars().nth(first_idx.unwrap()).unwrap();
        let last = s.chars().nth(last_idx.unwrap()).unwrap();

        let mut combined = String::from(first);
        combined.push(last);
        
        accu = accu.add(combined.parse::<u128>().unwrap());
    }

    log::info!("Answer: {}", accu);
    log::info!("");
}
