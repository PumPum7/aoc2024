advent_of_code::solution!(18);

use std::collections::{HashSet, VecDeque};

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

        // Try all four directions
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = x + dx;
            let new_y = y + dy;
            
            // Check bounds and if position is not blocked
            if new_x >= 0 && new_x <= 70 && new_y >= 0 && new_y <= 70 
                && !blocked.contains(&(new_x, new_y))
                && !visited.contains(&(new_x, new_y)) {
                queue.push_back(((new_x, new_y), steps + 1));
                visited.insert((new_x, new_y));
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<String> {
    let coordinates: Vec<(i32, i32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    // Try each coordinate in sequence
    let mut blocked = HashSet::new();
    for &coord in &coordinates {
        blocked.insert(coord);
        
        // Check if path still exists
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        queue.push_back(((0, 0), 0));
        visited.insert((0, 0));
        let mut path_exists = false;

        while let Some(((x, y), _)) = queue.pop_front() {
            if x == 70 && y == 70 {
                path_exists = true;
                break;
            }

            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let new_x = x + dx;
                let new_y = y + dy;
                
                if new_x >= 0 && new_x <= 70 && new_y >= 0 && new_y <= 70 
                    && !blocked.contains(&(new_x, new_y))
                    && !visited.contains(&(new_x, new_y)) {
                    queue.push_back(((new_x, new_y), 0));
                    visited.insert((new_x, new_y));
                }
            }
        }

        if !path_exists {
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".to_string()));
    }
}
