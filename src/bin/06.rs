advent_of_code::solution!(6);

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn get_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn get_start_pos(grid: &Vec<Vec<u8>>) -> Option<(i32, i32)> {
    for (i, row) in grid.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == b'^' {
                return Some((i as i32, j as i32));
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // Find starting position
    let mut pos = get_start_pos(&grid).unwrap();
    
    let mut dir = Direction::Up;
    let mut visited = std::collections::HashSet::new();
    visited.insert(pos);

    loop {
        let (dx, dy) = dir.get_delta();
        let new_pos = (pos.0 + dx, pos.1 + dy);
        
        // Check if out of bounds
        if new_pos.0 < 0 || new_pos.0 >= rows || new_pos.1 < 0 || new_pos.1 >= cols {
            break;
        }
        
        // Check if obstacle ahead
        if grid[new_pos.0 as usize][new_pos.1 as usize] == b'#' {
            dir = dir.turn_right();
        } else {
            pos = new_pos;
            visited.insert(pos);
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    // Find starting position
    let start_pos = get_start_pos(&grid).unwrap();

    let mut loop_count = 0;
    let mut visited = std::collections::HashSet::new();
    let mut path = Vec::with_capacity((rows * cols) as usize);

    // Pre-allocate these to avoid repeated allocations
    visited.reserve((rows * cols * 4) as usize);

    // Try adding an obstruction at each empty position
    for i in 0..rows {
        for j in 0..cols {
            if grid[i as usize][j as usize] != b'.' {
                continue;
            }

            // Add temporary obstruction
            grid[i as usize][j as usize] = b'#';

            // Reset tracking structures
            visited.clear();
            path.clear();

            let mut pos = start_pos;
            let mut dir = Direction::Up;

            loop {
                let state = (pos, dir);
                if visited.contains(&state) {
                    loop_count += 1;
                    break;
                }
                
                if path.len() > (rows * cols) as usize {
                    break;
                }

                visited.insert(state);
                path.push(state);

                let (dx, dy) = dir.get_delta();
                let new_pos = (pos.0 + dx, pos.1 + dy);
                
                // Check if out of bounds
                if new_pos.0 < 0 || new_pos.0 >= rows || new_pos.1 < 0 || new_pos.1 >= cols {
                    break;
                }
                
                // Check if obstacle ahead
                if grid[new_pos.0 as usize][new_pos.1 as usize] == b'#' {
                    dir = dir.turn_right();
                } else {
                    pos = new_pos;
                }
            }

            // Remove the temporary obstruction
            grid[i as usize][j as usize] = b'.';
        }
    }

    Some(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
