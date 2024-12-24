use crate::REPEAT;
use core::panic;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: i64 = 0;
    let mut part2: i64 = 0;

    for _i in 0..REPEAT {
        let mut registers: HashMap<&str, bool> = HashMap::new();
        let now = Instant::now();
        let content =
            fs::read_to_string("day24.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        let mut line = 0;
        for i in content.lines() {
            line += 1;
            if i.is_empty() {
                break;
            }
            let mut s = i.split(": ");
            let mut reg = s.next().unwrap();
            let mut value = match s.next().unwrap() {
                "0" => false,
                "1" => true,
                _ => panic!("huh"),
            };
            registers.insert(reg, value);
        }

        let mut to_loop = true;

        while to_loop {
            to_loop = false;
            for i in &content.lines().collect::<Vec<_>>()[line..] {
                let mut values = i.split(' ');
                let reg1 = values.next().unwrap();
                let operator = values.next().unwrap();
                let reg2 = values.next().unwrap();
                values.next();
                let reg3 = values.next().unwrap();
                if registers.contains_key(reg3) {
                    continue;
                }
                let reg1value = registers.get(reg1);
                let reg2value = registers.get(reg2);
                if let Some(v1) = reg1value {
                    if let Some(v2) = reg2value {
                        registers.insert(
                            reg3,
                            match operator {
                                "XOR" => v1 ^ v2,
                                "OR" => v1 | v2,
                                "AND" => v1 & v2,
                                _ => panic!("yeah"),
                            },
                        );
                        to_loop = true;
                    }
                }
            }
        }
        let mut output: u64 = 0;
        for i in registers {
            if i.1 {
                if i.0.chars().nth(0) == Some('z') {
                    let bit: i64 = i.0[1..].parse().unwrap();
                    println!("{}", bit);
                    output = output | (1 << bit);
                    println!("{:b}", output);
                }
            }
        }
        println!("{}", output);

        read.push(now.elapsed().as_nanos());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
