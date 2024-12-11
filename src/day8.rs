use crate::REPEAT;
use std::fs;
use std::time::Instant;

fn get_antennae(input: &Vec<(&char, (i64, i64))>, filter: &char) -> Vec<(i64, i64)> {
    let mut output = vec![];
    for i in input {
        if i.0 == filter {
            output.push(i.1);
        }
    }
    output
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
            fs::read_to_string("day8.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let content = content
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut antennae = vec![];
        let mut types: Vec<char> = vec![];

        for i in content.iter().enumerate() {
            for j in i.1.iter().enumerate() {
                if j.1 != &'.' {
                    antennae.push((j.1, (j.0 as i64, i.0 as i64)));
                    if !types.contains(j.1) {
                        types.push(*j.1);
                    }
                }
            }
        }
        cleanup.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut antinodes = vec![];
        for i in &types {
            let antennas = get_antennae(&antennae, i);
            for antenna1 in &antennas {
                for antenna2 in &antennas {
                    if antenna1 == antenna2 {
                        continue;
                    }
                    let diff = (antenna1.0 - antenna2.0, antenna1.1 - antenna2.1);
                    let pos = (antenna1.0 + diff.0, antenna1.1 + diff.1);
                    if pos.0 >= content[0].len() as i64 || pos.1 >= content.len() as i64 {
                        continue;
                    }
                    if pos.0 < 0 || pos.1 < 0 {
                        continue;
                    }
                    if !antinodes.contains(&pos) {
                        antinodes.push(pos);
                    }
                }
            }
        }
        part1 = antinodes.len() as i64;
        part1t.push(now.elapsed().as_nanos());

        let now = Instant::now();
        let mut antinodes = vec![];
        for i in &types {
            let antennas = get_antennae(&antennae, i);
            for antenna1 in &antennas {
                for antenna2 in &antennas {
                    if antenna1 == antenna2 {
                        continue;
                    }
                    let diff = (antenna1.0 - antenna2.0, antenna1.1 - antenna2.1);
                    for i in 0..100 {
                        let pos = (antenna1.0 + diff.0 * i, antenna1.1 + diff.1 * i);
                        if pos.0 >= content[0].len() as i64 || pos.1 >= content.len() as i64 {
                            break;
                        }
                        if pos.0 < 0 || pos.1 < 0 {
                            break;
                        }
                        if !antinodes.contains(&pos) {
                            antinodes.push(pos);
                        }
                    }
                }
            }
        }
        part2 = antinodes.len() as i64;
        part2t.push(now.elapsed().as_nanos());

        // let mut output: String = "".to_owned();
        // for i in content.iter().enumerate() {
        //     for j in i.1.iter().enumerate() {
        //         if *j.1 != '.' {
        //             output.push(*j.1);
        //             continue;
        //         }
        //         if antinodes.contains(&(j.0 as i64, i.0 as i64)) {
        //             output.push('#');
        //             continue;
        //         }
        //         output.push(*j.1);
        //     }
        //     output.push_str("\n");
        // }

        // println!("{}", output);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
