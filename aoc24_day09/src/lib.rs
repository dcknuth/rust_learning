pub fn expand_map(s: &str) -> Vec<i32> {
    let mut disk: Vec<i32> = vec![];
    let mut fnum = 0;
    let mut is_block = true;
    
    for c in s.trim().chars() {
        let num = c.to_digit(10).unwrap();
        if is_block {
            for _ in 0..num {
                disk.push(fnum);
            }
            fnum += 1;
            is_block = false;
        } else {
            for _ in 0..num {
                disk.push(-1);
            }
            is_block = true;
        }
    }
    
    disk
}

pub fn part1(s: &str) -> usize {
    let mut disk = expand_map(s);
    let mut left_i = 0;
    let mut right_i = disk.len() - 1;

    while left_i <= right_i {
        if disk[left_i] > -1 {
            left_i += 1;
        } else if disk[right_i] > -1 {
            disk[left_i] = disk[right_i];
            disk[right_i] = -1;
            right_i -= 1;
        } else {
            right_i -= 1;
        }
    }
    
    // now calc the checksum
    let mut total = 0;
    for (i, fnum) in disk.iter().enumerate() {
        if *fnum > -1 {
            total += i * (*fnum as usize);
        }
    }

    total
}

pub fn part2(s: &str) -> usize {
    let mut disk = expand_map(s);
    let mut right_i = disk.len() - 1;

    let mut first_free = 0;
    loop {
        let mut left_i = first_free;
        let mut r_count = 0;
        
        // find the next file R to L
        while right_i > first_free && disk[right_i] == -1 {
            right_i -= 1;
        }
        let mut cur_fnum = disk[right_i];
        // count the size of the file at our right index
        while disk[right_i] == cur_fnum {
            r_count += 1;
            if right_i == 0 {
                break;
            }
            right_i -= 1;
        }

        // see if we have a free spot large enough
        let mut is_first_free = true;
        while left_i < disk.len() {
            while left_i < right_i && left_i < disk.len() && disk[left_i] != -1 {
                left_i += 1;
            }
            if left_i >= right_i || left_i >= disk.len() {
                break;
            }
            // found a -1, Count size
            if is_first_free {
                first_free = left_i;
                is_first_free = false;
            }
            let mut free_count = 1;
            left_i += 1;
            while left_i < disk.len() && disk[left_i] == -1 {
                left_i += 1;
                free_count += 1;
            }
            if free_count >= r_count {
                // move the file
                for i in 0..r_count {
                    disk[left_i-free_count+i] = cur_fnum;
                    disk[right_i+i+1] = -1;
                }
                break;
            }
        }
        
        if right_i < 1 {
            break;
        }
    }
    
    // now calc the checksum
    let mut total = 0;
    for (i, fnum) in disk.iter().enumerate() {
        if *fnum > -1 {
            total += i * (*fnum as usize);
        }
    }

    total
}

#[cfg(test)]
mod test {
    use std::fs;
    use once_cell::sync::Lazy;
    use super::*;

    static S: Lazy<String> = Lazy::new(|| {
        fs::read_to_string("aoc24_day09/test.txt")
            .expect("Could not read test file")
    });

    #[test]
    fn test_part1() {
        assert_eq!(part1(&S), 1928);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&S), 2858);
    }
}