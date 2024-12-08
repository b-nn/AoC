use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use std::vec;

use crate::REPEAT;

fn is_safe(vector: &Vec<i32>) -> Option<usize> {
    let sign = (vector[0] - vector[1]).signum();
    let mut index = 0;
    let mut safe = true;
    for j in 0..vector.len() - 1 {
        let diff = vector[j] - vector[j + 1];
        if diff == 0 || diff.abs() > 3 || diff.signum() != sign {
            index = j;
            safe = false;
            break;
        }
    }
    if safe {
        None
    } else {
        Some(index)
    }
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];

    let mut total_safe = (0, 0);

    for _i in 0..REPEAT {
        let now = Instant::now();
        let contents =
            fs::read_to_string("day2.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let contents_ver = contents
            .lines()
            .map(|x| {
                x.split(" ")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        total_safe = (0, 0);
        for (index, vector) in contents_ver.iter().enumerate() {
            if let Some(i) = is_safe(&vector) {
                for skip_idx in 0..vector.len() {
                    let subset: Vec<i32> = vector
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| *i != skip_idx)
                        .map(|(_, &n)| n)
                        .collect();
                }
                let mut tvector2 = vector.clone();
                let mut tvector1 = vector.clone();
                tvector2.remove(i + 1);
                tvector1.remove(i);
                if is_safe(&tvector1) == None {
                    total_safe.1 += 1;
                }
                if is_safe(&tvector2) == None {
                    total_safe.1 += 1;
                }
            } else {
                total_safe.0 += 1;
                total_safe.1 += 1;
            }
        }
        part1t.push(now.elapsed().as_nanos());
        part2t.push(now.elapsed().as_nanos());
    }

    // I DON'T KNOW WHY THIS DOESN'T GIVE THE CORRECT PART 2 ANSWER I SWEAR IT USED TO WORK BEFORE

    (total_safe, (read, cleanup, part1t, part2t))
}
