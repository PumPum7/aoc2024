use std::collections::HashSet;

advent_of_code::solution!(12);

struct Grid {
    data: Vec<char>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut width = 0;
        let data: Vec<char> = input
            .lines()
            .flat_map(|line| {
                width = line.len();
                line.chars()
            })
            .collect();
        let height = data.len() / width;
        Self { data, width, height }
    }

    #[inline]
    fn get(&self, row: usize, col: usize) -> char {
        self.data[row * self.width + col]
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let mut visited = vec![false; grid.width * grid.height];
    let mut total_price = 0;
    let mut stack = Vec::with_capacity(grid.width * grid.height);

    for i in 0..grid.height {
        for j in 0..grid.width {
            let idx = i * grid.width + j;
            if !visited[idx] {
                let plant = grid.get(i, j);
                let mut area = 0;
                let mut perimeter = 0;

                stack.clear();
                stack.push((i, j));

                while let Some((y, x)) = stack.pop() {
                    let curr_idx = y * grid.width + x;
                    if visited[curr_idx] || grid.get(y, x) != plant {
                        continue;
                    }

                    visited[curr_idx] = true;
                    area += 1;

                    // Check boundaries and neighbors
                    if y > 0 && !visited[(y - 1) * grid.width + x] && grid.get(y - 1, x) == plant {
                        stack.push((y - 1, x));
                    } else if y == 0 || grid.get(y - 1, x) != plant {
                        perimeter += 1;
                    }

                    if y + 1 < grid.height && !visited[(y + 1) * grid.width + x] && grid.get(y + 1, x) == plant {
                        stack.push((y + 1, x));
                    } else if y + 1 == grid.height || grid.get(y + 1, x) != plant {
                        perimeter += 1;
                    }

                    if x > 0 && !visited[y * grid.width + x - 1] && grid.get(y, x - 1) == plant {
                        stack.push((y, x - 1));
                    } else if x == 0 || grid.get(y, x - 1) != plant {
                        perimeter += 1;
                    }

                    if x + 1 < grid.width && !visited[y * grid.width + x + 1] && grid.get(y, x + 1) == plant {
                        stack.push((y, x + 1));
                    } else if x + 1 == grid.width || grid.get(y, x + 1) != plant {
                        perimeter += 1;
                    }
                }

                total_price += area * perimeter;
            }
        }
    }

    Some(total_price)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let mut visited = vec![false; grid.width * grid.height];
    let mut total_price = 0;
    let mut region = HashSet::with_capacity(grid.width * grid.height);
    let mut stack = Vec::with_capacity(grid.width * grid.height);

    for i in 0..grid.height {
        for j in 0..grid.width {
            let idx = i * grid.width + j;
            if !visited[idx] {
                region.clear();
                collect_region(&grid, &mut visited, (i, j), &mut stack, &mut region);
                let area = region.len() as u32;
                let discount = calculate_discount(&region, (grid.height, grid.width));
                total_price += area * discount;
            }
        }
    }

    Some(total_price)
}

#[inline]
fn collect_region(
    grid: &Grid,
    visited: &mut [bool],
    start: (usize, usize),
    stack: &mut Vec<(usize, usize)>,
    region: &mut HashSet<(usize, usize)>,
) {
    stack.clear();
    stack.push(start);
    let plant = grid.get(start.0, start.1);
    region.insert(start);
    visited[start.0 * grid.width + start.1] = true;

    while let Some((y, x)) = stack.pop() {
        // Pre-calculate neighbor coordinates
        let neighbors = [
            (y.wrapping_sub(1), x),
            (y + 1, x),
            (y, x.wrapping_sub(1)),
            (y, x + 1),
        ];

        for &(ny, nx) in &neighbors {
            if ny < grid.height && nx < grid.width {
                let idx = ny * grid.width + nx;
                if !visited[idx] && grid.get(ny, nx) == plant {
                    visited[idx] = true;
                    region.insert((ny, nx));
                    stack.push((ny, nx));
                }
            }
        }
    }
}

#[inline]
fn calculate_discount(region: &HashSet<(usize, usize)>, limit: (usize, usize)) -> u32 {
    let mut total = 0;
    for &(y, x) in region {
        let mut count = 0;
        
        // Top edge
        if y == 0 || !region.contains(&(y - 1, x)) {
            if x == 0 || !region.contains(&(y, x - 1)) || (y > 0 && region.contains(&(y - 1, x - 1))) {
                count += 1;
            }
        }
        
        // Bottom edge
        if y == limit.0 - 1 || !region.contains(&(y + 1, x)) {
            if x == 0 || !region.contains(&(y, x - 1)) || (y < limit.0 - 1 && region.contains(&(y + 1, x - 1))) {
                count += 1;
            }
        }
        
        // Left edge
        if x == 0 || !region.contains(&(y, x - 1)) {
            if y == 0 || !region.contains(&(y - 1, x)) || (x > 0 && region.contains(&(y - 1, x - 1))) {
                count += 1;
            }
        }
        
        // Right edge
        if x == limit.1 - 1 || !region.contains(&(y, x + 1)) {
            if y == 0 || !region.contains(&(y - 1, x)) || (x < limit.1 - 1 && region.contains(&(y - 1, x + 1))) {
                count += 1;
            }
        }
        
        total += count;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
