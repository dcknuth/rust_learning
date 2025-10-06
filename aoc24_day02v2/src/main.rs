use std::fs;
use std::time::Instant;

fn test_line(nums: &Vec<i32>, second_try: bool, skip: usize) -> bool {
    let mut direction_is_set = false;
    let mut failed = false;
    let mut increasing = true;
    let mut failed_at: usize = 0;

    for i in 0..nums.len() - 1 {
        if second_try && i == skip {
            continue;
        }
        let mut next_num = nums[i+1];
        if second_try && i + 1 == skip {
            next_num = nums[i+2];
        }
        if nums[i] < next_num && next_num - nums[i] <= 3 {
            if !direction_is_set {
                increasing = true;
                direction_is_set = true;
            } else if !increasing {
                failed_at = i;
                failed = true;
                break;
            }
        } else if nums[i] > next_num && nums[i] - next_num <= 3 {
            if !direction_is_set {
                increasing = false;
                direction_is_set = true;
            } else if increasing {
                failed_at = i;
                failed = true;
                break;
            }
        } else if nums[i] == next_num {
            failed_at = i;
            failed = true;
            break;
        }
        if (nums[i] - next_num).abs() > 3 {
            failed_at = i;
            failed = true;
            break;
        }
    }
    if !failed {
        return true;
    } else if !second_try {
        // special case if we fail at index 1 as we could have to skip 0
        if failed_at == 1 {
            return test_line(nums, true, 0) ||
                test_line(nums, true, 1) ||
                test_line(nums, true, 2);
        } else if failed_at == nums.len() - 2 {
            return true;
        } else {
            return test_line(nums, true, failed_at) ||
                test_line(nums, true, failed_at + 1);
        }
    }

    false
}

fn main() {
    //let input_file = "aoc24_day02/test.txt";
    let input_file = "aoc24_day02/input.txt";
    let contents = fs::read_to_string(input_file)
        .expect("Failed to read file");
    
    // Part 1
    let t1 = Instant::now();
    let mut total_safe = 0;
    for line in contents.lines() {
        let nums: Vec<u32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        let mut set_count = 0;
        let mut num_inline = 0;
        let mut increasing = true;
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i + 1] && nums[i] + 3 >= nums[i + 1] {
                if set_count < 1 {
                    increasing = true;
                    set_count += 1;
                } else if !increasing {
                    break;
                }
                num_inline += 1;
            } else if nums[i] > nums[i+1] && nums[i] <= nums[i + 1] + 3 {
                if set_count < 1 {
                    increasing = false;
                    set_count += 1;
                } else if increasing {
                    break;
                }
                num_inline += 1;
            } else if nums[i] == nums[i+1] {
                break;
            }
        }
        if num_inline == nums.len() - 1 {
            total_safe += 1;
            //println!("Line {line_num} is safe: {line}");
        } else {
            //println!("Line {line_num} is not safe: {line}");
        }
    }
    let duration = t1.elapsed();
    println!("Part1 safe lines: {total_safe} in {:.4?}", duration);

    // Part 2
    let t1 = Instant::now();
    let mut total_safe = 0;
    for line in contents.lines() {
        let nums: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        if test_line(&nums, false, 0) {
            total_safe += 1;
            //println!("Line {line_num} is safe: {line}");
        } else {
            //println!("Line {line_num} is not safe: {line}");
        }
    }
    let duration = t1.elapsed();
    println!("Part2 safe lines: {total_safe} in {:.4?} ms", duration);
}

