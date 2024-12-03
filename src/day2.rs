use std::collections::HashMap;
use std::fs;
use std::time::Instant;
use std::vec;

use crate::REPEAT;

fn is_safe(vector: &Vec<i32>) -> Option<usize> {
    let sign = (vector[0] - vector[1]).signum();
    let mut index = 0;
    let mut safe = true;
    for j in 0..vector.len() - 1 {
        let diff = vector[j] - vector[j + 1];
        if diff == 0 || diff.abs() > 3 || diff.signum() != sign {
            index = j;
            safe = false;
            break;
        }
    }
    if safe {
        None
    } else {
        Some(index)
    }
}

pub fn run() -> (i32, i32) {
    let contents = fs::read_to_string("day2.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");

    let contents_ver = contents
        .lines()
        .map(|x| {
            x.split(" ")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    if let Some(x) = is_safe(&vec![16, 18, 21, 22]) {
        println!("{} ih", x);
    }
    if is_safe(&vec![16, 18, 21, 22]) == None {
        println!("b");
    }

    let mut total_safe = 0;
    for (index, vector) in contents_ver.iter().enumerate() {
        if let Some(i) = is_safe(&vector) {
            let mut one = 0;
            let mut two = 0;
            for skip_idx in 0..vector.len() {
                let subset: Vec<i32> = vector
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != skip_idx)
                    .map(|(_, &n)| n)
                    .collect();
                if is_safe(&subset) == None {
                    one += 1;
                }
            }
            let mut tvector2 = vector.clone();
            let mut tvector1 = vector.clone();
            tvector2.remove(i + 1);
            tvector1.remove(i);
            if is_safe(&tvector1) == None {
                total_safe += 1;
                two = 1;
            }
            if is_safe(&tvector2) == None {
                total_safe += 1;
                two = 1;
            }
            if one > two {
                println!("{:?} {:?} {:?} {} {}", vector, tvector1, tvector2, one, two);
            }
            if two > one {
                println!("{:?} WHAT THE FUCKKKKKK", vector);
            }
        } else {
            total_safe += 1;
        }
    }
    (total_safe, 0)
}
