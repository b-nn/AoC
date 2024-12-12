use crate::REPEAT;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn update(input: &mut HashMap<u64, i64>) {
    let mut temp = HashMap::new();
    for i in input.keys() {
        if *i == 0 {
            *temp.entry(1).or_insert(0) += input.get(&0).unwrap();
            continue;
        }
        if i.ilog10() % 2 == 0 {
            let power = 10u64.pow(i.ilog10() / 2 + 1); // I stole the power trick I'm not that
                                                       // smart
            *temp.entry(i / power).or_insert(0) += input.get(i).unwrap();
            *temp.entry(i % power).or_insert(0) += input.get(i).unwrap();
            continue;
        }
        *temp.entry(i * 2024).or_insert(0) += input.get(i).unwrap();
    }

    *input = temp;
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let mut content =
            fs::read_to_string("day11.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        content.pop();
        let content = content
            .split(' ')
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let mut stones: HashMap<u64, i64> = HashMap::new();
        for i in &content {
            *stones.entry(*i).or_insert(0) += 1;
        }
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        for _i in 0..25 {
            update(&mut stones);
        }
        for j in stones.values() {
            part1 += *j as i64;
        }
        part1t.push(now.elapsed().as_nanos());

        let mut stones: HashMap<u64, i64> = HashMap::new();
        for i in &content {
            *stones.entry(*i).or_insert(0) += 1;
        }

        let now = Instant::now();
        for _i in 0..75 {
            update(&mut stones);
        }
        for j in stones.values() {
            part2 += *j;
        }
        part2t.push(now.elapsed().as_nanos());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
