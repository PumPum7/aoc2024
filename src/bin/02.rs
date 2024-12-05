advent_of_code::solution!(2);

fn parse_levels(line: &str) -> Vec<u32> {
    let mut nums = Vec::with_capacity(8);
    line.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .for_each(|n| nums.push(n));
    nums
}

fn is_valid_sequence(nums: &[u32]) -> bool {
    let len = nums.len();

    // Use first difference to avoid Option
    let first_diff = nums[1] as i32 - nums[0] as i32;
    if first_diff.abs() < 1 || first_diff.abs() > 3 {
        return false;
    }
    let should_increase = first_diff > 0;

    // Start from index 1 to avoid recomputing first difference
    for i in 1..len - 1 {
        let diff = nums[i + 1] as i32 - nums[i] as i32;
        if diff.abs() < 1 || diff.abs() > 3 || (diff > 0) != should_increase {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .map(parse_levels)
        .filter(|levels| is_valid_sequence(levels))
        .count();

    Some(safe_reports as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let safe_reports = input
        .lines()
        .map(parse_levels)
        .filter(|levels| {
            // First check without removing any numbers
            if is_valid_sequence(levels) {
                return true;
            }

            // Try removing one number at a time
            (0..levels.len()).any(|i| {
                let mut modified = levels.to_vec();
                modified.remove(i);
                is_valid_sequence(&modified)
            })
        })
        .count();

    Some(safe_reports as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
