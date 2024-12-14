use crate::REPEAT;
use std::fs;
use std::io::Lines;
use std::time::Instant;

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

        let robots = content
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
        let content = content.lines().collect::<Vec<_>>();
        let height = 103;
        let width = 101;

        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;
        for i in &robots {
            let position = (
                (i.0 .0 + i.1 .0 * 100).rem_euclid(width),
                (i.0 .1 + i.1 .1 * 100).rem_euclid(height),
            );
            if position.0 == width / 2 {
                continue;
            }
            if position.1 == height / 2 {
                continue;
            }
            if position.0 < width / 2 {
                if position.1 < height / 2 {
                    q1 += 1;
                } else {
                    q4 += 1;
                }
            } else {
                if position.1 < height / 2 {
                    q2 += 1;
                } else {
                    q3 += 1;
                }
            }
        }
        part1 = q1 * q2 * q3 * q4;

        println!("{}", part1);
        println!("{}  {}", width, height);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
