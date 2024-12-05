use std::fs;
use std::time::Instant;

fn update(i: &mut Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    let mut okay = true;
    for j in rules {
        let pos_1 = match i.iter().position(|x| x == &j[0]) {
            Some(x) => x,
            None => continue,
        };
        let pos_2 = match i.iter().position(|x| x == &j[1]) {
            Some(x) => x,
            None => continue,
        };
        if pos_1 > pos_2 {
            okay = false;
            // continue;
            i.swap(pos_1, pos_2);
        }
    }
    okay
}

pub fn run() -> (i32, i32) {
    let content = fs::read_to_string("day5_p1.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
    let rules = fs::read_to_string("day5_p2.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");

    let rules = rules
        .lines()
        .map(|x| {
            x.split('|')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut content = content
        .lines()
        .map(|x| {
            x.split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut result = (0, 0);

    let now = Instant::now();
    for i in &mut content {
        let mut okay = true;
        let mut count = 0;

        while !update(i, &rules) {
            okay = false;
            count += 1;
        }

        if !okay {
            result.0 += i[i.len() / 2];
        } else {
            result.1 += i[i.len() / 2];
        }
        if count > 3 {
            println!("{:?}", i);
        }
    }

    println!("{}", now.elapsed().as_micros());

    result
}
