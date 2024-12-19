use crate::REPEAT;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn u8toi32(input: &[u8]) -> i32 {
    let mut temp = 0;
    let mut index = 0;
    for i in input {
        index += 1;
        let x = match i {
            b'w' => 0,
            b'u' => 1,
            b'r' => 2,
            b'g' => 3,
            _ => 4,
        };
        temp += index * 6 * x;
    }
    temp
}

fn check_p1<'a>(available: &'a Vec<Vec<&[u8]>>, goal: &'a [u8]) -> i64 {
    if goal.is_empty() {
        return 1;
    }
    if goal.len() <= available.len() {
        if available[goal.len() - 1].contains(&goal) {
            return 1;
        }
    }
    for len in 0..8 {
        if len >= goal.len() {
            return 0;
        }
        if available[len].contains(&&goal[(goal.len() - len - 1)..]) {
            if check_p1(available, &goal[..(goal.len() - len - 1)]) == 1 {
                return 1;
            }
        }
    }
    0
}

fn check<'a>(
    available: &Vec<&[u8]>,
    goal: &'a [u8],
    r: bool,
    map: &mut HashMap<&'a [u8], i64>,
) -> i64 {
    if goal.is_empty() {
        return 1;
    }
    let mut total = 0;
    for pattern in available {
        if pattern.len() > goal.len() {
            continue;
        }
        if goal[goal.len() - pattern.len()..] == **pattern {
            let t = &goal[..goal.len() - pattern.len()];
            if r {
                if check(available, t, r, map) == 1 {
                    return 1;
                }
            } else {
                if let Some(x) = map.get(t) {
                    total += x;
                } else {
                    let temp = check(available, t, r, map);
                    total += temp;
                    map.insert(t, temp);
                }
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
        part1 = 0;
        part2 = 0;

        let now = Instant::now();
        let content =
            fs::read_to_string("day19.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let content = content.lines().collect::<Vec<_>>();
        let available = content[0]
            .split(", ")
            .map(|x| x.as_bytes())
            .collect::<Vec<_>>();
        let mut desired = content[2..]
            .iter()
            .map(|x| x.as_bytes())
            .collect::<Vec<_>>();
        desired.sort();
        let mut t = vec![vec![]; 8];
        for i in &available {
            println!("{} {:?}", i.len(), i);
            t[i.len() - 1].push(*i);
        }
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        for i in &desired {
            part1 += check_p1(&t, i);
        }
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut map = HashMap::new();
        for i in &desired {
            part2 += check(&available, i, false, &mut map);
        }
        part2t.push(now.elapsed().as_nanos());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
