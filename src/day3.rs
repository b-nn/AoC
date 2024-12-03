use std::fs;
use std::process::Output;
use std::vec;

fn get_int(input: &Vec<char>, i: &usize) -> Option<(u32, usize)> {
    let mut output = 0;
    let mut index = i.clone();
    let mut success = false;
    println!("{}", input[index]);
    while input[index].to_digit(10).is_some() {
        success = true;
        output *= 10;
        output += input[index].to_digit(10).unwrap();
        index += 1;
    }

    if success {
        Some((output, index))
    } else {
        None
    }
}

pub fn run() -> u32 {
    let mut result = 0;
    let contents = fs::read_to_string("day3.txt").expect("THERE'S NO INPUT WHAT THE FUCKKKKKKKK");

    let input = contents.chars().collect::<Vec<_>>();
    let mut index = 0;
    let mut enabled = true;
    for j in input.windows(7) {
        if j[..4] == ['d', 'o', '(', ')'] {
            enabled = true;
        }
        if j[..7] == ['d', 'o', 'n', '\'', 't', '(', ')'] {
            enabled = false;
        }
        if j[..4] == ['m', 'u', 'l', '('] && enabled {
            let output_1 = get_int(&input, &(index + 4));
            if output_1.is_none() {
                index += 1;
                continue;
            }
            if input[output_1.unwrap().1] != ',' {
                index += 1;
                continue;
            }
            let output_2 = get_int(&input, &(output_1.unwrap().1 + 1));
            if output_2.is_none() {
                index += 1;
                continue;
            }
            if input[output_2.unwrap().1] != ')' {
                index += 1;
                continue;
            }
            println!("{} {}", output_1.unwrap().0, output_2.unwrap().0);
            result += output_1.unwrap().0 * output_2.unwrap().0;
        }
        index += 1;
    }

    result
}
