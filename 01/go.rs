use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "input";
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("asdf");

    let mut vec = Vec::new();
    // Put all the characters in an array as integers
    for c in contents.chars() {
        // The last char in the string is a null character
        if !c.is_digit(10) {
            continue;
        }
        vec.push(c.to_digit(10).unwrap());
    }
    let len = vec.len();
    let half = len / 2;

    let mut sum_part1 = 0;
    let mut sum_part2 = 0;
    for i in 0..len {
        if vec[i] == vec[(i + 1)%len] {
            sum_part1 = sum_part1 + vec[i];
        }

        if vec[i] == vec[(i + half)%len] {
            sum_part2 = sum_part2 + vec[i];
        }
    }
    println!("sum1: {}", sum_part1);
    println!("sum2: {}", sum_part2);
}
