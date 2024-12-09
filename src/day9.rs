use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn trim(input: &mut Vec<Option<usize>>) -> usize {
    let element = input.pop().unwrap().unwrap();
    loop {
        let t = input.pop().unwrap();
        if t.is_some() {
            input.push(t);
            break;
        }
    }

    element
}

fn get_spans(input: &Vec<u32>, filter: bool) -> Vec<(usize, usize)> {
    // (index, size of span)
    // false = None, true = Some(_)
    let mut spans = vec![];
    let mut index = 0;

    for i in input.iter().enumerate() {
        if i.0 % 2 == !filter as usize {
            spans.push((index, *i.1 as usize));
        }
        index += *i.1 as usize;
    }

    spans
}

// fn print_blocks(blocks: &Vec<Option<usize>>) {
//     let mut output: String = "".to_owned();
//     for j in blocks {
//         output.push_str(&match j {
//             Some(x) => x.to_string(),
//             None => ".".to_string(),
//         });
//     }
//     println!("{}", output);
// }

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut total: (i64, i64) = (0, 0);

    for _i in 0..REPEAT {
        let now = Instant::now();
        let mut content =
            fs::read_to_string("day9.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        content.pop();
        let content = content
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<_>>();

        let mut blocks: Vec<Option<usize>> = vec![];
        for i in content.iter().enumerate() {
            let mut temp: Vec<Option<usize>>;
            if i.0 % 2 == 1 {
                temp = vec![None; *i.1 as usize];
            } else {
                temp = vec![Some(i.0 / 2); *i.1 as usize];
            }
            blocks.append(&mut temp);
        }
        let mut blocks_part_1 = blocks.clone();
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut index = 0;
        for block in blocks_part_1.clone() {
            if blocks_part_1.len() == index {
                break;
            }
            if block == None {
                let last = trim(&mut blocks_part_1);
                blocks_part_1[index] = Some(last);
                total.0 += (index * last) as i64;
            } else {
                total.0 += (index * block.unwrap()) as i64;
            }
            index += 1;
        }
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut clear = get_spans(&content, false);

        for block in get_spans(&content, true).iter().rev().collect::<Vec<_>>() {
            let mut remove = None;
            let mut removed = 0;
            for spot in clear.iter().enumerate() {
                if spot.1 .1 >= block.1 && spot.1 .0 < block.0 {
                    for j in 0..block.1 {
                        blocks.swap(spot.1 .0 + j, block.0 + j);
                        removed += 1;
                    }
                    remove = Some(spot.0);
                    break;
                }
            }
            if let Some(x) = remove {
                if clear[x].1 == removed {
                    clear.remove(x);
                } else {
                    clear[x] = (clear[x].0 + removed, clear[x].1 - removed);
                }
            }
        }
        for i in blocks.iter().enumerate() {
            if let Some(x) = i.1 {
                total.1 += (x * i.0) as i64;
            }
        }
        part2t.push(now.elapsed().as_nanos());

        println!("{:?}", total);

        //0099811188827763336446555
        //0099811188827773336446555566
        //0099811188827773336446555566
    }

    (total, (read, cleanup, part1t, part2t))
}
