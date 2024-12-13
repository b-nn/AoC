use crate::REPEAT;
use std::fs;
use std::time::Instant;

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day13.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let t = content
            .replace("X+", "")
            .replace("Y+", "")
            .replace("X=", "")
            .replace("Y=", "")
            .replace("Button A: ", "")
            .replace("Button B: ", "")
            .replace("Prize: ", "");
        let content = t.lines().filter(|x| *x != "");
        println!("{:?}", content);
        for (index, item) in content.enumerate() {
            let mut t = item.split(", ");
            let x = t.next().unwrap().parse::<i32>().unwrap();
            let y = t.next().unwrap().parse::<i32>().unwrap();
            println!("{}, {}", x, y);
        }
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
