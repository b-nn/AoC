mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
const REPEAT: i32 = 1000;
use colored::Colorize;

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
        "{day} Part 1: {x} Part 2: {y} 
{day} Read: {:.2} {plusminus}{:.2} 
{day} Cleanup: {:.2} {plusminus}{:.2} 
{day} Part 1: {:.2} {plusminus}{:.2} 
{day} Part 2: {:.2} {plusminus}{:.2} ",
        t1[(REPEAT / 2) as usize] as f64 / 1000.0,
        t1[(REPEAT / 2) as usize] as f64 / 1000.0 - t1[(REPEAT / 10) as usize] as f64 / 1000.0,
        t2[(REPEAT / 2) as usize] as f64 / 1000.0,
        t2[(REPEAT / 2) as usize] as f64 / 1000.0 - t2[(REPEAT / 10) as usize] as f64 / 1000.0,
        t3[(REPEAT / 2) as usize] as f64 / 1000.0,
        t3[(REPEAT / 2) as usize] as f64 / 1000.0 - t3[(REPEAT / 10) as usize] as f64 / 1000.0,
        t4[(REPEAT / 2) as usize] as f64 / 1000.0,
        t4[(REPEAT / 2) as usize] as f64 / 1000.0 - t4[(REPEAT / 10) as usize] as f64 / 1000.0,
    );
    println!("{}", text);
}

fn main() {
    performance(day1::run(), 1);
    performance(day2::run(), 2);
    performance(day3::run(), 3);
    performance(day4::run(), 4);
    performance(day5::run(), 5);
    // performance(day6::run(), 6);
    performance(day7::run(), 7);
    performance(day8::run(), 8);
    let repeat = REPEAT.to_string().red();
    println!("Repeat: {repeat}");
}
