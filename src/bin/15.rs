advent_of_code::solution!(15);

fn is_movable_vertical(grid: &[u8], w: usize, b: (usize, usize), y: usize, dy: isize) -> bool {
    let ny = y.checked_add_signed(dy).unwrap();

    // Check for walls
    if grid[ny * w + b.0] == b'#' || grid[ny * w + b.1] == b'#' {
        return false;
    }

    // Check for small boxes
    for x in [b.0, b.1] {
        if grid[ny * w + x] == b'O' && !is_movable_vertical(grid, w, (x, x), ny, dy) {
            return false;
        }
    }

    // Check for large box parts
    let checks = [
        (b.0, b'[', (b.0, b.0 + 1)),
        (b.0, b']', (b.0 - 1, b.0)),
        (b.1, b'[', (b.1, b.1 + 1)),
    ];

    for (x, c, box_coords) in checks {
        if grid[ny * w + x] == c && !is_movable_vertical(grid, w, box_coords, ny, dy) {
            return false;
        }
    }

    true
}

fn move_vertical(grid: &mut [u8], w: usize, b: (usize, usize), y: usize, dy: isize) {
    let ny = y.checked_add_signed(dy).unwrap();

    // Move any small boxes that are in the way
    for x in [b.0, b.1] {
        if grid[ny * w + x] == b'O' {
            move_vertical(grid, w, (x, x), ny, dy);
        }
    }

    // Move any large boxes that are in the way
    let box_checks = [
        (b.0, b'[', (b.0, b.0 + 1)), // Large box directly in way
        (b.0, b']', (b.0 - 1, b.0)), // Large box to the left
        (b.1, b'[', (b.1, b.1 + 1)), // Large box to the right
    ];

    for (x, c, box_coords) in box_checks {
        if grid[ny * w + x] == c {
            move_vertical(grid, w, box_coords, ny, dy);
        }
    }

    // Move the current box/robot
    grid[ny * w + b.0] = grid[y * w + b.0];
    grid[ny * w + b.1] = grid[y * w + b.1];

    // Clear previous position
    grid[y * w + b.0] = b'.';
    grid[y * w + b.1] = b'.';
}

fn move_horizontal(grid: &mut [u8], w: usize, pos: (usize, usize), dx: isize) {
    let sx = pos.0.checked_add_signed(dx).unwrap();
    let mut x = sx;
    while grid[pos.1 * w + x] == b'O' || grid[pos.1 * w + x] == b'[' || grid[pos.1 * w + x] == b']'
    {
        x = x.checked_add_signed(dx).unwrap();
    }
    if x != sx && grid[pos.1 * w + x] == b'.' {
        while x != sx {
            let mx = x.checked_add_signed(-(dx - 1) / 2).unwrap();
            let (l, r) = grid.split_at_mut(pos.1 * w + mx);
            std::mem::swap(&mut l[l.len() - 1], &mut r[0]);
            x = x.checked_add_signed(-dx).unwrap();
        }
    }
}

fn process_grid(input: &str) -> (Vec<u8>, Vec<u8>, usize, usize, (usize, usize)) {
    let (grid, instructions) = input.split_once("\n\n").expect("Valid input format");
    let width = grid
        .lines()
        .next()
        .expect("Grid has at least one line")
        .len();
    let mut grid: Vec<u8> = grid.lines().flat_map(|l| l.as_bytes()).copied().collect();
    let height = grid.len() / width;

    // Find robot position
    let robot_idx = grid
        .iter()
        .position(|&c| c == b'@')
        .expect("Robot position found");
    let pos = (robot_idx % width, robot_idx / width);
    grid[robot_idx] = b'.';

    let instructions = instructions
        .as_bytes()
        .iter()
        .copied()
        .filter(|&b| b != b'\n')
        .collect();

    (grid, instructions, width, height, pos)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid, instructions, width, height, pos) = process_grid(input);
    let total = run_instructions(pos, instructions, grid, width, height);
    Some(total as u32)
}

fn run_instructions(
    mut pos: (usize, usize),
    instructions: Vec<u8>,
    mut grid: Vec<u8>,
    w: usize,
    h: usize,
) -> usize {
    for &instr in instructions.iter() {
        match instr {
            b'>' => {
                move_horizontal(&mut grid, w, pos, 1);
                if grid[pos.1 * w + pos.0 + 1] == b'.' {
                    pos.0 += 1;
                }
            }

            b'<' => {
                move_horizontal(&mut grid, w, pos, -1);
                if grid[pos.1 * w + pos.0 - 1] == b'.' {
                    pos.0 -= 1;
                }
            }

            b'^' | b'v' => {
                let dy = if instr == b'^' { -1 } else { 1 };
                let new_y = pos.1.checked_add_signed(dy).unwrap();
                let idx = new_y * w + pos.0;

                match grid[idx] {
                    b'O' => {
                        let b = (pos.0, pos.0);
                        if is_movable_vertical(&grid, w, b, new_y, dy) {
                            move_vertical(&mut grid, w, b, new_y, dy);
                        }
                    }
                    b'[' => {
                        let b = (pos.0, pos.0 + 1);
                        if is_movable_vertical(&grid, w, b, new_y, dy) {
                            move_vertical(&mut grid, w, b, new_y, dy);
                        }
                    }
                    b']' => {
                        let b = (pos.0 - 1, pos.0);
                        if is_movable_vertical(&grid, w, b, new_y, dy) {
                            move_vertical(&mut grid, w, b, new_y, dy);
                        }
                    }
                    _ => {}
                }

                if grid[idx] == b'.' {
                    pos.1 = new_y;
                }
            }

            _ => panic!("Unknown instruction: {}", instr),
        }
    }

    let mut total = 0;
    for y in 0..h {
        let row_offset = y * w;
        for x in 0..w {
            match grid[row_offset + x] {
                b'O' | b'[' => total += 100 * y + x,
                _ => {}
            }
        }
    }

    total
}

pub fn part_two(input: &str) -> Option<u32> {
    let (grid, instructions) = input.split_once("\n\n").unwrap();

    // Get dimensions and parse grid in one pass
    let mut width = 0;
    let mut height = 0;
    let mut grid_bytes = Vec::with_capacity(grid.len());
    for line in grid.lines() {
        if width == 0 {
            width = line.len();
        }
        height += 1;
        grid_bytes.extend_from_slice(line.as_bytes());
    }

    // Make grid twice as wide
    width *= 2;
    let mut wider_grid = Vec::with_capacity(width * height);
    wider_grid.resize(width * height, b'.');

    let mut robot_pos = None;
    for (i, &c) in grid_bytes.iter().enumerate() {
        let wide_i = i * 2;
        match c {
            b'#' | b'.' => {
                wider_grid[wide_i] = c;
                wider_grid[wide_i + 1] = c;
            }
            b'O' => {
                wider_grid[wide_i] = b'[';
                wider_grid[wide_i + 1] = b']';
            }
            b'@' => {
                wider_grid[wide_i] = b'@';
                wider_grid[wide_i + 1] = b'.';
                robot_pos = Some((wide_i % width, wide_i / width));
            }
            _ => panic!("Unknown character in grid: {}", c),
        }
    }

    // Parse instructions in one pass
    let instructions: Vec<u8> = instructions.bytes().filter(|&b| b != b'\n').collect();

    // Get robot position
    let pos = robot_pos.unwrap();
    wider_grid[pos.1 * width + pos.0] = b'.';

    let total = run_instructions(pos, instructions, wider_grid, width, height);
    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
