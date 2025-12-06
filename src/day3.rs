use std::fs::read_to_string;
use std::ops::RangeTo;
use std::range::Range;

pub fn run() {
    let input = read_to_string("./input/3").unwrap();
    let banks = input.lines()
        .map(|line| line.chars().map(|c| (c as u8) - 48).collect())
        .collect::<Vec<Vec<u8>>>();

    let part1 = banks.iter()
        .map(|bank| find_largest_possible(bank,2))
        .fold(0u64,|total, joltage| total + joltage);
    println!("Part 1: {}", part1);

    let part2 = banks.iter()
        .map(|bank| find_largest_possible(bank,12))
        .fold(0u64,|total, joltage| total + joltage);
    println!("Part 2: {}", part2);

} 
fn find_largest_possible(bank: &Vec<u8>, digit_count: usize) -> u64 {
    let mut string = String::with_capacity(bank.len());

    let mut slice = bank.as_slice();
    for i in (1..=digit_count).rev() {
        let mut next = 0;
        (next, slice) = get_next_digit(slice, i);
        string.push((next + 48 ) as char);
    }
    string.parse::<u64>().unwrap() // janky but whatevs.
}
fn get_next_digit(bank: &[u8], n_remaining: usize) -> (u8, &[u8]) {
    let mut n_ahead = 0;
    let mut cur_max_search = bank.len();
    let (mut pos, mut max) = (0, 0);
    while n_remaining > n_ahead { // find the largest starting digit that will allow us to get at least `n_remaining` digits.
        (pos, max) = get_max_in_bank(bank, Range::from(0..cur_max_search));
        n_ahead = bank.len() - pos;
        cur_max_search = pos;
    }

    (max, &bank[cur_max_search + 1..])
}
fn get_max_in_bank(bank: &[u8], range: core::range::Range<usize>) -> (usize,u8) {
    let mut max = 0;
    let mut index = usize::MAX;
    for i in range {
        if bank[i] > max { // dont do >= we want the FIRST largest not last largest.
            max = bank[i];
            index = i;
        }
    }
    (index,max)
}