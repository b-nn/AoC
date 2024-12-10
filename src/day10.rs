use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn search(
    map: &Vec<&str>,
    trail: &(usize, usize, u32),
    count_duplicates: bool,
) -> Option<Vec<(usize, usize, u32)>> {
    let mut output = vec![];
    let position = (trail.0 as i32, trail.1 as i32);
    let rotations = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for i in rotations {
        let slice = map[(position.1 + i.1) as usize].chars().collect::<Vec<_>>();
        let t = slice[(position.0 + i.0) as usize].to_digit(10);
        if t.is_none() {
            continue;
        }
        if t.unwrap() == trail.2 + 1 {
            output.push((
                (position.0 + i.0) as usize,
                (position.1 + i.1) as usize,
                trail.2 + 1,
            ));
        }
    }

    // println!("{:?}", output);

    if output.len() > 0 {
        if output[0].2 == 9 {
            return Some(output);
        }
    }
    let mut temp: Vec<(usize, usize, u32)> = vec![];
    for i in &output {
        if let Some(x) = &mut search(map, i, count_duplicates) {
            temp.append(x);
        } else {
            return None;
        }
    }
    if !count_duplicates {
        temp.sort();
        temp.dedup();
    }
    Some(temp)
}

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

fn part_1(part: &mut i64, trails: &Vec<(usize, usize, u32)>, content: &Vec<&str>) {
    for trail in trails {
        let trailheads = search(content, trail, false);
        if let Some(x) = trailheads {
            *part += x.len() as i64;
        }
    }
}

fn part_2(part: &mut i64, trails: &Vec<(usize, usize, u32)>, content: &Vec<&str>) -> u32 {
    let mut total = 0;
    for trail in trails {
        total += search_p2(content, &(trail.1, trail.0));
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
        let mut trails: Vec<(usize, usize, u32)> = vec![];
        for j in content.iter().enumerate() {
            for i in j.1.chars().enumerate() {
                if i.1 == '0' {
                    trails.push((i.0, j.0, 0));
                }
            }
        }
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        part1 = 0;
        part_1(&mut part1, &trails, &content);
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        part2 = 0;
        part_2(&mut part2, &trails, &content);
        part2t.push(now.elapsed().as_nanos());

        println!("{}", search_p2(&content, &(3, 2)));
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
