use std::fs;
use aoc24_day06::{part1, part2};

fn main() {
    let s = fs::read_to_string("input.txt")
        .expect("Could not read input file");
    let p1_ans = part1(&s);
    println!("{p1_ans}");
    let p2_ans = part2(&s);
    println!("{p2_ans}")
}
