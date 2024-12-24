use crate::REPEAT;
use core::panic;
use std::collections::HashMap;
use std::fmt::format;
use std::fs;
use std::time::Instant;

fn find(vector: &Vec<&str>, input: &str) -> Vec<usize> {
    let mut output = vec![];
    for i in 0..vector.len() {
        if vector[i] == input {
            output.push(i);
        }
    }
    output
}
fn find_t<'a>(
    vector: &'a Vec<(&str, &str, &str, &str)>,
    input: &str,
) -> Vec<(&'a str, &'a str, &'a str, &'a str)> {
    let mut output = vec![];
    for i in 0..vector.len() {
        if vector[i].0 == input || vector[i].2 == input {
            output.push(vector[i]);
        }
    }
    output
}

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
        let mut index = 0;

        let mut register1 = vec![];
        let mut register2 = vec![];
        let mut instructions = vec![];

        while to_loop {
            to_loop = false;
            index += 1;
            for i in &content.lines().collect::<Vec<_>>()[line..] {
                let mut values = i.split(' ');
                let reg1 = values.next().unwrap();
                let operator = values.next().unwrap();
                let reg2 = values.next().unwrap();
                values.next();
                let reg3 = values.next().unwrap();
                if index == 1 {
                    register1.push(reg1);
                    register2.push(reg2);
                    instructions.push((reg1, operator, reg2, reg3));
                }
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
                    output = output | (1 << bit);
                }
            }
        }
        println!("{}", output);

        let mut carry = "";
        for index in 0..=44 {
            let locations = find(&register1, &format!("x{:02}", index));
            let locations2 = find(&register2, &format!("x{:02}", index));
            if index == 0 {
                continue;
            } else {
                let mut t1 = "";
                let mut t2 = "";
                let mut t3 = "";
                let mut o1 = "";
                let mut c1 = "";
                for i in [locations, locations2].concat() {
                    match instructions[i].1 {
                        "XOR" => {
                            t1 = instructions[i].3;
                        }
                        "AND" => {
                            t2 = instructions[i].3;
                        }
                        _ => {
                            println!("odd 1 {} {index}", instructions[i].3);
                        }
                    }
                }
                let locations3 = find_t(&instructions, t1);
                for i in locations3 {
                    match i.1 {
                        "XOR" => {
                            o1 = i.3;
                            c1 = if t1 == i.0 { i.2 } else { i.0 }
                        }
                        "AND" => {
                            t3 = i.3;
                            c1 = if t1 == i.0 { i.2 } else { i.0 }
                        }
                        _ => {
                            println!("odd 2 {} {index}", t1);
                        }
                    }
                }
                if carry != c1 {
                    println!("odd 3 {} {} {index}", t1, c1);
                }
                if o1.chars().nth(0) != Some('z') {
                    println!("odd 4 {} {} {} {index}", o1, t1, c1)
                }
                let locations4 = find_t(&instructions, t3);
                for i in locations4 {
                    match i.1 {
                        "OR" => {
                            carry = i.3;
                        }
                        _ => {
                            println!("odd 5 {} {index}", t3);
                        }
                    }
                    if t2 != if t3 == i.0 { i.2 } else { i.0 } {
                        println!("odd 6 {} {} {index}", t2, t3);
                    }
                }
            }
        }
        println!("{:?}", register2);

        // println!();
        // for i in 0..instructions.len() {
        //     println!(
        //         "1: {} 2: {} instrc: {:?}",
        //         register1[i], register2[i], instructions[i]
        //     )
        // }

        read.push(now.elapsed().as_nanos());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
