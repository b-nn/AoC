use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn search(map: &Vec<&str>, trail: (usize, usize, u32)) -> Option<Vec<(usize, usize, u32)>> {
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
        if let Some(x) = &mut search(map, *i) {
            temp.append(x);
        } else {
            return None;
        }
    }
    temp.sort();
    temp.dedup();
    Some(temp)
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

        let content = content.lines().collect::<Vec<_>>();

        let mut trails: Vec<(usize, usize, u32)> = vec![];
        for j in content.iter().enumerate() {
            for i in j.1.chars().enumerate() {
                if i.1 == '0' {
                    trails.push((i.0, j.0, 0));
                }
            }
        }
        part1 = 0;
        for trail in trails {
            println!("{:?}", trail);
            println!("{:?}", search(&content, trail));
            let trailheads = search(&content, trail);
            if let Some(x) = trailheads {
                part1 += x.len() as i64;
            }
        }

        println!("{}", part1);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
