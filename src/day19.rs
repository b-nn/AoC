use crate::REPEAT;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn check<'a>(
    available: &'a Vec<&str>,
    goal: &'a str,
    depth: i32,
    map: &mut HashMap<&'a str, i64>,
) -> i64 {
    if goal == "" {
        return 1;
    }
    let mut total = 0;
    for pattern in available {
        if pattern.len() > goal.len() {
            continue;
        }
        if goal[goal.len() - pattern.len()..] == **pattern {
            if let Some(x) = map.get(&goal[..goal.len() - pattern.len()]) {
                total += x;
            } else {
                let temp = check(
                    available,
                    &goal[..goal.len() - pattern.len()],
                    depth + 1,
                    map,
                );

                total += temp;
                map.insert(&goal[..goal.len() - pattern.len()], temp);
            }
        }
    }
    total
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
            fs::read_to_string("day19.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let content = content.lines().collect::<Vec<_>>();
        let available = content[0].split(", ").collect::<Vec<_>>();
        let desired = &content[2..];
        println!("{:?} {:?}", available, desired);
        let mut total = 0;
        let now = Instant::now();
        for i in desired {
            total += check(&available, i, 0, &mut HashMap::new());
            println!("{}", i);
        }
        println!("{}", now.elapsed().as_micros());
        println!("total: {}", total);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
