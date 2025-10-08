pub fn part1(s: &str) -> i64 {
    let mut total: i64 = 0;

    // Find all the starts of a valid multiply
    let matches: Vec<_> = s.match_indices("mul(").map(|(idx, _)| idx).collect();
    //println!("Matches: {matches:?}");
    for i in matches {
        //println!("Match found at {i} of {}", &s[i..]);

        // Find the first number
        let mut mul_arg1 = 0;
        if i + 4 >= s.len() {
            break;
        }
        let mut tail = &s[(i+4)..];
        let mut len = tail.chars()
            .take_while(|c| c.is_ascii_digit())
            .count();
        if len > 0 && len < 4 {
            //println!("This position has a valid first number of length {len}");
            mul_arg1 = tail[..len].parse().unwrap();
            //println!("First number: {mul_arg1}");
        }

        // Ensure there's a comma after the first number
        let i_comma = i + 4 + len;
        if i_comma >= s.len() || &s[i_comma..i_comma+1] != "," {
            //println!("No comma after first number");
            continue;
        }

        // Find the second number
        let mut mul_arg2 = 0;
        if i_comma + 1 >= s.len() {
            break;
        }
        tail = &s[(i_comma+1)..];
        len = tail.chars()
            .take_while(|c| c.is_ascii_digit())
            .count();
        if len > 0 && len < 4 {
            //println!("This position has a valid second number of length {len}");
            mul_arg2 = tail[..len].parse().unwrap();
            //println!("Second number: {mul_arg2}");
        }

        // Ensure there's a parenthesis after the second number
        let i_paren = i_comma + 1 + len;
        if i_paren >= s.len() || &s[i_paren..i_paren+1] != ")" {
            //println!("No parenthesis after second number");
            continue;
        }

        // Perform the multiplication and add to total
        total += mul_arg1 * mul_arg2;
    }
    total
}

fn find_valid_mul(s: &str) -> (usize, usize, i32, i32) {
    let mut i = 0;
    let mut mul_arg1: i32 = 0;
    
    while i < s.len() {
        let match_idx = s[i..].find("mul(");
        if match_idx.is_none() {
            return (s.len(), 0, 0, 0);
        }
        let idx = i + match_idx.unwrap();
        
        // Find the first number
        if idx + 4 >= s.len() {
            return (s.len(), 0, 0, 0);
        }
        let mut tail = &s[(idx+4)..];
        let l1 = tail.chars()
            .take_while(|c| c.is_ascii_digit())
            .count();
        if l1 > 0 && l1 < 4 {
            mul_arg1 = tail[..l1].parse().unwrap();
        } else {
            i += idx + 4 + l1;
            continue;
        }

        // Ensure there's a comma after the first number
        let i_comma = idx + 4 + l1;
        if i_comma >= s.len() || &s[i_comma..i_comma+1] != "," {
            i = i_comma;
            continue;
        }

        // Find the second number
        let mut mul_arg2 = 0;
        if i_comma + 1 >= s.len() {
            return (s.len(), 0, 0, 0);
        }
        tail = &s[(i_comma+1)..];
        let l2 = tail.chars()
            .take_while(|c| c.is_ascii_digit())
            .count();
        if l2 > 0 && l2 < 4 {
            mul_arg2 = tail[..l2].parse().unwrap();
        }

        // Ensure there's a parenthesis after the second number
        let i_paren = i_comma + 1 + l2;
        if i_paren >= s.len() || &s[i_paren..i_paren+1] != ")" {
            i = i_paren;
            continue;
        }

        let mul_end = i_paren + 1;
        return (idx, mul_end, mul_arg1, mul_arg2);
    }

    return (0, 0, 0, 0);
}

pub fn part2(s: &str) -> i64 {
    let mut total: i64 = 0;
    let mut now_adding = true;
    let mut idx = 0;
    let mut do_idx = 0;
    let mut dont_idx = 0;
    let mut mul_start = 0;
    let mut mul_stop = 0;
    let mut arg1 = 0;
    let mut arg2 = 0;
    
    while idx < s.len() {
        // Find the next multiply
        (mul_start, mul_stop, arg1, arg2) = find_valid_mul(&s[idx..]);
        if mul_stop == 0 {
            break;
        } else {
            // Find the next do/don't switch
            match s[idx..].find("do()") {
                Some(next_switch) => do_idx = next_switch,
                None => do_idx = s.len(),
            }
            match s[idx..].find("don't()") {
                Some(next_switch) => dont_idx = next_switch,
                None => dont_idx = s.len(),
            }
            
            if mul_start < do_idx && mul_start < dont_idx {
                if now_adding {
                    total += (arg1 * arg2) as i64;
                }
                idx += mul_stop;
            } else if do_idx < dont_idx {
                now_adding = true;
                idx += do_idx + 4;
            } else if dont_idx < do_idx {
                now_adding = false;
                idx += dont_idx + 7;
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_find_valid_mul() {
        println!("Testing find_valid_mul...");

        let (start, stop, arg1, arg2) = find_valid_mul("abc mul(12,34) xyz");
        assert_eq!(start, 4);
        assert_eq!(stop, 14);
        assert_eq!(arg1, 12);
        assert_eq!(arg2, 34);
        println!("Passed test");

        let (start, stop, arg1, arg2) = find_valid_mul("no multiply here");
        assert_eq!(start, 16);
        assert_eq!(stop, 0);
        assert_eq!(arg1, 0);
        assert_eq!(arg2, 0);
        println!("Passed test");

        let (start, stop, arg1, arg2) = find_valid_mul("mul(5,6) and mul(7,8)");
        assert_eq!(start, 0);
        assert_eq!(stop, 8);
        assert_eq!(arg1, 5);
        assert_eq!(arg2, 6);
        println!("Passed test");
    }

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("aoc24_day03/test.txt")
            .expect("Failed to read test file");
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("aoc24_day03/test2.txt")
            .expect("Failed to read part 2 test file");
        assert_eq!(part2(&input), 48);
    }
}