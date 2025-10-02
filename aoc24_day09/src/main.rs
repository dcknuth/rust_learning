use std::fs;
use aoc24_day09::{part1, part2};

fn main() {
    let s = fs::read_to_string("aoc24_day09/input.txt")
        .expect("Could not open input file");
    let p1_ans = part1(&s);
    println!("Part 1 answer is {p1_ans}");

    let p2_ans = part2(&s);
    println!("Part 2 answer is {p2_ans}");
}
