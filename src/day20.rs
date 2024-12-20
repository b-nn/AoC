use crate::REPEAT;
use core::panic;
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

fn differences(
    map: &Vec<Vec<i32>>,
    p: (usize, usize),
    to_return: &mut Vec<(usize, usize)>,
    searched: &mut Vec<(usize, usize)>,
) {
    if searched.contains(&p) {
        return;
    }
    searched.push(p);
    println!("{:?}", p);
    let dir: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (0, 2),
        (0, -2),
        (2, 0),
        (-2, 0),
    ];
    for i in dir {
        if map[p.0][p.1] - map[p.0 + i.0 as usize][p.1 + i.1 as usize] > 100 {
            to_return.push(p);
        }
        if map[p.0 + i.0 as usize][p.1 + i.1 as usize] < map[p.0][p.1] {
            differences(
                map,
                (p.0 + i.0 as usize, p.1 + i.1 as usize),
                to_return,
                searched,
            );
        }
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
            fs::read_to_string("day20.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        let mut start = (0, 0);
        let mut end = (0, 0);
        let mut map = content
            .lines()
            .enumerate()
            .map(|(x, i)| {
                i.chars()
                    .enumerate()
                    .map(|(y, j)| match j {
                        '#' => 10009,
                        '.' => 0,
                        'S' => {
                            start = (x, y);
                            0
                        }
                        'E' => {
                            end = (x, y);
                            0
                        }
                        _ => panic!("bad input"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        read.push(now.elapsed().as_nanos());

        for row in &map {
            for i in row {
                print!(
                    "{}",
                    match i {
                        10009 => "#".to_owned(),
                        _ => (i % 10).to_string(),
                    }
                );
            }
            println!();
        }

        println!("{:?}", search(&mut map, end, &mut vec![], 0));

        let mut diff = vec![];
        differences(&map, start, &mut diff, &mut vec![]);

        for row in 0..map.len() {
            for column in 0..map[row].len() {
                print!(
                    "{}",
                    if diff.contains(&(row, column)) {
                        "@".to_owned()
                    } else {
                        match map[row][column] {
                            10009 => "#".to_owned(),
                            _ => (map[row][column] % 10).to_string(),
                        }
                    }
                );
            }
            println!();
        }
        println!("{:?}", diff);
        println!("{}", diff.len());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
