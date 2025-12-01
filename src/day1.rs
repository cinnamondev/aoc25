use std::fs::File;
use std::io::Read;

pub fn run() {
    let mut file = File::open("./input/1").expect("No read");
    let mut string = String::new();

    let mut counter_special = 0;
    let mut counter = 0;
    file.read_to_string(&mut string).expect("IO error");
    let mut vec: i32 = string.split_whitespace()
        .map(|c| {
            let int = c[1..].parse::<i32>().unwrap();
            if c.as_bytes()[0] == b'L' {
                -1 * int
            } else {
                int
            }
        })
        .fold(50, |acc, v| {
            let orig = v;
            let mut full_rotations = (v / 100).abs();
            let v = v % 100; // filter out full rotations#
            let mut new = acc + v;
            new = if new == 100 {
                0
            } else if new > 100 {
                println!("beep");
                full_rotations += 1;
                0 + ((100-acc).abs_diff(v) as i32)
            } else if new < 0 {
                if acc != 0 {
                    println!("boop");
                    full_rotations += 1;
                }
                100 - (acc.abs_diff(-1 * v) as i32)
            } else {
                new
            };
            if new == 0 { counter += 1; }
            counter_special += full_rotations;
            println!("{0} {1} {2} {3} {4}", acc, v, orig, new, full_rotations);
            new
        });

    println!("CODE IS {0}", counter);
    println!("CODE(2) is {0}", counter + counter_special);
}