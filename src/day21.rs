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
    let mut output = 0;
    for i in temp.windows(2) {
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
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: usize = 0;
    let mut part2: usize = 0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day21.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        cleanup.push(0);

        let now = Instant::now();
        let mut hash = HashMap::new();
        part1 = 0;
        for i in content.lines() {
            let mut t = i.split(',');
            let input = t.next().unwrap().as_bytes();
            let output = search(&mut input.to_vec(), 2, &mut hash);
            part1 += t.next().unwrap().parse::<usize>().unwrap() * output;
        }
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut hash = HashMap::new();
        part2 = 0;
        for i in content.lines() {
            let mut t = i.split(',');
            let input = t.next().unwrap().as_bytes();
            let output = search(&mut input.to_vec(), 25, &mut hash);
            part2 += t.next().unwrap().parse::<usize>().unwrap() * output;
        }
        part2t.push(now.elapsed().as_nanos());
    }

    (
        (part1 as i64, part2 as i64),
        (read, cleanup, part1t, part2t),
    )
}
