advent_of_code::solution!(16);

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

#[derive(PartialEq, Eq)]
struct State {
    score: usize,
    x: i32,
    y: i32,
    di: usize,
    path: Option<Vec<(i32, i32)>>, // Only used in part 2
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };

    for y in 0..height {
        for x in 0..width {
            if grid[y as usize][x as usize] == b'S' {
                start = Position { x, y };
            } else if grid[y as usize][x as usize] == b'E' {
                end = Position { x, y };
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(Reverse(State {
        score: 0,
        x: start.x,
        y: start.y,
        di: 0,
        path: None,
    }));

    while let Some(Reverse(State {
        score,
        x,
        y,
        di,
        path: None,
    })) = heap.pop()
    {
        if x == end.x && y == end.y {
            return Some(score as u32);
        }

        let state_key = (x, y, di);
        if !visited.insert(state_key) {
            continue;
        }

        // Move forward
        let new_x = x + DIRS[di].0;
        let new_y = y + DIRS[di].1;
        if new_x >= 0
            && new_x < width
            && new_y >= 0
            && new_y < height
            && grid[new_y as usize][new_x as usize] != b'#'
        {
            heap.push(Reverse(State {
                score: score + 1,
                x: new_x,
                y: new_y,
                di,
                path: None,
            }));
        }

        // Turn left
        let new_di = (di + 3) % 4;
        heap.push(Reverse(State {
            score: score + 1000,
            x,
            y,
            di: new_di,
            path: None,
        }));

        // Turn right
        let new_di = (di + 1) % 4;
        heap.push(Reverse(State {
            score: score + 1000,
            x,
            y,
            di: new_di,
            path: None,
        }));
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let grid: Vec<u8> = lines.iter().flat_map(|l| l.as_bytes()).copied().collect();

    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..height {
        for x in 0..width {
            if grid[y * width + x] == b'S' {
                start = (x as i32, y as i32);
            } else if grid[y * width + x] == b'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    let mut heap = BinaryHeap::new();
    let mut seen = vec![usize::MAX - 1000; width * height * DIRS.len()];
    let mut paths = Vec::new();
    let mut min = usize::MAX;

    heap.push(Reverse(State {
        score: 0,
        x: start.0,
        y: start.1,
        di: 0,
        path: Some(vec![(start.0, start.1)]),
    }));

    while let Some(Reverse(State {
        score,
        x,
        y,
        di: prev_di,
        path,
    })) = heap.pop()
    {
        if x == end.0 && y == end.1 {
            if score > min {
                break;
            }
            paths.push(path);
            min = score;
            continue;
        }

        for (di, (dx, dy)) in DIRS.iter().enumerate() {
            if (prev_di + 2) % DIRS.len() == di {
                continue;
            }

            let nscore = if di == prev_di {
                score + 1
            } else {
                score + 1001
            };
            let nx = x + dx;
            let ny = y + dy;

            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                let gi = ny as usize * width + nx as usize;
                let si = gi * DIRS.len() + di;

                if grid[gi] != b'#' && nscore <= seen[si] + 1000 {
                    seen[si] = nscore;

                    let mut new_path = path.clone();
                    new_path.as_mut()?.push((nx, ny));

                    heap.push(Reverse(State {
                        score: nscore,
                        x: nx,
                        y: ny,
                        di,
                        path: new_path,
                    }));
                }
            }
        }
    }

    let mut visited = HashSet::new();
    for path in paths {
        if let Some(path) = path {
            for pos in path {
                visited.insert(pos);
            }
        }
    }

    Some(visited.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
