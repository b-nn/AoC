use std::fs;
use std::process::Output;
use std::time::Instant;
use std::vec;

use crate::REPEAT;

fn get_int(input: &Vec<char>, i: &usize) -> Option<(u32, usize)> {
    let mut output = 0;
    let mut index = i.clone();
    let mut success = false;
    while input[index].to_digit(10).is_some() {
        success = true;
        output *= 10;
        output += input[index].to_digit(10).unwrap();
        index += 1;
    }

    if success {
        Some((output, index))
    } else {
        None
    }
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];

    let mut result = (0, 0);

    for _i in 0..REPEAT {
        let now = Instant::now();
        result = (0, 0);
        let contents =
            fs::read_to_string("day3.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let input = contents.chars().collect::<Vec<_>>();
        cleanup.push(now.elapsed().as_nanos());

        let mut index = 0;
        let mut enabled = true;
        let now = Instant::now();
        for j in input.windows(7) {
            if j[..4] == ['d', 'o', '(', ')'] {
                enabled = true;
            }
            if j[..7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {
                enabled = false;
            }
            if j[..4] == ['m', 'u', 'l', '('] {
                let output_1 = get_int(&input, &(index + 4));
                if output_1.is_none() {
                    index += 1;
                    continue;
                }
                if input[output_1.unwrap().1] != ',' {
                    index += 1;
                    continue;
                }
                let output_2 = get_int(&input, &(output_1.unwrap().1 + 1));
                if output_2.is_none() {
                    index += 1;
                    continue;
                }
                if input[output_2.unwrap().1] != ')' {
                    index += 1;
                    continue;
                }
                if enabled {
                    result.1 += (output_1.unwrap().0 * output_2.unwrap().0) as i64;
                }
                result.0 += (output_1.unwrap().0 * output_2.unwrap().0) as i64;
            }
            index += 1;
        }
        part1t.push(now.elapsed().as_nanos());
        part2t.push(now.elapsed().as_nanos());
    }

    (result, (read, cleanup, part1t, part2t))
}
