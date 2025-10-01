use std::fs;
use aoc24_day08::{part1, part2};

fn main() {
    let input = fs::read_to_string("aoc24_day08/input.txt")
        .expect("Could not open input file");
    let p1_ans = part1(&input);
    println!("{p1_ans}");

    let p2_ans = part2(&input);
    println!("{p2_ans}");
}
