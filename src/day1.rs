use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use std::vec;

use crate::REPEAT;

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut result1 = 0;
    let mut result2 = 0;
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    for i in 0..REPEAT {
        let now = Instant::now();
        let contents = fs::read_to_string("day1.txt").expect("FUCK OFFFFFF");
        read.push(now.elapsed().as_nanos());

        let mut part1: Vec<i32> = vec![];
        let mut part2: Vec<i32> = vec![];
        let now = Instant::now();
        contents.lines().for_each(|x| {
            let mut i = x.split("   ");
            part1.push(i.next().unwrap().parse::<i32>().unwrap());
            part2.push(i.next().unwrap().parse::<i32>().unwrap());
        });

        part1.sort();
        part2.sort();
        cleanup.push(now.elapsed().as_nanos());

        // part 1

        let now = Instant::now();
        result1 = part1
            .iter()
            .zip(part2.iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<i32>();
        part1t.push(now.elapsed().as_nanos());

        // part 2
        let now = Instant::now();
        let mut hash: HashMap<i32, i32> = HashMap::new();
        let mut sum = 0;
        for i in &part2 {
            *hash.entry(*i).or_insert(0) += 1;
        }
        for i in &part1 {
            match hash.get(i) {
                Some(x) => sum += i * x,
                None => continue,
            }
        }
        result2 = sum;
        part2t.push(now.elapsed().as_nanos());
    }

    return (
        (result1.into(), result2.into()),
        (read, cleanup, part1t, part2t),
    );
}
