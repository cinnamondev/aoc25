#![feature(iter_macro)]
#![feature(yield_expr)]
#![feature(string_into_chars)]

mod day1;
mod day2;

fn main() {
    println!("DAY1");
    day1::run();

    println!("DAY2");
    day2::run();
}
