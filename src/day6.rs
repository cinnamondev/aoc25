use std::fs::read_to_string;
use std::ops::Mul;
use std::str::Chars;

pub fn run() {
    let str = read_to_string("./input/6").unwrap();

    let mut lines = str.lines().rev().map(|l| l.split_whitespace().collect::<Vec<&str>>());
    let mut vec1: Vec<(bool, Vec<u64>)> = lines.next().expect("cant get first line").iter()
        .map(|str| *str == "*")
        .map(|do_mul| (do_mul, Vec::new()))
        .collect();

    lines.for_each(|line| {
        for n in 0..line.len() {
            let num = line[n].parse::<u64>().unwrap();
            vec1[n].1.push(num);
        }
    });

    // Part1
    let part1 = vec1.iter()
        .fold(0u64, |acc,(do_mul,vec)|
            acc + vec.iter().fold(0u64, |acc,&n| {
                if *do_mul {
                    if acc == 0 { return n; }
                    acc * n
                } else {
                    acc + n
                }
            })
        );

    println!("Part 1: {}", part1);
    // Part2
    let mut lines2 = str.lines().rev().map(|l| l.chars());
    let mut operator_line = lines2.next().unwrap();

    let mut vec2: Vec<(bool, usize)> = Vec::new();
    let digit_grid: Vec<Vec<char>> = lines2.rev().map(|c| c.collect()).collect::<Vec<Vec<char>>>();
    let mut cur_do_mul = operator_line.next().unwrap() == '*'; // initial pos should always be a operator
    let mut column_width = 1;
    while let Some(c) = operator_line.next() {
        if (c.is_whitespace()) {
            column_width += 1;
        } else {
            vec2.push((cur_do_mul, column_width-1));
            cur_do_mul = (c=='*');
            column_width = 1;
        }
    }
    vec2.push((cur_do_mul,column_width+1)); // push remaining item

    // part2 column fixing
    let mut starting_column = 0;
    let mut total = 0;
    for (do_mul, column_width) in vec2 {
        // iterate vertically over digit grid for each 'column width'
        let mut prev = if do_mul { 1 } else { 0 };
        for i in 0..column_width {
            let mut string = digit_grid.iter().map(|row| row[i+starting_column]).collect::<String>();
            let value = string.trim().parse::<u128>().expect("bbbb");
            if do_mul { prev *= value; } else { prev += value }
        }
        starting_column += column_width+1;
        total += prev;
    }

    println!("Part2 {}", total);
}