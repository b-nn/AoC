use crate::REPEAT;
use core::panic;
use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn search<'a, 'b>(
    input: &'a mut Vec<u8>,
    depth: i64,
    hash: &mut HashMap<((u8, u8), i64), usize>,
) -> usize {
    if depth == 0 {
        return input.len();
    }

    let mut temp = vec![b'A'];
    temp.append(input);
    *input = temp;
    let mut output = 0;
    for i in input.windows(2) {
        if let Some(x) = hash.get(&((i[0], i[1]), depth)) {
            output += x;
        } else {
            let mut t = get_paths((i[0], i[1]));
            let searched = search(&mut t, depth - 1, hash);
            hash.insert(((i[0], i[1]), depth), searched);
            output += searched;
        }
    }

    output
}

fn get_paths_keypad<'a>(input: (u8, u8)) -> Vec<Vec<u8>> {
    let first: (i32, i32) = match input.0 {
        b'7' => (0, 0),
        b'8' => (0, 1),
        b'9' => (0, 2),
        b'4' => (1, 0),
        b'5' => (1, 1),
        b'6' => (1, 2),
        b'1' => (2, 0),
        b'2' => (2, 1),
        b'3' => (2, 2),
        b'0' => (3, 1),
        b'A' => (3, 2),
        _ => panic!("invalid input"),
    };
    let second: (i32, i32) = match input.1 {
        b'7' => (0, 0),
        b'8' => (0, 1),
        b'9' => (0, 2),
        b'4' => (1, 0),
        b'5' => (1, 1),
        b'6' => (1, 2),
        b'1' => (2, 0),
        b'2' => (2, 1),
        b'3' => (2, 2),
        b'0' => (3, 1),
        b'A' => (3, 2),
        _ => panic!("invalid input"),
    };

    let mut output = vec![vec![]];
    if (first.1 - second.0).abs() == 2 && first.0 < 2 {
        for _i in 0..(first.1 - second.1).abs() {
            if (first.1 - second.1).signum() > 0 {
                output[0].push(b'<');
            } else {
                output[0].push(b'>');
            }
        }
        for _i in 0..(first.0 - second.0).abs() {
            if (first.0 - second.0).signum() > 0 {
                output[0].push(b'^');
            } else {
                output[0].push(b'v');
            }
        }
    } else {
        for _i in 0..(first.0 - second.0).abs() {
            if (first.0 - second.0).signum() > 0 {
                output[0].push(b'^');
            } else {
                output[0].push(b'v');
            }
        }
        for _i in 0..(first.1 - second.1).abs() {
            if (first.1 - second.1).signum() > 0 {
                output[0].push(b'<');
            } else {
                output[0].push(b'>');
            }
        }
    }
    let t = output.len();
    output[0].push(b'A');
    output
}

