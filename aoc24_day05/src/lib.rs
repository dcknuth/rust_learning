use std::collections::HashMap;

pub fn part1(input: &str) -> i64 {
    // Rules will have the earlier page as key and the later as value 
    let mut rules: HashMap<i32, HashMap<i32, bool>> = HashMap::new();
    // The input is in two parts, so we need the index to separate them
    let mut input_i = 0;
    let mut total: i64 = 0;

    let lines = input.lines().collect::<Vec<&str>>();
    while input_i < lines.len() {
        if lines[input_i] == "" {
            input_i += 1;
            break;
        } else {
            let rule_parts: Vec<i32> = lines[input_i].split("|").
                map(|s| s.trim().parse().expect("Invalid Int")).
                collect();
            if let Some(later_pages) = rules.get_mut(&rule_parts[0]) {
                later_pages.insert(rule_parts[1], true);
            } else {
                let mut later_pages: HashMap<i32, bool> = HashMap::new();
                later_pages.insert(rule_parts[1], true);
                rules.insert(rule_parts[0], later_pages);
            }
            input_i += 1;
        }
    }
    while input_i < lines.len() {
        // Pages will have the page as the key and position as value
        let mut pages: HashMap<i32, usize> = HashMap::new();
        let pages_list: Vec<i32> = lines[input_i].split(",").
            map(|s| s.trim().parse().expect("Invalid page num")).
            collect();
        for (pos, page) in pages_list.iter().enumerate() {
            pages.insert(*page, pos);
        }
        input_i += 1;
        
        let mut good_list = true;
        for (pos, page) in pages_list.iter().enumerate() {
            if let Some(later_pages) = rules.get(page) {
                for later_page in later_pages.keys() {
                    if let Some(later_pos) = pages.get(later_page) {
                        if *later_pos < pos {
                            good_list = false;
                            break;
                        }
                    }
                }
            }
        }
        if good_list {
            // println!("Good list: {:?}", pages_list);
            // println!("Middle page: {}", pages_list[pages_list.len() / 2]);
            total += pages_list[pages_list.len() / 2] as i64;
        }
    }
    
    total
}

pub fn part2(input: &str) -> i64 {
    // We will just copy code from part1 to get the bad lists
    let mut rules: HashMap<i32, HashMap<i32, bool>> = HashMap::new();
    let mut input_i = 0;
    let mut total: i64 = 0;

    let lines = input.lines().collect::<Vec<&str>>();
    while input_i < lines.len() {
        if lines[input_i] == "" {
            input_i += 1;
            break;
        } else {
            let rule_parts: Vec<i32> = lines[input_i].split("|").
                map(|s| s.trim().parse().expect("Invalid Int")).
                collect();
            if let Some(later_pages) = rules.get_mut(&rule_parts[0]) {
                later_pages.insert(rule_parts[1], true);
            } else {
                let mut later_pages: HashMap<i32, bool> = HashMap::new();
                later_pages.insert(rule_parts[1], true);
                rules.insert(rule_parts[0], later_pages);
            }
            input_i += 1;
        }
    }
    let mut bad_lists: Vec<Vec<i32>> = Vec::new();
    while input_i < lines.len() {
        // Pages will have the page as the key and position as value
        let mut pages: HashMap<i32, usize> = HashMap::new();
        let pages_list: Vec<i32> = lines[input_i].split(",").
            map(|s| s.trim().parse().expect("Invalid page num")).
            collect();
        for (pos, page) in pages_list.iter().enumerate() {
            pages.insert(*page, pos);
        }
        input_i += 1;
        
        'outer: for (pos, page) in pages_list.iter().enumerate() {
            if let Some(later_pages) = rules.get(page) {
                for later_page in later_pages.keys() {
                    if let Some(later_pos) = pages.get(later_page) {
                        if *later_pos < pos {
                            bad_lists.push(pages_list.clone());
                            break 'outer;
                        }
                    }
                }
            }
        }
    }
    // now that we have the bad lists, we can try fixing them
    // In Python, I think I made a custom sort function, but here let's just try
    //  swapping bad pairs as they are found until the list is good. This is
    //  probably bubble-sort level inefficient
    for bad_list in bad_lists.iter_mut() {
        let mut fixed = false;
        while !fixed {
            fixed = true;
            let mut pages: HashMap<i32, usize> = HashMap::new();
            for (pos, page) in bad_list.iter().enumerate() {
                pages.insert(*page, pos);
            }
            'filter_outer: for (pos, page) in bad_list.iter().enumerate() {
                if let Some(later_pages) = rules.get(page) {
                    for later_page in later_pages.keys() {
                        if let Some(later_pos) = pages.get(later_page) {
                            if *later_pos < pos {
                                // swap the two
                                let temp = bad_list[pos];
                                bad_list[pos] = bad_list[*later_pos];
                                bad_list[*later_pos] = temp;
                                fixed = false;
                                break 'filter_outer;
                            }
                        }
                    }
                }
            }
        }
        // println!("Fixed list: {:?}", bad_list);
        // println!("Middle page: {}", bad_list[bad_list.len() / 2]);
        total += bad_list[bad_list.len() / 2] as i64;
    }
    total
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_part1() {
        let s = fs::read_to_string("aoc24_day05/test.txt")
            .expect("Failed to read test file");
        assert_eq!(part1(&s), 143);
    }

    #[test]
    fn test_part2() {
        let s = fs::read_to_string("aoc24_day05/test.txt")
            .expect("P2 failed to read test file");
        assert_eq!(part2(&s), 123);
    }
}