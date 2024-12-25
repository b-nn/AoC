use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn convert(schematic: &Vec<&str>, key: &mut Vec<Vec<i32>>, lock: &mut Vec<Vec<i32>>) {
    println!("{:?}", schematic);
    let mut first = false;
    for index in 0..5 {
        let mut t = 0;
        for i in schematic {
            if i.chars().nth(index) == Some('#') {
                t += 1;
            }
        }
        if schematic[0] == "#####" {
            lock[index].push(t);
        } else {
            key[index].push(t);
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
            fs::read_to_string("day25.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());
        let mut index = 0;
        let mut schematic = vec![];
        let mut keys: Vec<Vec<i32>> = vec![vec![], vec![], vec![], vec![], vec![]];
        let mut locks: Vec<Vec<i32>> = vec![vec![], vec![], vec![], vec![], vec![]];
        for i in content.lines() {
            schematic.push(i);
            if i.is_empty() {
                convert(&schematic, &mut keys, &mut locks);
                schematic = vec![];
                index += 1;
            }
        }

        let mut pairs = 0;
        for lock_index in 0..locks[0].len() {
            for key_index in 0..keys[0].len() {
                if locks[0][lock_index] + keys[0][key_index] > 7 {
                    continue;
                }
                if locks[1][lock_index] + keys[1][key_index] > 7 {
                    continue;
                }
                if locks[2][lock_index] + keys[2][key_index] > 7 {
                    continue;
                }
                if locks[3][lock_index] + keys[3][key_index] > 7 {
                    continue;
                }
                if locks[4][lock_index] + keys[4][key_index] > 7 {
                    continue;
                }
                pairs += 1;
            }
        }
        println!("pairs {:?}", pairs);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
