use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn check<'a>(available: &'a Vec<&str>, goal: &str, depth: i32) -> Option<Vec<&'a str>> {
    let mut output = vec![];
    if goal == "" {
        return Some(output);
    }
    for pattern in available {
        if pattern.len() > goal.len() {
            continue;
        }
        if goal[goal.len() - pattern.len()..] == **pattern {
            let mut t = check(available, &goal[..goal.len() - pattern.len()], depth + 1);
            if let Some(x) = &mut t {
                output.append(x);
                output.push(pattern);
                break;
            }
        }
    }
    if output.len() == 0 {
        None
    } else {
        Some(output)
    }
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
        for i in desired {
            if let Some(x) = check(&available, i, 0) {
                total += 1;
                println!("{:?} {}", x, i);
            }
        }
        println!("{}", total);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
