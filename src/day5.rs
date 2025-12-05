use std::fs::read_to_string;
use std::range::{Range, RangeInclusive};

pub fn run() {
    let str = read_to_string("./input/5").unwrap();

    let mut lines = str.lines();

    let mut ranges: Vec<RangeInclusive<u64>> = Vec::new();
    while let Some(line) = lines.next() && !line.is_empty() {
        let (l,r) = line.split_once('-').unwrap();
        ranges.push(RangeInclusive::from(l.parse::<u64>().unwrap()..=r.parse::<u64>().unwrap()));
    }

    let ids = lines.map(|id| id.parse::<u64>().unwrap()).collect::<Vec<u64>>();
    // Part 1 - find all items in ranges
    let mut fresh = 0;
    for id in ids {
        if ranges.iter().find(|range| range.contains(&id)).is_some() {
            fresh += 1;
        }
    }
    println!("5.1: {}", fresh);

    // Part 2 - Eliminate all range overlap and total how many ids in all ranges
    let mut unique_ranges: Vec<RangeInclusive<u64>> = vec![];
    // lets figure out which ranges overlap
    ranges.sort_by(|l,r| l.start.cmp(&r.start));
    for range in ranges {
        if let Some(prev) = unique_ranges.last_mut() && prev.last >= range.start {
            prev.last = range.last.max(prev.last); // merge into one larger range instead
            continue;
        }
        unique_ranges.push(range);
    }

    let part2 = unique_ranges.iter()
        .fold(0, |acc, range| acc + (range.last - range.start)+1);
    println!("5.2: {}", part2);
}