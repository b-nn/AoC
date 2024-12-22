use itertools::Itertools;

use crate::REPEAT;
use std::collections::{hash_set, HashMap, HashSet};
use std::fs;
use std::time::Instant;

fn cycle(mut input: &mut i64) {
    *input = ((*input * 64) ^ *input) % 16777216;
    *input = ((*input / 32) ^ *input) % 16777216;
    *input = ((*input << 11) ^ *input) % 16777216;
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
        let mut deltas: Vec<Vec<i64>> = vec![];
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        for i in 0..pseudo.len() {
            deltas.push(vec![]);
            let mut hash = HashSet::new();
            for j in 0..2000 {
                let last = pseudo[i];
                cycle(&mut pseudo[i]);
                deltas[i].push(pseudo[i] % 10 - last % 10);
                if j >= 3 {
                    let key = (
                        deltas[i][j - 3],
                        deltas[i][j - 2],
                        deltas[i][j - 1],
                        deltas[i][j - 0],
                    );
                    if hash.get(&key).is_none() {
                        *hash_total.entry(key).or_insert(0) += pseudo[i] % 10;
                        hash.insert(key);
                    }
                }
            }
        }
        for i in hash_total {
            if part2 < i.1 {
                part2 = i.1;
            }
        }
        part1t.push(0);
        part2t.push(now.elapsed().as_nanos());

        part1 = pseudo.iter().sum::<i64>();
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
