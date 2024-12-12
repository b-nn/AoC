use crate::REPEAT;
use std::fs;
use std::time::Instant;

// uses rc coordinates
fn search_p2(
    map: &mut Vec<Vec<char>>,
    position: (usize, usize),
    plant: char,
    searched: &mut Vec<(usize, usize)>,
) -> (i32, i32) {
    fn get_corners(map: &mut Vec<Vec<char>>, pos: (usize, usize), p: char) -> i64 {
        let mut total = 0;
        for i in 0..2 {
            let i = i * 2 - 1;
            for j in 0..2 {
                let j = j * 2 - 1;
                let matches = (map[pos.0 + i][pos.1 + j] == p) as i64
                    + (map[pos.0 + i][pos.1] == p) as i64
                    + (map[pos.0][pos.1 + j] == p) as i64
                    + (map[pos.0][pos.1] == p) as i64;
                total += matches % 2;
            }
        }
        total
    }

    get_corners(map, position, plant);

    if searched.contains(&position) {
        return (0, 0);
    }
    searched.push(position);
    let up = if map[position.0 + 1][position.1] == plant {
        search_p2(map, (position.0 + 1, position.1), plant, searched)
    } else {
        (0, 1)
    };
    let down = if map[position.0 - 1][position.1] == plant {
        search_p2(map, (position.0 - 1, position.1), plant, searched)
    } else {
        (0, 1)
    };
    let right = if map[position.0][position.1 + 1] == plant {
        search_p2(map, (position.0, position.1 + 1), plant, searched)
    } else {
        (0, 1)
    };
    let left = if map[position.0][position.1 - 1] == plant {
        search_p2(map, (position.0, position.1 - 1), plant, searched)
    } else {
        (0, 1)
    };

    (
        up.0 + down.0 + right.0 + left.0 + 1,
        up.1 + down.1 + right.1 + left.1,
    )
}

fn search_p1(
    map: &mut Vec<Vec<char>>,
    position: (usize, usize),
    plant: char,
    searched: &mut Vec<(usize, usize)>,
) -> (i32, i32) {
    if searched.contains(&position) {
        return (0, 0);
    }
    searched.push(position);
    let up = if map[position.0 + 1][position.1] == plant {
        search_p1(map, (position.0 + 1, position.1), plant, searched)
    } else {
        (0, 1)
    };
    let down = if map[position.0 - 1][position.1] == plant {
        search_p1(map, (position.0 - 1, position.1), plant, searched)
    } else {
        (0, 1)
    };
    let right = if map[position.0][position.1 + 1] == plant {
        search_p1(map, (position.0, position.1 + 1), plant, searched)
    } else {
        (0, 1)
    };
    let left = if map[position.0][position.1 - 1] == plant {
        search_p1(map, (position.0, position.1 - 1), plant, searched)
    } else {
        (0, 1)
    };

    (
        up.0 + down.0 + right.0 + left.0 + 1,
        up.1 + down.1 + right.1 + left.1,
    )
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
            fs::read_to_string("day12.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");
        read.push(now.elapsed().as_nanos());

        let mut content = content
            .lines()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut searched = vec![];
        let mut total = 0;
        for row in 1..content.len() - 1 {
            for column in 1..content[0].len() - 1 {
                if searched.contains(&(row, column)) {
                    continue;
                }
                let plant = content[row][column];
                let search = search_p1(&mut content, (row, column), plant, &mut searched);
                total += search.0 * search.1;
                println!("{:?} {}", search, content[row][column]);
            }
        }
        println!("{}", total);
    }

    ((part1, part2), (read, cleanup, part1t, part2t))
}
