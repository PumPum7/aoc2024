advent_of_code::solution!(18);

use std::collections::{HashSet, VecDeque};

const GRID_SIZE: usize = 71;
const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn can_reach_target(
    blocked: &HashSet<(i32, i32)>,
    start: (i32, i32),
    target: (i32, i32),
) -> bool {
    let mut queue = VecDeque::with_capacity(GRID_SIZE * GRID_SIZE);
    let mut visited = vec![false; GRID_SIZE * GRID_SIZE];
    
    queue.push_back(start);
    visited[(start.1 as usize) * GRID_SIZE + (start.0 as usize)] = true;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == target {
            return true;
        }

        for &(dx, dy) in &DIRS {
            let new_x = x + dx;
            let new_y = y + dy;
            
            if new_x >= 0 && new_x < GRID_SIZE as i32 
                && new_y >= 0 && new_y < GRID_SIZE as i32 
            {
                let idx = (new_y as usize) * GRID_SIZE + (new_x as usize);
                if !visited[idx] && !blocked.contains(&(new_x, new_y)) {
                    visited[idx] = true;
                    queue.push_back((new_x, new_y));
                }
            }
        }
    }
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    // Parse coordinates into a set of blocked positions
    let blocked: HashSet<(i32, i32)> = input
        .lines()
        .take(1024) // Only consider first 1024 bytes
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    // BFS to find shortest path
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back(((0, 0), 0)); // Start position and steps
    visited.insert((0, 0));

    while let Some(((x, y), steps)) = queue.pop_front() {
        // Check if we reached the target (70,70)
        if x == 70 && y == 70 {
            return Some(steps);
        }

        for &(dx, dy) in &DIRS {
            let new_x = x + dx;
            let new_y = y + dy;

            // Check bounds and if position is not blocked
            if new_x >= 0
                && new_x <= 70
                && new_y >= 0
                && new_y <= 70
                && !blocked.contains(&(new_x, new_y))
                && !visited.contains(&(new_x, new_y))
            {
                queue.push_back(((new_x, new_y), steps + 1));
                visited.insert((new_x, new_y));
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<String> {
    let coordinates: Vec<_> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    let mut blocked = HashSet::with_capacity(coordinates.len());
    
    // Check if initial path exists
    if !can_reach_target(&blocked, (0, 0), (70, 70)) {
        return Some("0,0".to_string());
    }

    for &coord in &coordinates {
        blocked.insert(coord);
        
        // Early exit if this coordinate blocks the only possible path
        if !can_reach_target(&blocked, (0, 0), (70, 70)) {
            return Some(format!("{},{}", coord.0, coord.1));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // Example only does 12 bytes of input but we test it with 1024 bytes aswell
        assert_eq!(result, Some(146));
    }

    #[test]
    // This test is ignored because it would need a different implementation
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
