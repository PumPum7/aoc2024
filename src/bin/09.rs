advent_of_code::solution!(9);

pub fn parse(input: &str) -> Vec<i32> {
    let mut result = Vec::with_capacity(input.len());
    input.bytes().enumerate().for_each(|(i, b)| {
        let len = (b - b'0') as usize;
        let value = if i & 1 == 0 { i as i32 >> 1 } else { -1 };
        result.extend(std::iter::repeat(value).take(len));
    });
    result
}

fn move_files_to_right(blocks: &[i32]) -> Vec<i32> {
    let mut result = blocks.to_vec();
    let mut left = 0;
    let mut right = result.len() - 1;

    while left < right {
        while left < right && result[left] != -1 {
            left += 1;
        }
        while left < right && result[right] == -1 {
            right -= 1;
        }
        if left < right {
            result.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
    result
}

#[inline]
fn checksum(blocks: &[i32]) -> i64 {
    blocks
        .iter()
        .enumerate()
        .filter(|(_, &id)| id != -1)
        .map(|(pos, &id)| pos as i64 * id as i64)
        .sum()
}

pub fn part_one(input: &str) -> Option<i64> {
    let blocks = parse(input);
    Some(checksum(&move_files_to_right(&blocks)))
}

pub fn part_two(input: &str) -> Option<i64> {
    let blocks = parse(input);
    Some(checksum(&move_files_to_earliest_gaps(&blocks)))
}

fn move_files_to_earliest_gaps(blocks: &[i32]) -> Vec<i32> {
    let mut result = blocks.to_vec();
    let len = blocks.len();

    let mut file_counts = vec![0; len];
    let mut max_file_id = 0;
    let mut file_positions = vec![Vec::new(); len];

    // Single pass to gather all necessary information
    for (pos, &block) in blocks.iter().enumerate() {
        if block >= 0 {
            file_counts[block as usize] += 1;
            file_positions[block as usize].push(pos);
            max_file_id = max_file_id.max(block);
        }
    }

    // Process files from largest to smallest
    for file_id in (0..=max_file_id).rev() {
        let file_size = file_counts[file_id as usize];
        if file_size == 0 {
            continue;
        }

        let positions = &file_positions[file_id as usize];
        if positions.is_empty() || positions[0] == 0 {
            continue;
        }

        let mut gap_start = None;
        let mut current_gap_size = 0;
        let mut pos = 0;

        // Find first suitable gap
        while pos < positions[0] {
            if result[pos] == -1 {
                if gap_start.is_none() {
                    gap_start = Some(pos);
                }
                current_gap_size += 1;
                if current_gap_size >= file_size {
                    break;
                }
            } else {
                gap_start = None;
                current_gap_size = 0;
            }
            pos += 1;
        }

        // Move file if suitable gap found
        if let Some(start) = gap_start {
            if current_gap_size >= file_size {
                // Batch update the positions
                for (offset, &src_pos) in positions.iter().enumerate().take(file_size) {
                    result[start + offset] = file_id;
                    result[src_pos] = -1;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
