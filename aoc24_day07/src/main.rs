use std::fs;
use aoc24_day07::{part1, part2};

fn main() {
    let input = fs::read_to_string("aoc24_day07/input.txt")
        .expect("Could not read input file");
    let p1_ans = part1(&input);
    println!("Part 1 answer is {p1_ans}");

    let p2_ans = part2(&input);
    println!("Part 2 answer is {p2_ans}");
}
