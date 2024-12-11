use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn search_p2(map: &Vec<&str>, trail: &(usize, usize)) -> u32 {
    let mut count = 0;
    if map[trail.0].as_bytes()[trail.1] == b'.' {
        return 0;
    }
    if map[trail.0].as_bytes()[trail.1] == b'9' {
        return 1;
    } else {
        if map[trail.0 + 1].as_bytes()[trail.1] - 1 == map[trail.0].as_bytes()[trail.1] {
            count += search_p2(map, &(trail.0 + 1, trail.1));
        }
        if map[trail.0 - 1].as_bytes()[trail.1] - 1 == map[trail.0].as_bytes()[trail.1] {
            count += search_p2(map, &(trail.0 - 1, trail.1));
        }
        if map[trail.0].as_bytes()[trail.1 + 1] - 1 == map[trail.0].as_bytes()[trail.1] {
            count += search_p2(map, &(trail.0, trail.1 + 1));
        }
        if map[trail.0].as_bytes()[trail.1 - 1] - 1 == map[trail.0].as_bytes()[trail.1] {
            count += search_p2(map, &(trail.0, trail.1 - 1));
        }
    }

    count
}

fn search_p1(map: &Vec<&str>, trail: &(usize, usize)) -> Vec<usize> {
    let mut output = vec![];
    if map[trail.0].as_bytes()[trail.1] == b'9' {
        output = vec![trail.0 * map[2].len() + trail.1];
    } else {
        if map[trail.0 + 1].as_bytes()[trail.1] - 1 == map[trail.0].as_bytes()[trail.1] {
            let mut t = search_p1(map, &(trail.0 + 1, trail.1));
            output.append(&mut t);
        }
        if map[trail.0 - 1].as_bytes()[trail.1] - 1 == map[trail.0].as_bytes()[trail.1] {
            let mut t = search_p1(map, &(trail.0 - 1, trail.1));
            output.append(&mut t);
        }
        if map[trail.0].as_bytes()[trail.1 + 1] - 1 == map[trail.0].as_bytes()[trail.1] {
            let mut t = search_p1(map, &(trail.0, trail.1 + 1));
            output.append(&mut t);
        }
        if map[trail.0].as_bytes()[trail.1 - 1] - 1 == map[trail.0].as_bytes()[trail.1] {
            let mut t = search_p1(map, &(trail.0, trail.1 - 1));
            output.append(&mut t);
        }
    }

    output
}

fn part_1(trails: &Vec<(usize, usize)>, content: &Vec<&str>) -> usize {
    let mut total = 0;
    for trail in trails {
        let mut trailheads = search_p1(content, trail);
        trailheads.sort();
        trailheads.dedup();
        total += trailheads.len();
    }
    total
}

fn part_2(trails: &Vec<(usize, usize)>, content: &Vec<&str>) -> u32 {
    let mut total = 0;
    for trail in trails {
        total += search_p2(content, &trail);
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
            fs::read_to_string("day10.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let content = content.lines().collect::<Vec<_>>();
        let mut trails: Vec<(usize, usize)> = vec![];
        for j in content.iter().enumerate() {
            for i in j.1.chars().enumerate() {
                if i.1 == '0' {
                    trails.push((j.0, i.0));
                }
            }
        }
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        part1 = part_1(&trails, &content) as i64;
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        part2 = part_2(&trails, &content) as i64;
        part2t.push(now.elapsed().as_nanos());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
