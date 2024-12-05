advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    const TARGET: [u8; 4] = [b'X', b'M', b'A', b'S'];

    // Check all possible starting positions
    for row in 0..height {
        for col in 0..width {
            // Only check if starting with 'X'
            if grid[row][col] != b'X' {
                continue;
            }

            // Check all 8 directions from each position
            const DIRECTIONS: [(i32, i32); 8] = [
                (0, 1),
                (1, 0),
                (1, 1),
                (-1, 1),
                (0, -1),
                (-1, 0),
                (-1, -1),
                (1, -1),
            ];

            for &(dy, dx) in &DIRECTIONS {
                let end_row = row as i32 + dy * 3;
                let end_col = col as i32 + dx * 3;

                if end_row >= 0 && end_row < height as i32 && end_col >= 0 && end_col < width as i32
                {
                    // Check remaining characters inline
                    let mut matches = true;
                    for i in 1..4 {
                        let r = (row as i32 + dy * i as i32) as usize;
                        let c = (col as i32 + dx * i as i32) as usize;
                        if grid[r][c] != TARGET[i] {
                            matches = false;
                            break;
                        }
                    }
                    if matches {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    // Check all possible starting positions
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            // Only check positions that have 'A' in center
            if grid[row][col] != b'A' {
                continue;
            }

            // Check all four possible patterns around center 'A':
            // M.S/M.S, S.M/S.M, M.M/S.S, S.S/M.M
            let (tl, tr, bl, br) = (
                grid[row - 1][col - 1],
                grid[row - 1][col + 1],
                grid[row + 1][col - 1],
                grid[row + 1][col + 1],
            );

            // Check if any pattern matches using bitwise operations
            let is_m = |b| b == b'M';
            let is_s = |b| b == b'S';

            if (is_m(tl) && is_s(tr) && is_m(bl) && is_s(br)) || // M.S/M.S
               (is_s(tl) && is_m(tr) && is_s(bl) && is_m(br)) || // S.M/S.M
               (is_m(tl) && is_m(tr) && is_s(bl) && is_s(br)) || // M.M/S.S
               (is_s(tl) && is_s(tr) && is_m(bl) && is_m(br))
            // S.S/M.M
            {
                count += 1;
            }
        }
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
