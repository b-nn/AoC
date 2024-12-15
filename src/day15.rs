use crate::REPEAT;
use core::panic;
use std::fs;
use std::time::Instant;

fn check_empty_p1(
    map: &Vec<Vec<u8>>,
    direction: &(i32, i32),
    position: (i32, i32),
    depth: i32,
) -> (bool, Option<(i32, i32)>) {
    if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b'#' {
        return (false, None);
    } else if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b'O'
    {
        return check_empty_p1(
            map,
            direction,
            (position.0 + direction.0, position.1 + direction.1),
            depth + 1,
        );
    } else if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b'.'
    {
        if depth == 0 {
            return (true, None);
        } else {
            return (
                true,
                Some((position.0 + direction.0, position.1 + direction.1)),
            );
        }
    }
    (false, None)
}

fn check_empty_p2(
    map: &Vec<Vec<u8>>,
    direction: &(i32, i32),
    position: (i32, i32),
    depth: i32,
    boxes: &mut Vec<(i32, i32)>,
) -> bool {
    if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b'#' {
        return false;
    }
    if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b'[' {
        boxes.push((position.0 + direction.0, position.1 + direction.1));
        return check_empty_p2(
            map,
            direction,
            (position.0 + direction.0, position.1 + direction.1),
            depth + 1,
            boxes,
        ) & check_empty_p2(
            map,
            direction,
            (
                position.0 + direction.0,
                position.1 + direction.1 + direction.0.abs(),
            ),
            depth + 1,
            boxes,
        );
    }
    if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b']' {
        boxes.push((position.0 + direction.0, position.1 + direction.1 - 1));
        return check_empty_p2(
            map,
            direction,
            (position.0 + direction.0, position.1 + direction.1),
            depth + 1,
            boxes,
        ) & check_empty_p2(
            map,
            direction,
            (
                position.0 + direction.0,
                position.1 + direction.1 - direction.0.abs(),
            ),
            depth + 1,
            boxes,
        );
    }
    if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b'.' {
        return true;
    }
    panic!(
        "WHAT {:?} {}",
        position,
        map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize]
    );
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
            fs::read_to_string("day15_p1.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        let moves =
            fs::read_to_string("day15_p2.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let mut robot: (i32, i32);
        let mut robot_origin = (0, 0);
        let mut map_p1 = content
            .lines()
            .enumerate()
            .map(|(i, x)| {
                x.bytes()
                    .enumerate()
                    .map(|(j, y)| match y {
                        b'@' => {
                            robot_origin = (i as i32, j as i32);
                            b'.'
                        }
                        _ => y,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let temp = content
            .lines()
            .map(|x| {
                x.bytes()
                    .map(|y| match y {
                        b'@' => "..",
                        b'#' => "##",
                        b'O' => "[]",
                        b'.' => "..",
                        _ => panic!("invalid input"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut map_p2: Vec<Vec<u8>> = vec![];
        for row in map_p1.iter().enumerate() {
            map_p2.push(vec![]);
            for item in row.1 {
                match item {
                    b'O' => {
                        map_p2[row.0].push(b'[');
                        map_p2[row.0].push(b']');
                    }
                    b'#' => {
                        map_p2[row.0].push(b'#');
                        map_p2[row.0].push(b'#');
                    }
                    b'.' => {
                        map_p2[row.0].push(b'.');
                        map_p2[row.0].push(b'.');
                    }
                    _ => {
                        panic!("bad input");
                    }
                }
            }
        }
        println!("{:?}", moves);
        let moves = moves
            .trim()
            .replace("\n", "")
            .chars()
            .map(|x| match x {
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                'v' => (1, 0),
                x => panic!("HUHHHHHH {}", x),
            })
            .collect::<Vec<_>>();

        robot = robot_origin;
        println!("{:?}", robot);
        for direction in &moves {
            let check = check_empty_p1(&map_p1, direction, robot, 0);
            if check.0 {
                if let Some(x) = check.1 {
                    map_p1[x.0 as usize][x.1 as usize] = b'O';
                }
                robot = (robot.0 + direction.0, robot.1 + direction.1);
                map_p1[robot.0 as usize][robot.1 as usize] = b'.';
            }
        }

        robot = (robot_origin.0, robot_origin.1 * 2);
        println!("{:?}", robot_origin);
        for direction in &moves {
            let mut boxes = vec![];
            let check = check_empty_p2(&map_p2, direction, robot, 0, &mut boxes);
            if check {
                robot = (robot.0 + direction.0, robot.1 + direction.1);
                println!("can move {:?}", direction);
                for i in &boxes {
                    map_p2[i.0 as usize][i.1 as usize] = b'.';
                    map_p2[i.0 as usize][i.1 as usize + 1] = b'.';
                }
                for i in &boxes {
                    map_p2[(i.0 + direction.0) as usize][(i.1 + direction.1) as usize] = b'[';
                    map_p2[(i.0 + direction.0) as usize][(i.1 + direction.1) as usize + 1] = b']';
                }
            } else {
                println!("can't move {:?}", direction);
            }

            part2 = 0;
            let mut output = "".to_owned();
            for row in map_p2.iter().enumerate() {
                for item in row.1.iter().enumerate() {
                    if row.0 == robot.0 as usize && item.0 == robot.1 as usize {
                        output.push('@');
                        continue;
                    }
                    output.push(*item.1 as char);
                    if *item.1 == b'[' {
                        part2 += (row.0 * 100 + item.0) as i64;
                    }
                }
                output.push_str("\n");
            }
            println!("{}", output);
        }

        println!("{:?}", part2);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
