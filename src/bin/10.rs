advent_of_code::solution!(10);

type Grid = Vec<Vec<u8>>;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let total_score: u32 = (0..rows)
        .into_iter()
        .map(|i| {
            (0..cols)
                .filter(|&j| grid[i][j] == 0)
                .map(|j| count_reachable_nines(&grid, i, j, rows, cols))
                .sum::<u32>()
        })
        .sum();

    Some(total_score)
}

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn count_reachable_nines(
    grid: &Grid,
    start_i: usize,
    start_j: usize,
    rows: usize,
    cols: usize,
) -> u32 {
    let mut visited = vec![false; rows * cols];
    let mut count = 0;
    let mut queue = std::collections::VecDeque::with_capacity(8); // Most paths are short

    let get_index = |i: usize, j: usize| i * cols + j;
    visited[get_index(start_i, start_j)] = true;
    queue.push_back((start_i, start_j, 0u8));

    // Pre-calculate direction offsets
    let directions = [(0_isize, 1_isize), (1, 0), (0, -1), (-1, 0)];

    while let Some((i, j, height)) = queue.pop_front() {
        if grid[i][j] == 9 {
            count += 1;
        }

        for &(di, dj) in &directions {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            // Bounds check in one go
            if ni >= 0 && ni < rows as isize && nj >= 0 && nj < cols as isize {
                let ni = ni as usize;
                let nj = nj as usize;
                let idx = get_index(ni, nj);

                if !visited[idx] && grid[ni][nj] == height + 1 {
                    visited[idx] = true;
                    queue.push_back((ni, nj, height + 1));
                }
            }
        }
    }

    count
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let rows = grid.len();
    let cols = grid[0].len();

    let total_rating: u32 = (0..rows)
        .into_iter()
        .map(|i| {
            (0..cols)
                .filter(|&j| grid[i][j] == 0)
                .map(|j| count_distinct_paths(&grid, i, j, rows, cols))
                .sum::<u32>()
        })
        .sum();

    Some(total_rating)
}

fn count_distinct_paths(
    grid: &Grid,
    start_i: usize,
    start_j: usize,
    rows: usize,
    cols: usize,
) -> u32 {
    let mut path_count = 0;
    let mut current_path = Vec::with_capacity(rows * cols);

    fn dfs(
        i: usize,
        j: usize,
        grid: &Grid,
        path_count: &mut u32,
        current_path: &mut Vec<(usize, usize)>,
        height: u8,
        rows: usize,
        cols: usize,
    ) {
        current_path.push((i, j));

        if grid[i][j] == 9 {
            *path_count += 1;
            current_path.pop();
            return;
        }

        // Optimized direction checking without i32 conversions
        if i > 0 && !current_path.contains(&(i - 1, j)) && grid[i - 1][j] == height + 1 {
            dfs(
                i - 1,
                j,
                grid,
                path_count,
                current_path,
                height + 1,
                rows,
                cols,
            );
        }
        if j > 0 && !current_path.contains(&(i, j - 1)) && grid[i][j - 1] == height + 1 {
            dfs(
                i,
                j - 1,
                grid,
                path_count,
                current_path,
                height + 1,
                rows,
                cols,
            );
        }
        if i + 1 < rows && !current_path.contains(&(i + 1, j)) && grid[i + 1][j] == height + 1 {
            dfs(
                i + 1,
                j,
                grid,
                path_count,
                current_path,
                height + 1,
                rows,
                cols,
            );
        }
        if j + 1 < cols && !current_path.contains(&(i, j + 1)) && grid[i][j + 1] == height + 1 {
            dfs(
                i,
                j + 1,
                grid,
                path_count,
                current_path,
                height + 1,
                rows,
                cols,
            );
        }

        current_path.pop();
    }

    dfs(
        start_i,
        start_j,
        grid,
        &mut path_count,
        &mut current_path,
        0,
        rows,
        cols,
    );
    path_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
