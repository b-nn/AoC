use crate::REPEAT;
use std::fs;
use std::io::Lines;
use std::thread::sleep;
use std::time::{Duration, Instant};

fn step(robots: &mut Vec<((i32, i32), (i32, i32))>) {
    for j in robots {
        j.0 = (
            (j.0 .0 + j.1 .0).rem_euclid(WIDTH),
            (j.0 .1 + j.1 .1).rem_euclid(HEIGHT),
        );
    }
}

fn generate_list(robots: &Vec<((i32, i32), (i32, i32))>) -> [[i32; USIZE_WIDTH]; USIZE_HEIGHT] {
    let mut temp = [[0; USIZE_WIDTH]; USIZE_HEIGHT];

    for i in robots {
        temp[i.0 .1 as usize][i.0 .0 as usize] += 1;
    }

    temp
}

fn check_blocks(map: &[[i32; USIZE_WIDTH]; USIZE_HEIGHT]) -> bool {
    let rows = map.windows(13);
    for row in map {
        for index in 0..row.len() - 35 {
            let line = &row[index..index + 31];
            let mut is_ones = true;
            for i in line {
                if *i != 1 {
                    is_ones = false;
                    break;
                }
            }
            if is_ones {
                return true;
            }
        }
    }
    // for block in rows {
    //     let mut index = 1000;
    //     let mut found = false;
    //     for row in block {
    //         index = 0;
    //         for column in index..row.len() {
    //             if column >= USIZE_WIDTH - 10 {
    //                 break;
    //             }
    //             let t = &row[column..column + 9];
    //             let mut is_ones = true;
    //             for i in t {
    //                 if *i != 1 {
    //                     is_ones = false;
    //                 }
    //             }
    //             if is_ones {
    //                 index = column;
    //                 found = true;
    //                 break;
    //             }
    //         }
    //         if !found {
    //             break;
    //         }
    //     }
    //     if found {
    //         return true;
    //     }
    // }

    false
}

fn check(map: &[[i32; USIZE_WIDTH]; USIZE_HEIGHT]) -> (bool, bool) {
    let mut row_aligned = false;
    let mut column_aligned = false;
    for row in map {
        if row.iter().sum::<i32>() >= 30 {
            row_aligned = true;
            break;
        }
    }
    for i in 0..map[0].len() {
        let mut total = 0;
        for j in map {
            total += j[i];
        }
        if total >= 30 {
            column_aligned = true;
            break;
        }
    }

    (row_aligned, column_aligned)
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const USIZE_WIDTH: usize = 101;
const USIZE_HEIGHT: usize = 103;

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content = fs::read_to_string("day14.txt")
            .expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK")
            .replace("p=", "")
            .replace("v=", "");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut robots = content
            .lines()
            .map(|x| {
                let mut t = x.split(' ').map(|y| {
                    let mut t = y.split(',');
                    (
                        t.next().unwrap().parse::<i32>().unwrap(),
                        t.next().unwrap().parse::<i32>().unwrap(),
                    )
                });
                (t.next().unwrap(), t.next().unwrap())
            })
            .collect::<Vec<_>>();
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        for i in &robots {
            let position = (
                (i.0 .0 + i.1 .0 * 100).rem_euclid(WIDTH),
                (i.0 .1 + i.1 .1 * 100).rem_euclid(HEIGHT),
            );
            if position.0 == WIDTH / 2 {
                continue;
            }
            if position.1 == HEIGHT / 2 {
                continue;
            }
            if position.0 < WIDTH / 2 {
                if position.1 < HEIGHT / 2 {
                    q1 += 1;
                } else {
                    q4 += 1;
                }
            } else {
                if position.1 < HEIGHT / 2 {
                    q2 += 1;
                } else {
                    q3 += 1;
                }
            }
        }
        part1 = q1 * q2 * q3 * q4;
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut row = 0;
        let mut column = 0;
        for i in 0..101 {
            step(&mut robots);
            let r = generate_list(&robots);
            // let mut output = "".to_owned();
            // for row in &r {
            //     for column in row {
            //         if *column == 0 {
            //             output.push(' ');
            //         } else {
            //             output.push_str(&column.to_string());
            //         }
            //     }
            //     output.push_str("\n");
            // }
            // println!("{}", output);
            // println!("{}", i);

            let t = check(&r);
            if t.0 {
                row = i + 1;
            }
            if t.1 {
                column = i + 1;
            }
        }

        part2 = (row * 101 * 51 - column * 103 * 50) % (101 * 103);
        part2t.push(now.elapsed().as_nanos());

        // for i in 0..100000 {
        //     step(&mut robots);
        //     let r = generate_list(&robots);
        //     if check_blocks(&r) {
        //         part2 = i + 1;
        //         break;
        //     }
        // }
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
