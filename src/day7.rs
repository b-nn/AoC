use crate::REPEAT;
use core::panic;
use std::time::Instant;
use std::{fs, i64};

fn search(values: &[i64], goal: i64, concatenate: bool) -> bool {
    if values.len() == 0 {
        if goal == 0 {
            return true;
        }
        return false;
    }

    if concatenate {
        let t = match values[0] {
            ..1 => 1,
            ..10 => 10,
            ..100 => 100,
            ..1000 => 1000,
            _ => panic!("HELP!!!! WHAT THE FUCKKKKKK"),
        };
        if values[0] == goal % t {
            if goal / t == 0 {
                return true;
            }
            if search(&values[1..], goal / t, concatenate) {
                return true;
            }
        }
    }

    if goal % values[0] == 0 && search(&values[1..], goal / values[0], concatenate) {
        return true;
    }

    if search(&values[1..], goal - values[0], concatenate) {
        return true;
    }

    return false;
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];

    let mut total = (0, 0);
    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day7.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let content = content
            .lines()
            .map(|x| x.split(": ").collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let goals = content
            .iter()
            .map(|x| x[0].parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let values = content
            .iter()
            .map(|x| {
                x[1].split(' ')
                    .map(|y| y.parse::<i64>().unwrap())
                    .rev()
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        cleanup.push(now.elapsed().as_nanos());

        total = (0, 0);

        let now = Instant::now();
        for i in goals.iter().enumerate() {
            if search(&values[i.0], *i.1, false) {
                total.0 += i.1;
            }
        }
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        for i in goals.iter().enumerate() {
            if search(&values[i.0], *i.1, true) {
                total.1 += i.1;
            }
        }
        part2t.push(now.elapsed().as_nanos());
    }

    (total, (read, cleanup, part1t, part2t))
}
