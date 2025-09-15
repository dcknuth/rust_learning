use std::fs;

fn main() {
    //let input_file = "aoc24_day02/test.txt";
    let input_file = "aoc24_day02/input.txt";
    let contents = fs::read_to_string(input_file)
        .expect("Failed to read file");
    let lines: Vec<&str> = contents.lines().collect();
    
    // Part 1
    let mut total_safe = 0;
    for (line_num, line) in lines.iter().enumerate() {
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
            println!("Line {line_num} is safe: {line}");
        } else {
            println!("Line {line_num} is not safe: {line}");
        }
    }
    println!("Part1 safe lines: {total_safe}");

    // Part 2
    fn test_line(nums: &Vec<i32>, second_try: bool) -> bool {
        let mut set_count = 0;
        let mut num_inline = 0;
        let mut increasing = true;
        let mut failed_at = -1;
        for i in 0..nums.len() - 1 {
            if nums[i] < nums[i + 1] && nums[i+1] - nums[1] <= 3 {
                if set_count < 1 {
                    increasing = true;
                    set_count += 1;
                } else if !increasing {
                    failed_at = i as i32;
                    break;
                }
            } else if nums[i] > nums[i+1] && nums[i] - nums[i + 1] <= 3 {
                if set_count < 1 {
                    increasing = false;
                    set_count += 1;
                } else if increasing {
                    failed_at = i as i32;
                    break;
                }
            } else if nums[i] == nums[i+1] {
                failed_at = i as i32;
                break;
            }
            if (nums[i] - nums[i + 1]).abs() > 3 {
                failed_at = i as i32;
                break;
            }
            num_inline += 1;
        }
        if num_inline == nums.len() - 1 {
            return true;
        } else if !second_try {
            if failed_at > 0 {
                let mut l3 = nums.clone();
                l3.remove((failed_at - 1) as usize);
                if test_line(&l3, true) {
                    return true;
                }
            }
            let mut l1 = nums.clone();
            let mut l2 = nums.clone();
            l1.remove(failed_at as usize);
            l2.remove((failed_at + 1) as usize);
            return test_line(&l1, true) || 
                test_line(&l2, true);
        }
        return false;
    }

    let mut total_safe = 0;
    for (line_num, line) in lines.iter().enumerate() {
        let nums: Vec<i32> = line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

        if test_line(&nums, false) {
            total_safe += 1;
            println!("Line {line_num} is safe: {line}");
        } else {
            println!("Line {line_num} is not safe: {line}");
        }
    }
    println!("Part2 safe lines: {total_safe}");
}
