use crate::REPEAT;
use core::panic;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn search(
    map: &mut Vec<Vec<i32>>,
    p: (usize, usize),
    searched: &mut Vec<(usize, usize)>,
    value: i32,
) {
    if searched.contains(&p) {
        return;
    }
    map[p.0][p.1] = value;
    searched.push(p);
    let dir: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for i in dir {
        if map[p.0 + i.0 as usize][p.1 + i.1 as usize] == 0 {
            search(
                map,
                (p.0 + i.0 as usize, p.1 + i.1 as usize),
                searched,
                value + 1,
            );
        }
    }
}

fn differences_p2(map: &Vec<Vec<i32>>, p: (usize, usize), dir: &Vec<(i32, i32, i32)>) -> i32 {
    let mut total = 0;
    for i in dir {
        if p.0 as i32 + i.0 < 0 || p.0 as i32 + i.0 >= map.len() as i32 {
            continue;
        }
        if p.1 as i32 + i.1 < 0 || p.1 as i32 + i.1 >= map[0].len() as i32 {
            continue;
        }
        if map[p.0 + i.0 as usize][p.1 + i.1 as usize] == 10009 {
            continue;
        }
        if map[p.0][p.1] - map[p.0 + i.0 as usize][p.1 + i.1 as usize] - i.2 >= 100 {
            total += 1;
        }
    }
    total
}

fn differences(map: &Vec<Vec<i32>>, p: (usize, usize)) -> i32 {
    let dir: [(i32, i32); 4] = [(0, 2), (0, -2), (2, 0), (-2, 0)];
    let mut total = 0;
    for i in dir {
        if map[p.0][p.1] - map[p.0 + i.0 as usize][p.1 + i.1 as usize] > 100 {
            total += 1;
        }
    }
    total
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: i32 = 0;
    let mut part2: i32 = 0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day20.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut tiles = vec![];
        let mut map = content
            .lines()
            .enumerate()
            .map(|(x, i)| {
                i.chars()
                    .enumerate()
                    .map(|(y, j)| match j {
                        '#' => 10009,
                        '.' => {
                            tiles.push((x, y));
                            0
                        }
                        'S' => {
                            start = (x, y);
                            tiles.push((x, y));
                            0
                        }
                        'E' => {
                            end = (x, y);
                            tiles.push((x, y));
                            0
                        }
                        _ => panic!("bad input"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        search(&mut map, end, &mut vec![], 0);
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        part1 = 0;
        for i in &tiles {
            part1 += differences(&map, *i);
        }
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();

        let mut t = vec![];
        for r in -20..=20_i32 {
            for c in -20..=20_i32 {
                if r.abs() + c.abs() <= 20 && !(r == 0 && c == 0) {
                    t.push((r, c, r.abs() + c.abs()));
                }
            }
        }

        part2 = 0;
        for i in &tiles {
            part2 += differences_p2(&map, *i, &t);
        }
        part2t.push(now.elapsed().as_nanos());
    }
    if part1 != 1450 {
        println!("PART 1 ISN'T RIGHT");
    }
    if part2 != 1015247 {
        println!("pART 2 ISn'T RIGHT");
    }

    (
        (part1 as i64, part2 as i64),
        (read, cleanup, part1t, part2t),
    )
}
