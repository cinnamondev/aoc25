use std::fs::File;
use std::io::Read;
use std::iter::iter;
use std::num::ParseIntError;
use std::ops::{Range, RangeInclusive};

pub fn run() {
    let mut file = File::open("./input/2").expect("No read");
    let mut string = String::new();
    file.read_to_string(&mut string).expect("IO Error reading file");

    let ranges = string.split(',')
        .filter_map(|str| str.split_once('-'))
        .map(|(l,r)| l.parse::<u64>().unwrap()..=r.parse::<u64>().unwrap());

    let part1 = ranges.clone()
        .flat_map(|r| find_all_patterned(r, part1_patterned).into_iter())
        .fold(0, |acc, i| acc + i);
    println!("day 2.1 {}", part1);

    let part2 = ranges
        .flat_map(|r| find_all_patterned(r, part2_patterned).into_iter())
        .fold(0, |acc, i| acc + i);
    println!("day 2.2 {}", part2);
}

pub fn find_all_patterned<T>(range: RangeInclusive<u64>, condition: T) -> Vec<u64> where T: Fn(u64) -> bool {
    range.filter(|n| condition(*n))
        .collect()
}

pub fn part2_patterned(num: u64) -> bool {
    let string = num.to_string();

    for chunk_size in 1..=string.len() / 2 { // increasing pattern width with each iter
        if string.len() % chunk_size != 0 { continue; }

        let mut is_invalid = true;
        let mut chunks = string.as_bytes().chunks_exact(chunk_size);
        let mut last_chunk = chunks.next().unwrap();
        for chunk in chunks {
            //println!("chunk: {:?} vs {:?}", chunk, last_chunk);
            if chunk != last_chunk {
                is_invalid = false;
                break;
                //return false;
            }
            last_chunk = chunk;
        }
        if is_invalid { return true; } // we are looking to see when it is a invalid
    }
    false
}
pub fn part1_patterned(num: u64) -> bool {
    let chars = num.to_string();
    let (l,r) = chars.split_at(chars.len()/2);
    if l.len() != r.len() { return false; }
    let l_digit = l.parse::<u64>().unwrap();
    let r_digit = r.parse::<u64>().unwrap();
    l_digit == r_digit
}