mod awawa;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
const REPEAT: i32 = 1;
use colored::{ColoredString, Colorize};

fn performance(
    ((x, y), (mut t1, mut t2, mut t3, mut t4)): (
        (i64, i64),
        (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>),
    ),
    day: i64,
) {
    t1.sort();
    t2.sort();
    t3.sort();
    t4.sort();
    let day = match day % 5 {
        0 => format!("Day {}", day).red(),
        1 => format!("Day {}", day).yellow(),
        2 => format!("Day {}", day).blue(),
        3 => format!("Day {}", day).magenta(),
        4 => format!("Day {}", day).cyan(),
        _ => format!("Day {}", day).black(),
    };
    let plusminus = format!("±").red();
    let text = format!(
        "{day} Part 1: {} Part 2: {}
{day} Read: {}μs {plusminus}{}μs 
{day} Cleanup: {}μs {plusminus}{}μs 
{day} Part 1: {}μs {plusminus}{}μs 
{day} Part 2: {}μs {plusminus}{}μs ",
        x.to_string().green(),
        y.to_string().green(),
        c(t1[(REPEAT / 2) as usize] as f64 / 1000.0),
        c(t1[(REPEAT / 2) as usize] as f64 / 1000.0 - t1[(REPEAT / 10) as usize] as f64 / 1000.0),
        c(t2[(REPEAT / 2) as usize] as f64 / 1000.0),
        c(t2[(REPEAT / 2) as usize] as f64 / 1000.0 - t2[(REPEAT / 10) as usize] as f64 / 1000.0),
        c(t3[(REPEAT / 2) as usize] as f64 / 1000.0),
        c(t3[(REPEAT / 2) as usize] as f64 / 1000.0 - t3[(REPEAT / 10) as usize] as f64 / 1000.0),
        c(t4[(REPEAT / 2) as usize] as f64 / 1000.0),
        c(t4[(REPEAT / 2) as usize] as f64 / 1000.0 - t4[(REPEAT / 10) as usize] as f64 / 1000.0),
    );
    println!("{}", text);
}

fn c(value: f64) -> ColoredString {
    format!("{:.2}", value).green()
}

fn main() {
    // performance(day1::run(), 1);
    // performance(day2::run(), 2);
    // performance(day3::run(), 3);
    // performance(day4::run(), 4);
    // performance(day5::run(), 5);
    // performance(day6::run(), 6);
    // performance(day7::run(), 7);
    // performance(day8::run(), 8);
    // performance(day9::run(), 9);
    // performance(day10::run(), 10);
    // performance(day11::run(), 11);
    // performance(day12::run(), 12);
    // performance(day13::run(), 13);
    // performance(day14::run(), 14);
    // performance(day15::run(), 15);
    // performance(day16::run(), 16);
    // performance(day17::run(), 17);
    // day18::run();
    // performance(day19::run(), 19);
    day20::run();
    // awawa::test();
    let repeat = REPEAT.to_string().red();
    println!("Repeat: {repeat}");
}
