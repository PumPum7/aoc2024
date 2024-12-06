advent_of_code::solution!(6);

type Point = (i32, i32);
type Grid = Vec<u8>;

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn get_delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }

    const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];
}

fn find_loop(
    grid: &Grid,
    width: usize,
    height: usize,
    pos: (i32, i32),
    mut route: Option<&mut Vec<usize>>,
    seen: &mut [u8],
    marked: &mut [bool],
) -> bool {
    seen.fill(0);
    if route.is_some() {
        marked.fill(false);
    }

    let mut direction = 0;
    let mut current_pos = pos;

    loop {
        let current_idx = (current_pos.1 as usize * width) + current_pos.0 as usize;

        if let Some(ref mut path) = route {
            if !marked[current_idx] {
                marked[current_idx] = true;
                path.push(current_idx);
            }
        }

        let delta = Direction::ALL[direction].get_delta();
        let next_pos = (
            current_pos.0 + delta.0,
            current_pos.1 + delta.1,
        );

        if next_pos.0 < 0 
            || next_pos.1 < 0 
            || next_pos.0 >= width as i32 
            || next_pos.1 >= height as i32 
        {
            return false;
        }

        let next_idx = (next_pos.1 as usize * width) + next_pos.0 as usize;

        // Check if the next position is a wall -> turn 90 degrees
        if grid[next_idx] == b'#' {
            direction = (direction + 1) % 4;
            let direction_flag = 1u8 << direction;
            
            if seen[current_idx] & direction_flag != 0 {
                return true;
            }
            seen[current_idx] |= direction_flag;
        } else {
            current_pos = next_pos;
        }
    }
}

fn data_handler(input: &str) -> Option<(Grid, usize, usize)> {
    let lines: Vec<_> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let grid: Grid = lines.iter().flat_map(|l| l.as_bytes()).copied().collect();
    Some((grid, width, height))
}

fn find_start(grid: &Grid, width: usize) -> Point {
    grid.iter()
        .enumerate()
        .find(|(_, &cell)| cell == b'^')
        .map(|(i, _)| ((i % width) as i32, (i / width) as i32))
        .unwrap_or((0, 0))
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, width, height) = data_handler(input)?;
    let start = find_start(&grid, width);
    
    let mut seen = vec![0u8; grid.len()];
    let mut marked = vec![false; grid.len()];
    let mut route = Vec::with_capacity(grid.len());
    
    find_loop(&grid, width, height, start, Some(&mut route), &mut seen, &mut marked);
    Some(route.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut grid, width, height) = data_handler(input)?;
    let start = find_start(&grid, width);
    
    let mut seen = vec![0u8; grid.len()];
    let mut marked = vec![false; grid.len()];
    let mut route = Vec::with_capacity(grid.len());
    
    find_loop(&grid, width, height, start, Some(&mut route), &mut seen, &mut marked);
    
    route[1..].iter()
        .map(|&i| {
            grid[i] = b'#';
            let has_loop = find_loop(&grid, width, height, start, None, &mut seen, &mut marked);
            grid[i] = b'.';
            u32::from(has_loop)
        })
        .sum::<u32>()
        .into()
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
