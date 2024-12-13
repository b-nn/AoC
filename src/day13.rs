use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn counts(a: (f64, f64), b: (f64, f64), total: (f64, f64)) -> Option<(f64, f64)> {
    let b_presses = (a.1 * total.0 - total.1 * a.0) / (a.1 * b.0 - a.0 * b.1);
    // check if it's lower than 0.0001 to
    // account for imprecision shenanigans (may be premature but oh well)

    if (b_presses.round() - b_presses).abs() < 0.0001 {
        let a_presses = (total.0 - b.0 * b_presses) / a.0;
        if (a_presses.round() - a_presses).abs() < 0.0001 {}
        return Some((a_presses, b_presses));
    }
    None
}

pub fn run() -> ((i64, i64), (Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>)) {
    let mut read: Vec<u128> = vec![];
    let mut cleanup: Vec<u128> = vec![];
    let mut part1t: Vec<u128> = vec![];
    let mut part2t: Vec<u128> = vec![];
    let mut part1: f64 = 0.0;
    let mut part2: f64 = 0.0;

    for _i in 0..REPEAT {
        let now = Instant::now();
        let content =
            fs::read_to_string("day13.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let t = content
            .replace("X+", "")
            .replace("Y+", "")
            .replace("X=", "")
            .replace("Y=", "")
            .replace("Button A: ", "")
            .replace("Button B: ", "")
            .replace("Prize: ", "");
        let content = t.lines().filter(|x| *x != "");
        let mut a: (f64, f64) = (0.0, 0.0);
        let mut b: (f64, f64) = (0.0, 0.0);
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        for (index, item) in content.enumerate() {
            let mut t = item.split(", ");
            let x = t.next().unwrap().parse::<f64>().unwrap();
            let y = t.next().unwrap().parse::<f64>().unwrap();
            match index % 3 {
                0 => a = (x, y),
                1 => b = (x, y),
                _ => {
                    if let Some(counts) = counts(a, b, (x, y)) {
                        part1 += counts.0 * 3.0 + counts.1;
                    }
                    if let Some(counts) =
                        counts(a, b, (x + 10000000000000f64, y + 10000000000000f64))
                    {
                        part2 += counts.0 * 3.0 + counts.1;
                    }
                }
            }
        }
        part1t.push(0);
        part2t.push(now.elapsed().as_nanos());
    }

    (
        (part1 as i64, part2 as i64),
        (read, cleanup, part1t, part2t),
    )
}