fn get_paths<'a>(input: (u8, u8)) -> Vec<u8> {
    match input {
        (b'A', b'A') => b"A".to_vec(),
        (b'A', b'^') => b"<A".to_vec(),
        (b'A', b'>') => b"vA".to_vec(),
        (b'A', b'v') => b"<vA".to_vec(),
        (b'A', b'<') => b"v<<A".to_vec(),
        (b'^', b'A') => b">A".to_vec(),
        (b'^', b'^') => b"A".to_vec(),
        (b'^', b'>') => b"v>A".to_vec(),
        (b'^', b'v') => b"vA".to_vec(),
        (b'^', b'<') => b"v<A".to_vec(),
        (b'>', b'A') => b"^A".to_vec(),
        (b'>', b'^') => b"<^A".to_vec(),
        (b'>', b'>') => b"A".to_vec(),
        (b'>', b'v') => b"<A".to_vec(),
        (b'>', b'<') => b"<<A".to_vec(),
        (b'v', b'A') => b"^>A".to_vec(),
        (b'v', b'^') => b"^A".to_vec(),
        (b'v', b'>') => b">A".to_vec(),
        (b'v', b'v') => b"A".to_vec(),
        (b'v', b'<') => b"<A".to_vec(),
        (b'<', b'A') => b">>^A".to_vec(),
        (b'<', b'^') => b">^A".to_vec(),
        (b'<', b'>') => b">>A".to_vec(),
        (b'<', b'v') => b">A".to_vec(),
        (b'<', b'<') => b"A".to_vec(),
        _ => panic!("FUCKKKKK {} {}", input.0 as char, input.1 as char),
    }
    // let first: (i32, i32) = match input.0 {
    //     b'^' => (0, 1),
    //     b'A' => (0, 2),
    //     b'<' => (1, 0),
    //     b'v' => (1, 1),
    //     b'>' => (1, 2),
    //     _ => panic!("invalid input"),
    // };
    // let second: (i32, i32) = match input.1 {
    //     b'^' => (0, 1),
    //     b'A' => (0, 2),
    //     b'<' => (1, 0),
    //     b'v' => (1, 1),
    //     b'>' => (1, 2),
    //     _ => panic!("invalid input"),
    // };

    // let mut output = vec![];
    // for _i in 0..(first.0 - second.0).abs() {
    //     if (first.0 - second.0).signum() > 0 {
    //         output.push(b'^');
    //     } else {
    //         output.push(b'v');
    //     }
    // }
    // for _i in 0..(first.1 - second.1).abs() {
    //     if (first.1 - second.1).signum() > 0 {
    //         output.push(b'<');
    //     } else {
    //         output.push(b'>');
    //     }
    // }
    // let t = output.len();
    // let out = output.into_iter().permutations(t).collect_vec();
    // out
}

fn control(input: (u8, u8)) -> Vec<u8> {
    let first: (i32, i32) = match input.0 {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => panic!("invalid input"),
    };
    let second: (i32, i32) = match input.1 {
        b'^' => (0, 1),
        b'A' => (0, 2),
        b'<' => (1, 0),
        b'v' => (1, 1),
        b'>' => (1, 2),
        _ => panic!("invalid input"),
    };
    let mut output = vec![];
    for _i in 0..(first.0 - second.0).abs() {
        if (first.0 - second.0).signum() > 0 {
            output.push(b'^');
        } else {
            output.push(b'v');
        }
    }
    for _i in 0..(first.1 - second.1).abs() {
        if (first.1 - second.1).signum() > 0 {
            output.push(b'<');
        } else {
            output.push(b'>');
        }
    }
    output.push(b'A');
    output
}

fn keypad(input: (u8, u8)) -> Vec<u8> {
    let first: (i32, i32) = match input.0 {
        b'7' => (0, 0),
        b'8' => (0, 1),
        b'9' => (0, 2),
        b'4' => (1, 0),
        b'5' => (1, 1),
        b'6' => (1, 2),
        b'1' => (2, 0),
        b'2' => (2, 1),
        b'3' => (2, 2),
        b'0' => (3, 1),
        b'A' => (3, 2),
        _ => panic!("invalid input"),
    };
    let second: (i32, i32) = match input.1 {
        b'7' => (0, 0),
        b'8' => (0, 1),
        b'9' => (0, 2),
        b'4' => (1, 0),
        b'5' => (1, 1),
        b'6' => (1, 2),
        b'1' => (2, 0),
        b'2' => (2, 1),
        b'3' => (2, 2),
        b'0' => (3, 1),
        b'A' => (3, 2),
        _ => panic!("invalid input"),
    };
    let mut output = vec![];
    for _i in 0..(first.0 - second.0).abs() {
        if (first.0 - second.0).signum() > 0 {
            output.push(b'^');
        } else {
            output.push(b'v');
        }
    }
    for _i in 0..(first.1 - second.1).abs() {
        if (first.1 - second.1).signum() > 0 {
            output.push(b'<');
        } else {
            output.push(b'>');
        }
    }
    output.push(b'A');
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
        let content =
            fs::read_to_string("day21.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());
        let mut total = 0;

        let mut hash = HashMap::new();

        for i in content.lines() {
            let mut t = i.split(',');
            let input = t.next().unwrap().as_bytes();
            let output = search(&mut input.to_vec(), 25, &mut hash);
            total += t.next().unwrap().parse::<usize>().unwrap() * output;
        }
        println!("{}", total);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
