use std::fs;

fn main() {
    let input = fs::read_to_string("aoc24_day05/input.txt")
        .expect("Failed to read input file");
    let part1_ans = aoc24_day05::part1(&input);
    println!("Part 1 total is {part1_ans}");

    let part2_ans = aoc24_day05::part2(&input);
    println!("Part 2 total is {part2_ans}");
}
