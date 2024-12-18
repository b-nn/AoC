use crate::REPEAT;
use core::panic;
use std::fs;
use std::time::Instant;

fn test(value: u64) -> u64 {
    let t = value % 8;
    (((t ^ 1) ^ (value >> (t ^ 1))) ^ 6) % 8
}

fn get_input(goal: &[u64], start: u64) -> (Option<u64>, u32) {
    let mut temp = start << 3;
    for i in 0..8 {
        if test(temp + i) == goal[0] {
            if goal.len() == 1 {
                return (Some(temp + i), 0);
            }
            let output = get_input(&goal[1..], temp + i);
            if let Some(x) = output.0 {
                if test(x >> (output.1 + 1) * 3) == goal[0] {
                    return (Some(x), output.1 + 1);
                }
            }
        }
    }
    (None, 0)
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: i64 = 0;
    let mut part2: u64 = 0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day17.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let content = content.lines().collect::<Vec<_>>();
        let mut a = content[0]
            .replace("Register A: ", "")
            .parse::<usize>()
            .unwrap();
        let mut b = content[1]
            .replace("Register B: ", "")
            .parse::<usize>()
            .unwrap();
        let mut c = content[2]
            .replace("Register C: ", "")
            .parse::<usize>()
            .unwrap();
        let t = content[4].replace("Program: ", "");
        let instructions = t
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        cleanup.push(now.elapsed().as_nanos());

        fn combo<'a>(value: usize, a: &usize, b: &usize, c: &usize) -> usize {
            match value {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => *a,
                5 => *b,
                6 => *c,
                _ => panic!("7 IS INVALID"),
            }
        }

        let now = Instant::now();
        let mut desired = [2, 4, 1, 1, 7, 5, 0, 3, 4, 3, 1, 6, 5, 5, 3, 0];
        desired.reverse();
        part2 = get_input(&desired, 0b0).0.unwrap();
        part2t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut pointer = 0;
        while pointer < instructions.len() {
            match instructions[pointer] {
                0 => a = a / (2usize.pow(combo(instructions[pointer + 1], &a, &b, &c) as u32)),
                1 => b ^= instructions[pointer + 1],
                2 => b = combo(instructions[pointer + 1], &a, &b, &c) % 8,
                3 => {
                    if a != 0 {
                        pointer = instructions[pointer + 1] - 2;
                    }
                }
                4 => b = b ^ c,
                5 => print!("{},", combo(instructions[pointer + 1], &a, &b, &c) % 8),
                6 => b = a / (2usize.pow(combo(instructions[pointer + 1], &a, &b, &c) as u32)),
                7 => c = a / (2usize.pow(combo(instructions[pointer + 1], &a, &b, &c) as u32)),
                _ => panic!("INVALID OPTCODE"),
            }
            pointer += 2;
        }
        println!("");
        part1t.push(now.elapsed().as_nanos());
    }

    ((part1, part2 as i64), (read, cleanup, part1t, part2t))
}
