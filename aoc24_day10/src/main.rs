use std::fs;
use aoc24_day10::{part1, part2};
use std::time::Instant;

fn main() {
    let s = fs::read_to_string("aoc24_day10/input.txt").
        expect("Could not read input file");
    
    let t0 = Instant::now();
    let p1_ans = part1(&s);
    let duration = t0.elapsed();
    println!("Part 1 answer is {p1_ans} in {:.4?}", duration);

    let t0 = Instant::now();
    let p2_ans = part2(&s);
    let duration = t0.elapsed();
    println!("Part 2 answer is {p2_ans} in {:.4?}", duration);
}
