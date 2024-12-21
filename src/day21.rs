use itertools::Itertools;

use crate::REPEAT;
use core::panic;
use std::fs;
use std::time::Instant;

fn search(mut input: &mut Vec<u8>, depth: i32) -> Vec<u8> {
    let mut output: Vec<u8> = vec![];
    if depth == 0 {
        return input.to_vec();
    }
    let mut t = vec![b'A'];
    t.append(&mut input);
    let input = t;
    let mut output = vec![];
    let mut index = 0;
    for i in input.windows(2) {
        let mut min = vec![b'.'; 10000];
        let paths = if depth == 1 {
            get_paths((i[0], i[1]))
        } else {
            get_paths((i[0], i[1]))
        };
        for mut j in paths {
            j.push(b'A');
            let temp = search(&mut j, depth - 1);
            if temp.len() < min.len() {
                min = temp.to_vec();
            }
        }
        output.append(&mut min);
        index += 1;
    }
    output.clone()
}

fn get_paths<'a>(input: (u8, u8)) -> Vec<Vec<u8>> {
    let first: (i32, i32) = match input.0 {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => panic!("invalid input"),
    };
    let second: (i32, i32) = match input.1 {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => panic!("invalid input"),
    };

    let mut output = vec![];
    for _i in 0..(first.1 - second.1).abs() {
        if (first.1 - second.1).signum() > 0 {
            output.push(b'<');
        } else {
            output.push(b'>');
        }
    }
    for _i in 0..(first.0 - second.0).abs() {
        if (first.0 - second.0).signum() > 0 {
            output.push(b'^');
        } else {
            output.push(b'v');
        }
    }
    let t = output.len();
    let out = output.into_iter().permutations(t).collect_vec();
    out
}

fn control(input: (u8, u8)) -> Vec<u8> {
    let first: (i32, i32) = match input.0 {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => panic!("invalid input"),
    };
    let second: (i32, i32) = match input.1 {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => panic!("invalid input"),
    };
    let mut output = vec![];
    for _i in 0..(first.1 - second.1).abs() {
        if (first.1 - second.1).signum() > 0 {
            output.push(b'<');
        } else {
            output.push(b'>');
        }
    }
    for _i in 0..(first.0 - second.0).abs() {
        if (first.0 - second.0).signum() > 0 {
            output.push(b'^');
        } else {
            output.push(b'v');
        }
    }
    output.push(b'A');
    output
}

fn keypad(input: (u8, u8)) -> Vec<u8> {
    let first: (i32, i32) = match input.0 {
        b'7' => (0, 0),
        b'8' => (0, 1),
        b'9' => (0, 2),
        b'4' => (1, 0),
        b'5' => (1, 1),
        b'6' => (1, 2),
        b'1' => (2, 0),
        b'2' => (2, 1),
        b'3' => (2, 2),
        b'0' => (3, 1),
        b'A' => (3, 2),
        _ => panic!("invalid input"),
    };
    let second: (i32, i32) = match input.1 {
        b'7' => (0, 0),
        b'8' => (0, 1),
        b'9' => (0, 2),
        b'4' => (1, 0),
        b'5' => (1, 1),
        b'6' => (1, 2),
        b'1' => (2, 0),
        b'2' => (2, 1),
        b'3' => (2, 2),
        b'0' => (3, 1),
        b'A' => (3, 2),
        _ => panic!("invalid input"),
    };
    let mut output = vec![];
    for _i in 0..(first.1 - second.1).abs() {
        if (first.1 - second.1).signum() > 0 {
            output.push(b'<');
        } else {
            output.push(b'>');
        }
    }
    for _i in 0..(first.0 - second.0).abs() {
        if (first.0 - second.0).signum() > 0 {
            output.push(b'^');
        } else {
            output.push(b'v');
        }
    }
    output.push(b'A');
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
        let now = Instant::now();
        let content =
            fs::read_to_string("day21.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());
        let mut total = 0;
        for i in content.lines() {
            let input = i.as_bytes();
        }
        let mut test = vec![
            b'<', b'A', b'^', b'A', b'>', b'^', b'^', b'A', b'v', b'v', b'v', b'A',
        ];
        let t = search(&mut test, 2);
        for i in &t {
            print!("{}", *i as char);
        }
        println!();
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
