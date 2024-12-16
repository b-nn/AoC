use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn search_p2(vertices: &Vec<Vec<(i32, (i32, i32))>>, position: (usize, usize)) -> i32 {
    let mut total = 0;
    let v = vertices[position.0][position.1].0;
    println!("{:?} {}", position, v);
    if vertices[position.0][position.1 + 1].0 < v
        || (vertices[position.0][position.1 + 1].0 + 2 == vertices[position.0][position.1 - 1].0
            && vertices[position.0][position.1 + 1].1 == (0, -1)
            && v < 999999)
    {
        total += search_p2(vertices, (position.0, position.1 + 1));
    }
    if vertices[position.0][position.1 - 1].0 < v
        || (vertices[position.0][position.1 - 1].0 + 2 == vertices[position.0][position.1 + 1].0
            && vertices[position.0][position.1 - 1].1 == (0, 1)
            && v < 999999)
    {
        total += search_p2(vertices, (position.0, position.1 - 1));
    }
    if vertices[position.0 + 1][position.1].0 < v
        || (vertices[position.0 + 1][position.1].0 + 2 == vertices[position.0 - 1][position.1].0
            && vertices[position.0 + 1][position.1].1 == (-1, 0)
            && v < 999999)
    {
        total += search_p2(vertices, (position.0 + 1, position.1));
    }
    if vertices[position.0 - 1][position.1].0 < v
        || (vertices[position.0 - 1][position.1].0 + 2 == vertices[position.0 + 1][position.1].0
            && vertices[position.0 - 1][position.1].1 == (1, 0)
            && v < 999999)
    {
        total += search_p2(vertices, (position.0 - 1, position.1));
    }
    total + 1
}

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

        let now = Instant::now();
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
        let mut vertices_p2: Vec<Vec<(i32, (i32, i32))>> = vec![];
        for row in 0..map.len() {
            vertices.push(vec![]);
            vertices_p2.push(vec![]);
            for column in 0..map[row].len() {
                if (row, column) == origin {
                    vertices[row].push((0, (0, 1)));
                    vertices_p2[row].push((9999999, (0, 0)));
                } else if (row, column) == end {
                    vertices[row].push((9999999, (0, 0)));
                    vertices_p2[row].push((0, (1, 0)));
                } else {
                    vertices[row].push((9999999, (0, 0)));
                    vertices_p2[row].push((9999999, (0, 0)));
                }
            }
        }
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut searching = vec![(0, (origin))];
        let mut searched = vec![];
        while !searching.is_empty() {
            search(
                &map,
                searching[0].1,
                &mut vertices,
                &mut searched,
                &mut searching,
            );
            searching.remove(0);
            searching.sort();
        }
        part1 = vertices[end.0][end.1].0 as i64;
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut searching = vec![(0, (end))];
        let mut searched = vec![];
        while !searching.is_empty() {
            search(
                &map,
                searching[0].1,
                &mut vertices_p2,
                &mut searched,
                &mut searching,
            );
            searching.remove(0);
            searching.sort();
        }

        let mut seats = 0;
        for row in 0..vertices.len() {
            for i in 0..vertices[row].len() {
                if vertices[row][i].0 + vertices_p2[row][i].0 == vertices[end.0][end.1].0
                    || vertices[row][i].0 + vertices_p2[row][i].0 + 1000 == vertices[end.0][end.1].0
                {
                    seats += 1;
                }
            }
        }
        part2 = seats;
        part2t.push(now.elapsed().as_nanos());
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
