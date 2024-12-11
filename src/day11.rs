use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn update(input: &Vec<String>) -> Vec<String> {
    let mut output = vec![];
    for i in input {
        if i == &"0" {
            output.push("1".to_string());
            continue;
        }
        if i.len() % 2 == 0 {
            output.push(i[..i.len() / 2].parse::<i128>().unwrap().to_string());
            output.push(i[i.len() / 2..].parse::<i128>().unwrap().to_string());
            continue;
        }
        output.push((i.parse::<i128>().unwrap() * 2024).to_string());
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
        let now = Instant::now();
        let mut content =
            fs::read_to_string("day11.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        content.pop();
        let content = content
            .split(' ')
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let mut stones = content;
        for _i in 0..75 {
            stones = update(&stones);
            println!("{:?}", stones);
            println!("{}", _i);
        }
        println!("{}", stones.len());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
