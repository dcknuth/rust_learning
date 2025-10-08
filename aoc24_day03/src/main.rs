use std::fs;
use std::time::Instant;

fn main() {
    let input_file = "aoc24_day03/input.txt";
    let input = fs::read_to_string(input_file)
        .expect("Failed to read input file");

    let t1 = Instant::now();
    let part1_ans = aoc24_day03::part1(&input);
    let duration = t1.elapsed();
    println!("Part 1 total is {part1_ans} in {:.4?}", duration);

    let t1 = Instant::now();
    let part2_ans = aoc24_day03::part2(&input);
    let duration = t1.elapsed();
    println!("Part 2 total is {part2_ans} in {:.4?}", duration);
}
