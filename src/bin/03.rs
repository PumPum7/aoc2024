advent_of_code::solution!(3);

use regex::Regex;

// valid format: mul(2,3)
pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    for line in input.lines() {
        for cap in re.captures_iter(line) {
            let a = cap[1].parse::<u32>().unwrap();
            let b = cap[2].parse::<u32>().unwrap();
            total += a * b;
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total = 0;
    let mut enabled = true;
    for line in input.lines() {
        let parts = valid_parts(line, &mut enabled);
        for cap in re.captures_iter(&parts) {
            let a = cap[1].parse::<u32>().unwrap();
            let b = cap[2].parse::<u32>().unwrap();
            total += a * b;
        }
    }
    Some(total)
}

pub fn valid_parts(line: &str, enabled: &mut bool) -> String {
    line.split("do")
        .enumerate()
        .fold(String::new(), |mut acc, (i, part)| {
            if i == 0 {
                if *enabled {
                    acc.push_str(part);
                }
            } else if part.starts_with("n't()") {
                *enabled = false;
            } else if part.starts_with("()") {
                *enabled = true;
                acc.push_str(part);
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
