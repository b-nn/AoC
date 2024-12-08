// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
mod day7;
const REPEAT: i32 = 10000;

fn main() {
    let ((x, y), (mut t1, mut t2, mut t3, mut t4)) = day7::run();
    t1.sort();
    t2.sort();
    t3.sort();
    t4.sort();
    println!("Part 1: {} \nPart 2: {}", x, y);
    println!(
        "Read: {:.2} ±{:.2}\nCleanup: {:.2} ±{:.2}\nPart 1: {:.2} ±{:.2}\nPart 2: {:.2} ±{:.2}",
        t1[(REPEAT / 2) as usize] as f64 / 1000.0,
        t1[(REPEAT / 2) as usize] as f64 / 1000.0 - t1[(REPEAT / 10) as usize] as f64 / 1000.0,
        t2[(REPEAT / 2) as usize] as f64 / 1000.0,
        t2[(REPEAT / 2) as usize] as f64 / 1000.0 - t2[(REPEAT / 10) as usize] as f64 / 1000.0,
        t3[(REPEAT / 2) as usize] as f64 / 1000.0,
        t3[(REPEAT / 2) as usize] as f64 / 1000.0 - t3[(REPEAT / 10) as usize] as f64 / 1000.0,
        t4[(REPEAT / 2) as usize] as f64 / 1000.0,
        t4[(REPEAT / 2) as usize] as f64 / 1000.0 - t4[(REPEAT / 10) as usize] as f64 / 1000.0,
    );
}
