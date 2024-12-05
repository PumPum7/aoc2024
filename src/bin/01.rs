advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_column, mut right_column): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            line.split_once("   ").and_then(|(left, right)| {
                Some((left.parse::<u32>().ok()?, right.parse::<u32>().ok()?))
            })
        })
        .unzip();

    left_column.sort_unstable();
    right_column.sort_unstable();

    Some(
        left_column
            .iter()
            .zip(right_column.iter())
            .map(|(a, b)| a.abs_diff(*b))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_column, right_column): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            line.split_once("   ")
                .map(|(left, right)| (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap()))
        })
        .unzip();

    // Create a frequency map for right column numbers
    let mut right_freq = std::collections::HashMap::new();
    for &num in &right_column {
        *right_freq.entry(num).or_insert(0) += 1;
    }

    let similarity_score: u32 = left_column
        .iter()
        .map(|&left_num| {
            let occurrences = right_freq.get(&left_num).copied().unwrap_or(0);
            left_num * occurrences
        })
        .sum();

    Some(similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
