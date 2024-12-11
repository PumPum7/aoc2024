use std::collections::HashMap;

advent_of_code::solution!(11);

fn apply_rules(num: u64) -> Vec<u64> {
    match num {
        0 => vec![1],
        n => {
            let s = n.to_string();
            if s.len() % 2 == 0 {
                let mid = s.len() / 2;
                let left = s[..mid].trim_start_matches('0').parse().unwrap_or(0);
                let right = s[mid..].trim_start_matches('0').parse().unwrap_or(0);
                vec![left, right]
            } else {
                vec![n * 2024]
            }
        }
    }
}

fn simulate_blinks(stones: &[u64], blinks: usize) -> usize {
    let mut stone_counts: HashMap<u64, usize> = stones.iter().map(|&n| (n, 1)).collect();

    for _ in 0..blinks {
        let mut new_counts = HashMap::with_capacity(stone_counts.len() * 2);
        for (stone, count) in stone_counts {
            for new_stone in apply_rules(stone) {
                *new_counts.entry(new_stone).or_insert(0) += count;
            }
        }
        stone_counts = new_counts;
    }

    stone_counts.values().sum()
}

pub fn part_one(input: &str) -> Option<usize> {
    let stones: Vec<u64> = input.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    Some(simulate_blinks(&stones, 25))
}

pub fn part_two(input: &str) -> Option<usize> {
    let stones: Vec<u64> = input.split_whitespace().filter_map(|s| s.parse().ok()).collect();
    Some(simulate_blinks(&stones, 75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
