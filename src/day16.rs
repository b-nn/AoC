use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn search(
    map: &Vec<Vec<u8>>,
    index: (usize, usize),
    vertices: &mut Vec<Vec<(i32, (i32, i32))>>,
    searched: &mut Vec<(usize, usize)>,
    to_search: &mut Vec<(i32, (usize, usize))>,
) {
    searched.push(index);
    if map[index.0][index.1 + 1] != b'#' {
        let value = vertices[index.0][index.1].0
            + 1
            + if vertices[index.0][index.1].1 != (0, 1) {
                1000
            } else {
                0
            };
        if vertices[index.0][index.1 + 1].0 > value {
            vertices[index.0][index.1 + 1] = (value, (0, 1));
        }
        if !searched.contains(&(index.0, index.1 + 1)) {
            to_search.push((value, (index.0, index.1 + 1)));
        }
    }
    if map[index.0][index.1 - 1] != b'#' {
        let value = vertices[index.0][index.1].0
            + 1
            + if vertices[index.0][index.1].1 != (0, -1) {
                1000
            } else {
                0
            };
        if vertices[index.0][index.1 - 1].0 > value {
            vertices[index.0][index.1 - 1] = (value, (0, -1));
        }
        if !searched.contains(&(index.0, index.1 - 1)) {
            to_search.push((value, (index.0, index.1 - 1)));
        }
    }
    if map[index.0 + 1][index.1] != b'#' {
        let value = vertices[index.0][index.1].0
            + 1
            + if vertices[index.0][index.1].1 != (1, 0) {
                1000
            } else {
                0
            };
        if vertices[index.0 + 1][index.1].0 > value {
            vertices[index.0 + 1][index.1] = (value, (1, 0));
        }
        if !searched.contains(&(index.0 + 1, index.1)) {
            to_search.push((value, (index.0 + 1, index.1)));
        }
    }
    if map[index.0 - 1][index.1] != b'#' {
        let value = vertices[index.0][index.1].0
            + 1
            + if vertices[index.0][index.1].1 != (-1, 0) {
                1000
            } else {
                0
            };
        if vertices[index.0 - 1][index.1].0 > value {
            vertices[index.0 - 1][index.1] = (value, (-1, 0));
        }
        if !searched.contains(&(index.0 - 1, index.1)) {
            to_search.push((value, (index.0 - 1, index.1)));
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
            fs::read_to_string("day16.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let mut origin = (0, 0);
        let mut end = (0, 0);
        let map = content
            .lines()
            .enumerate()
            .map(|(i, x)| {
                x.bytes()
                    .enumerate()
                    .map(|(j, y)| match y {
                        b'E' => {
                            end = (i, j);
                            b'.'
                        }
                        b'S' => {
                            origin = (i, j);
                            b'.'
                        }
                        _ => y,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut vertices: Vec<Vec<(i32, (i32, i32))>> = vec![];
        for row in 0..map.len() {
            vertices.push(vec![]);
            for column in 0..map[row].len() {
                if (row, column) == origin {
                    vertices[row].push((0, (0, 1)));
                } else {
                    vertices[row].push((9999999, (0, 0)));
                }
            }
        }

        let mut searching = vec![(0, (origin))];
        let mut searched = vec![];
        while !searching.is_empty() {
            println!("{:?}", searching);
            search(
                &map,
                searching[0].1,
                &mut vertices,
                &mut searched,
                &mut searching,
            );
            searching.remove(0);
            searching.sort();
            println!("{:?}", searching);
        }

        for row in &vertices {
            for i in row {
                print!("{:7} ", i.0);
            }
            println!("");
        }

        println!("{:?}", origin);
        println!("{:?}", vertices[end.0][end.1]);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
