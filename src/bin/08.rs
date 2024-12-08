advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

fn get_positions(grid: &[Vec<char>]) -> HashMap<char, Vec<(i32, i32)>> {
    let mut positions: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c != '.' {
                positions.entry(c).or_default().push((x as i32, y as i32));
            }
        }
    }
    positions
}

fn find_antinodes(grid: &[Vec<char>], max_distance: Option<i32>) -> HashSet<(i32, i32)> {
    let positions = get_positions(grid);
    let mut antinodes = HashSet::new();
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;

    for antennas in positions.values() {
        for (i, &pos1) in antennas.iter().enumerate() {
            for &pos2 in antennas[i + 1..].iter() {
                let dx = pos2.0 - pos1.0;
                let dy = pos2.1 - pos1.1;
                
                if max_distance == Some(2) {
                    // Special case for n=2 (part one)
                    let points = [
                        (pos1.0 + dx * 2, pos1.1 + dy * 2),
                        (pos2.0 - dx * 2, pos2.1 - dy * 2),
                    ];
                    
                    antinodes.extend(points.iter()
                        .filter(|&&(x, y)| x >= 0 && y >= 0 && x < width && y < height));
                    continue;
                }

                let max_n = max_distance.unwrap_or(i32::MAX);
                
                let mut n = 1;
                while n <= max_n {
                    let points = [
                        (pos1.0 + dx * n, pos1.1 + dy * n),
                        (pos2.0 - dx * n, pos2.1 - dy * n),
                    ];

                    let valid_points = points.iter()
                        .filter(|&&(x, y)| x >= 0 && y >= 0 && x < width && y < height)
                        .count();

                    if valid_points == 0 {
                        break;
                    }

                    antinodes.extend(points.iter()
                        .filter(|&&(x, y)| x >= 0 && y >= 0 && x < width && y < height));
                    
                    n += 1;
                }
            }
        }
    }
    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    
    let antinodes = find_antinodes(&grid, Some(2));

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    Some(find_antinodes(&grid, None).len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
