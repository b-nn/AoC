use std::fs;

fn check_xmas(input: &Vec<Vec<char>>, i: (i32, i32)) -> i32 {
    let mut count = 0;

    // part 1
    // for x in -1..2 {
    //     for y in -1..2 {
    //         if input[i.0 as usize][i.1 as usize] == 'X'
    //             && input[(i.0 + 1 * x) as usize][(i.1 + 1 * y) as usize] == 'M'
    //             && input[(i.0 + 2 * x) as usize][(i.1 + 2 * y) as usize] == 'A'
    //             && input[(i.0 + 3 * x) as usize][(i.1 + 3 * y) as usize] == 'S'
    //         {
    //             count += 1;
    //         }
    //     }
    // }

    for (x, y) in [(1, 1), (1, -1), (-1, 1), (-1, -1)] {
        if input[i.0 as usize][i.1 as usize] == 'A'
            && input[(i.0 + x) as usize][(i.1 + y) as usize] == 'M'
            && input[(i.0 - x) as usize][(i.1 - y) as usize] == 'S'
        {
            count += 1;
        }
    }

    if count > 2 {
        println!("HUH ???????");
        1
    } else if count == 2 {
        1
    } else {
        0
    }
}

pub fn run() -> i32 {
    let result = 0;

    let content = fs::read_to_string("day4.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");

    let content = content
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;
    for x in content.iter().enumerate() {
        for y in x.1.iter().enumerate() {
            if y.1 == &'A' {
                println!("{} {}", x.0, y.0);
                count += check_xmas(&content, (x.0 as i32, y.0 as i32));
            }
        }
    }

    count
}
