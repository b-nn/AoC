use std::fs;
use std::time::Instant;

use crate::REPEAT;

fn update(i: &mut Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    let mut okay = true;
    for j in rules {
        let pos_1 = match i.iter().position(|x| x == &j[0]) {
            Some(x) => x,
            None => continue,
        };
        let pos_2 = match i.iter().position(|x| x == &j[1]) {
            Some(x) => x,
            None => continue,
        };
        if pos_1 > pos_2 {
            okay = false;
            // continue;
            i.swap(pos_1, pos_2);
        }
    }
    okay
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];

    let mut result = (0, 0);

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day5_p1.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        let rules =
            fs::read_to_string("day5_p2.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let rules = rules
            .lines()
            .map(|x| {
                x.split('|')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut content = content
            .lines()
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        result = (0, 0);
        for i in &mut content {
            let mut okay = true;

            while !update(i, &rules) {
                okay = false;
            }

            if !okay {
                result.1 += i[i.len() / 2] as i64;
            } else {
                result.0 += i[i.len() / 2] as i64;
            }
        }
        part1t.push(now.elapsed().as_nanos());
        part2t.push(now.elapsed().as_nanos());
    }

    (result, (read, cleanup, part1t, part2t))
}
