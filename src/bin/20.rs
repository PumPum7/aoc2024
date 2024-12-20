use std::collections::VecDeque;

advent_of_code::solution!(20);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn adjacent_points(&self) -> Vec<Point> {
        vec![
            Point::new(self.x + 1, self.y),
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y + 1),
            Point::new(self.x, self.y - 1),
        ]
    }

    fn manhattan_distance(&self, other: &Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

fn parse_map(input: &str) -> (Vec<Vec<char>>, Point, Point) {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut start = Point::default();
    let mut end = Point::default();

    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = Point::new(x as i32, y as i32);
            } else if cell == 'E' {
                end = Point::new(x as i32, y as i32);
            }
        }
    }
    (map, start, end)
}

fn calculate_distances(map: &Vec<Vec<char>>, start: Point) -> Vec<Vec<u32>> {
    let rows = map.len();
    let cols = map[0].len();
    let mut distances = vec![vec![u32::MAX; cols]; rows];
    let mut current_queue = VecDeque::new();
    let mut next_queue = VecDeque::new();

    current_queue.push_back(start);
    let mut current_cost = 0;

    while !current_queue.is_empty() {
        while let Some(pos) = current_queue.pop_front() {
            if pos.y < 0
                || pos.y >= rows as i32
                || pos.x < 0
                || pos.x >= cols as i32
                || distances[pos.y as usize][pos.x as usize] != u32::MAX
            {
                continue;
            }

            distances[pos.y as usize][pos.x as usize] = current_cost;

            for next in pos.adjacent_points() {
                if next.y >= 0
                    && next.y < rows as i32
                    && next.x >= 0
                    && next.x < cols as i32
                    && map[next.y as usize][next.x as usize] != '#'
                    && distances[next.y as usize][next.x as usize] == u32::MAX
                {
                    next_queue.push_back(next);
                }
            }
        }

        std::mem::swap(&mut current_queue, &mut next_queue);
        current_cost += 1;
    }

    distances
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, start, end) = parse_map(input);
    let start_distances = calculate_distances(&map, start);
    let end_distances = calculate_distances(&map, end);

    let orig_distance = end_distances[start.y as usize][start.x as usize];
    let mut count = 0;

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            let mid_point = Point::new(c as i32, r as i32);
            if map[r][c] == '#' || start_distances[r][c] == u32::MAX {
                continue;
            }

            for r_offset in -2..=2 {
                for c_offset in -2..=2 {
                    let end_point = Point::new(mid_point.x + c_offset, mid_point.y + r_offset);
                    if mid_point.manhattan_distance(&end_point) != 2 {
                        continue;
                    }

                    if end_point.y < 0
                        || end_point.y >= map.len() as i32
                        || end_point.x < 0
                        || end_point.x >= map[0].len() as i32
                        || map[end_point.y as usize][end_point.x as usize] == '#'
                        || end_distances[end_point.y as usize][end_point.x as usize] == u32::MAX
                    {
                        continue;
                    }

                    let new_distance = start_distances[r][c]
                        + end_distances[end_point.y as usize][end_point.x as usize]
                        + 2;

                    if (new_distance + 100) <= orig_distance {
                        count += 1;
                    }
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (map, start, end) = parse_map(input);
    let start_distances = calculate_distances(&map, start);
    let end_distances = calculate_distances(&map, end);

    let orig_distance = end_distances[start.y as usize][start.x as usize];
    let mut count = 0;

    for r in 0..map.len() {
        for c in 0..map[0].len() {
            let mid_point = Point::new(c as i32, r as i32);
            if map[r][c] == '#' || start_distances[r][c] == u32::MAX {
                continue;
            }

            // Expanded search range for 20-picosecond cheats
            for r_offset in -20..=20 {
                for c_offset in -20..=20 {
                    let end_point = Point::new(mid_point.x + c_offset, mid_point.y + r_offset);
                    let manhattan_dist = mid_point.manhattan_distance(&end_point);

                    // Check if the cheat distance is within 20 moves
                    if manhattan_dist > 20 {
                        continue;
                    }

                    if end_point.y < 0
                        || end_point.y >= map.len() as i32
                        || end_point.x < 0
                        || end_point.x >= map[0].len() as i32
                        || map[end_point.y as usize][end_point.x as usize] == '#'
                        || end_distances[end_point.y as usize][end_point.x as usize] == u32::MAX
                    {
                        continue;
                    }

                    let new_distance = start_distances[r][c]
                        + end_distances[end_point.y as usize][end_point.x as usize]
                        + manhattan_dist;

                    if (new_distance + 100) <= orig_distance {
                        count += 1;
                    }
                }
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
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }
}
