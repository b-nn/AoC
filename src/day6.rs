#![allow(dead_code)]
use std::time::Instant;
use std::{fs, usize, vec};

use crate::REPEAT;
#[derive(Clone, Eq, PartialEq, Debug, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone)]
enum State {
    Empty,
    Visited(Direction),
    Block,
}

#[derive(Clone)]
struct Guard {
    direction: Direction,
    position: Vec<usize>,
}

fn check_path(map: &mut Vec<Vec<State>>, mut guard: Guard) -> (bool, Vec<(usize, usize)>) {
    let mut inbound = true;
    let mut path = vec![];

    while inbound {
        let position;
        match &map[guard.position[1]][guard.position[0]] {
            &State::Empty => {
                map[guard.position[1]][guard.position[0]] = State::Visited(guard.direction.clone());
                path.push((guard.position[0], guard.position[1]));
            }
            &State::Visited(x) => {
                if x == guard.direction {
                    return (false, path);
                }
            }
            _ => (),
        }
        match guard.moved() {
            Some(x) => {
                position = x;
            }
            None => {
                inbound = false;
                continue;
            }
        }
        if position[1] >= map.len() || position[0] >= map[0].len() {
            inbound = false;
            continue;
        }
        match &map[position[1]][position[0]] {
            State::Block => {
                guard.turn();
            }
            State::Empty => {
                guard.position = position.clone();
            }
            State::Visited(_) => {
                guard.position = position.clone();
            }
        }
    }

    (true, path)
}

impl Guard {
    fn moved(&self) -> Option<Vec<usize>> {
        match self.direction {
            Direction::Up => {
                if self.position[1] == 0 {
                    None
                } else {
                    Some(vec![self.position[0], self.position[1] - 1])
                }
            }
            Direction::Right => Some(vec![self.position[0] + 1, self.position[1]]),
            Direction::Down => Some(vec![self.position[0], self.position[1] + 1]),
            Direction::Left => {
                if self.position[0] == 0 {
                    None
                } else {
                    Some(vec![self.position[0] - 1, self.position[1]])
                }
            }
        }
    }
    fn turn(&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];

    let mut part1 = 0;
    let mut part2 = 0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day6.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let mut guard = Guard {
            direction: Direction::Up,
            position: vec![0, 0],
        };
        let mut original_guard = vec![0, 0];

        let now = Instant::now();
        let map = content
            .lines()
            .enumerate()
            .map(|(y, j)| {
                j.chars()
                    .enumerate()
                    .map(|(x, i)| match i {
                        '#' => State::Block,
                        '^' => {
                            original_guard = vec![x, y];
                            State::Empty
                        }
                        _ => State::Empty,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        guard.position = original_guard.clone();
        let path = check_path(&mut map.clone(), guard.clone()).1;
        part1 = path.len();
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut count = 0;
        for i in &path {
            let mut map = map
                .iter()
                .enumerate()
                .map(|(y, j)| {
                    j.iter()
                        .enumerate()
                        .map(|(x, state)| {
                            if x == i.0 && y == i.1 {
                                State::Block
                            } else {
                                state.clone()
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            if !check_path(
                &mut map,
                Guard {
                    direction: Direction::Up,
                    position: original_guard.clone(),
                },
            )
            .0
            {
                count += 1;
            }
        }
        part2t.push(now.elapsed().as_nanos());

        part2 = count;
    }

    ((part1 as i64, part2), (read, cleanup, part1t, part2t))
}
