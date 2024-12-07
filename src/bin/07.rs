advent_of_code::solution!(7);

fn check(target: u64, operands: &[u64], current_index: usize, part1: bool) -> bool {
    // Base case: if we're at the first number, it must match exactly
    if current_index == 0 {
        return target == operands[0];
    }

    let current_operand = operands[current_index];

    // Try addition first as it's the cheapest operation
    if check(target - current_operand, operands, current_index - 1, part1) {
        return true;
    }

    // Try multiplication if target is divisible
    if target % current_operand == 0
        && check(target / current_operand, operands, current_index - 1, part1)
    {
        return true;
    }

    // Part 2 only: Try concatenation
    if !part1 {
        let mut concat_value = current_operand;
        let mut target_value = target;

        // Extract digits from target
        while target_value > 0 && concat_value > 0 {
            if target_value % 10 != concat_value % 10 {
                break;
            }
            target_value /= 10;
            concat_value /= 10;
        }

        if concat_value == 0 && target_value > 0 {
            if check(target_value, operands, current_index - 1, part1) {
                return true;
            }
        }
    }

    false
}

fn process_input(input: &str, part1: bool) -> Option<u64> {
    input
        .lines()
        .map(|l| {
            let (result, numbers) = l.split_once(": ").unwrap();
            let result = result.parse::<u64>().unwrap();
            let numbers: Vec<u64> = numbers
                .split_whitespace()
                .map(|o| o.parse::<u64>().unwrap())
                .collect();

            if check(result, &numbers, numbers.len() - 1, part1) {
                result
            } else {
                0
            }
        })
        .sum::<u64>()
        .into()
}

pub fn part_one(input: &str) -> Option<u64> {
    process_input(input, true)
}

pub fn part_two(input: &str) -> Option<u64> {
    process_input(input, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
