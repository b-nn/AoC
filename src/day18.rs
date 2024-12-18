use crate::REPEAT;
use std::fs;
use std::time::Instant;

const SIZE: usize = 71;

fn search(
    map: &mut [[u32; SIZE + 2]; SIZE + 2],
    p: (usize, usize),
    to_search: &mut Vec<(u32, (usize, usize))>,
    searched: &Vec<(usize, usize)>,
) {
    for row in 0..map.len() {
        for item in 0..map.len() {
            if (row, item) == p {
                print!("@");
                continue;
            }
            match map[row][item] {
                0 => print!(" "),
                10000 => print!("#"),
                _ => print!("{}", map[row][item] % 10),
            }
        }
        println!();
    }
    println!("{:?}", p);
    println!();
    let curr = map[p.0][p.1];
    if map[p.0][p.1 + 1] == 0 {
        if !searched.contains(&(p.0, p.1 + 1)) {
            map[p.0][p.1 + 1] = curr + 1;
            to_search.push((curr + 1, (p.0, p.1 + 1)));
        }
    }
    if map[p.0][p.1 - 1] == 0 {
        if !searched.contains(&(p.0, p.1 - 1)) {
            map[p.0][p.1 - 1] = curr + 1;
            to_search.push((curr + 1, (p.0, p.1 - 1)));
        }
    }
    if map[p.0 + 1][p.1] == 0 {
        if !searched.contains(&(p.0 + 1, p.1)) {
            map[p.0 + 1][p.1] = curr + 1;
            to_search.push((curr + 1, (p.0 + 1, p.1)));
        }
    }
    if map[p.0 - 1][p.1] == 0 {
        if !searched.contains(&(p.0 - 1, p.1)) {
            map[p.0 - 1][p.1] = curr + 1;
            to_search.push((curr + 1, (p.0 - 1, p.1)));
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
            fs::read_to_string("day18.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");

        let values = content
            .lines()
            .map(|x| {
                x.split(',')
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut grid = [[0; SIZE + 2]; SIZE + 2];
        for row in 0..SIZE + 2 {
            for column in 0..SIZE + 2 {
                if row == 0 || row == SIZE + 1 {
                    grid[row][column] = 10000;
                }
                if column == 0 || column == SIZE + 1 {
                    grid[row][column] = 10000;
                }
            }
        }
        for i in &values[0..1024] {
            grid[i[1] + 1][i[0] + 1] = 10000;
        }
        let mut to_search = vec![(0, (1, 1))];
        let mut searched = vec![];
        while !to_search.is_empty() {
            search(&mut grid, to_search[0].1, &mut to_search, &searched);
            searched.push(to_search[0].1);
            to_search.remove(0);
        }
        println!("{} oh", grid[71][71]);
        read.push(now.elapsed().as_nanos());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
