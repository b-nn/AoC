use crate::REPEAT;
use core::panic;
use std::fs;
use std::time::Instant;

fn check_empty(
    map: &Vec<Vec<u8>>,
    direction: (i32, i32),
    position: (i32, i32),
    depth: i32,
) -> (bool, Option<(i32, i32)>) {
    if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b'#' {
        return (false, None);
    } else if map[(position.0 + direction.0) as usize][(position.1 + direction.1) as usize] == b'O'
    {
        return check_empty(
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

        let mut robot: (i32, i32) = (0, 0);
        let mut map = content
            .lines()
            .enumerate()
            .map(|(i, x)| {
                x.bytes()
                    .enumerate()
                    .map(|(j, y)| match y {
                        b'@' => {
                            robot = (i as i32, j as i32);
                            b'.'
                        }
                        _ => y,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        println!("{:?}", moves);
        let moves = moves
            .trim()
            .chars()
            .map(|x| match x {
                '<' => (0, -1),
                '>' => (0, 1),
                '^' => (-1, 0),
                'v' => (1, 0),
                x => panic!("HUHHHHHH {}", x),
            })
            .collect::<Vec<_>>();

        for direction in moves {
            let check = check_empty(&map, direction, robot, 0);
            if check.0 {
                if let Some(x) = check.1 {
                    map[x.0 as usize][x.1 as usize] = b'O';
                }
                robot = (robot.0 + direction.0, robot.1 + direction.1);
                map[robot.0 as usize][robot.1 as usize] = b'.';
            }
        }
        let mut output = "".to_owned();
        for row in map.iter().enumerate() {
            for item in row.1.iter().enumerate() {
                if row.0 == robot.0 as usize && item.0 == robot.1 as usize {
                    output.push('@');
                    continue;
                }
                output.push(match item.1 {
                    b'.' => '.',
                    b'O' => {
                        part1 += (row.0 * 100 + item.0) as i64;
                        'O'
                    }
                    b'#' => '#',
                    _ => '?',
                })
            }
            output.push_str("\n");
        }
        println!("{}", output);

        println!("{:?}", map);
        println!("{:?}", part1);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
