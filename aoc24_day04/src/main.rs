use std::fs;
use aoc24_day04;

fn main() {
    let input = fs::read_to_string("aoc24_day04/input.txt")
            .expect("Failed to read test file");
    println!("{}", aoc24_day04::part1(&input));
    println!("{}", aoc24_day04::part2(&input));
}
