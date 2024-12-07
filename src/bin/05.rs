use std::collections::HashMap;

advent_of_code::solution!(5);

fn parse_rules_and_input(
    input: &str,
) -> Option<(std::collections::HashMap<(u32, u32), bool>, Vec<Vec<u32>>)> {
    let mut sections = input.split("\n\n");
    let rules_section = sections.next()?;
    let updates_section = sections.next()?;

    // Pre-allocate HashMap with estimated capacity
    let mut rules_map = HashMap::with_capacity(rules_section.lines().count());

    // Process rules more efficiently
    for line in rules_section.lines().filter(|l| !l.is_empty()) {
        if let Some((before, after)) = line.split_once('|') {
            if let (Ok(before), Ok(after)) = (before.parse(), after.parse()) {
                rules_map.insert((before, after), true);
            }
        }
    }

    // Parse updates more efficiently
    let updates = updates_section
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            line.split(',')
                .filter_map(|n| n.parse().ok())
                .collect::<Vec<u32>>()
        })
        .collect();

    Some((rules_map, updates))
}

pub fn part_one(input: &str) -> Option<u32> {
    // Parse input into rules and updates sections
    let (rules_map, updates) = parse_rules_and_input(input).unwrap();

    let mut sum = 0;
    'update_loop: for update in updates {
        // Check each pair of pages in the update against rules
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                let page1 = update[i];
                let page2 = update[j];

                // Check if there's a rule violation using hashmap
                if rules_map.contains_key(&(page2, page1)) {
                    continue 'update_loop;
                }
            }
        }

        // If we get here, the update is valid
        sum += update[update.len() / 2];
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_map, mut updates) = parse_rules_and_input(input).unwrap();

    let mut sum = 0;
    for update in &mut updates {
        // Check if this update needs reordering
        let mut needs_reorder = false;
        'outer: for i in 0..update.len() {
            for j in i + 1..update.len() {
                let page1 = update[i];
                let page2 = update[j];
                if rules_map.contains_key(&(page2, page1)) {
                    needs_reorder = true;
                    break 'outer;
                }
            }
            if needs_reorder {
                break;
            }
        }

        if !needs_reorder {
            continue;
        }

        // Use a more efficient sorting algorithm if possible
        let len = update.len();
        if len > 0 {
            quicksort(&mut update[..], &rules_map, 0, len - 1);
        }

        // Add the middle number from the now correctly ordered sequence
        sum += update[len / 2];
    }

    Some(sum)
}

fn quicksort(arr: &mut [u32], rules: &HashMap<(u32, u32), bool>, start: usize, end: usize) {
    if start >= end {
        return;
    }

    // Use insertion sort for small arrays
    if end - start <= 10 {
        for i in (start + 1)..=end {
            let mut j = i;
            while j > start && rules.contains_key(&(arr[j - 1], arr[j])) {
                arr.swap(j - 1, j);
                if j > 0 {
                    j -= 1;
                } else {
                    break;
                }
            }
        }
        return;
    }

    let pivot = arr[end];
    let mut i = start;

    for j in start..end {
        // Compare using rules - if j should come before pivot
        if rules.contains_key(&(pivot, arr[j])) {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, end);

    if i > 0 {
        quicksort(arr, rules, start, i - 1);
    }
    quicksort(arr, rules, i + 1, end);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
