advent_of_code::solution!(19);

use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    
    let patterns: Vec<&str> = lines.next()?
        .split(", ")
        .collect();
    
    lines.next();
    
    let count = lines
        .filter(|design| count_arrangements(design, &patterns) > 0)
        .count();
    
    Some(count as u64)
}

fn count_arrangements(design: &str, patterns: &[&str]) -> u64 {
    // Pre-allocate with estimated capacity
    let mut memo = HashMap::with_capacity(design.len());
    count_arrangements_memo(design, 0, patterns, &mut memo)
}

fn count_arrangements_memo(design: &str, start_idx: usize, patterns: &[&str], memo: &mut HashMap<usize, u64>) -> u64 {
    // Check if we've already solved this subproblem
    if let Some(&count) = memo.get(&start_idx) {
        return count;
    }

    // Base case: reached the end
    if start_idx == design.len() {
        return 1;
    }
    
    // Try each pattern at the current position
    let mut total = 0;
    let remaining = &design[start_idx..];
    for pattern in patterns {
        if remaining.starts_with(pattern) {
            total += count_arrangements_memo(design, start_idx + pattern.len(), patterns, memo);
        }
    }
    
    memo.insert(start_idx, total);
    total
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    
    let patterns: Vec<&str> = lines.next()?
        .split(", ")
        .collect();
    
    lines.next();
    
    let sum: u64 = lines
        .map(|design| count_arrangements(design, &patterns))
        .sum();
    
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
