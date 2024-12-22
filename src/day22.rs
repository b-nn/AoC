use itertools::Itertools;

use crate::REPEAT;
use std::collections::{hash_set, HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn cycle(mut input: &mut i64) {
    *input = ((*input << 6) ^ *input) & 0xFFFFFF;
    *input = ((*input >> 5) ^ *input) & 0xFFFFFF;
    *input = ((*input << 11) ^ *input) & 0xFFFFFF;
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
        let content =
            fs::read_to_string("day22.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut pseudo = content
            .lines()
            .map(|x| x.parse::<i64>().expect("unable to parse"))
            .collect::<Vec<_>>();
        let mut hash_total = HashMap::new();
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut hash = HashSet::new();
        let mut v1 = 0;
        let mut v2 = 0;
        let mut v3 = 0;
        let mut v4 = 0;
        for mut i in pseudo {
            hash.drain();
            for j in 0..2000 {
                let last = i;
                cycle(&mut i);
                v1 = v2;
                v2 = v3;
                v3 = v4;
                v4 = i % 10 - last % 10 & 0b11111;
                if j >= 3 {
                    let key = (v1 | v2 << 5 | v3 << 10 | v4 << 15);
                    if hash.get(&key).is_none() {
                        *hash_total.entry(key).or_insert(0) += i % 10;
                        hash.insert(key);
                    }
                }
            }
            part1 += i;
        }
        for i in hash_total {
            if part2 < i.1 {
                part2 = i.1;
            }
        }
        part1t.push(0);
        part2t.push(now.elapsed().as_nanos());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
