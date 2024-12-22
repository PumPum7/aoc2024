use std::collections::{HashMap, HashSet};

advent_of_code::solution!(22);

fn next_secret(mut secret: u64) -> u64 {
    let result = secret * 64;
    secret ^= result;
    secret %= 16777216;

    let result = secret / 32;
    secret ^= result;
    secret %= 16777216;

    let result = secret * 2048;
    secret ^= result;
    secret %= 16777216;

    secret
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut secret = line.parse::<u64>().unwrap();
            for _ in 0..2000 {
                secret = next_secret(secret);
            }
            secret
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let initials: Vec<u64> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect();

    let mut pattern_values: HashMap<[i32; 4], Vec<i32>> = HashMap::new();

    for &initial in &initials {
        let mut seen_patterns = HashSet::new();
        let mut secret = initial;
        let mut window = [0i32; 5];

        // Initialize the rolling window
        window[0] = (secret % 10) as i32;
        for i in 1..5 {
            secret = next_secret(secret);
            window[i] = (secret % 10) as i32;
        }

        for _ in 0..2000 {
            let pattern = [
                window[1] - window[0],
                window[2] - window[1],
                window[3] - window[2],
                window[4] - window[3],
            ];
            if !seen_patterns.contains(&pattern) {
                seen_patterns.insert(pattern);
                pattern_values.entry(pattern).or_default().push(window[4]);
            }

            for w in 0..4 {
                window[w] = window[w + 1];
            }
            secret = next_secret(secret);
            window[4] = (secret % 10) as i32;
        }
    }

    pattern_values
        .into_iter()
        .map(|(_, values)| values.into_iter().sum::<i32>() as u32)
        .max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(23));
    }
}
