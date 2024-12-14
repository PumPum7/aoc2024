advent_of_code::solution!(14);

type Point = (i32, i32);
type Robot = (Point, Point); // (position, velocity)

fn parse_robot(line: &str) -> Robot {
    let (pos_str, vel_str) = line.split_once(' ').unwrap();
    let pos_str = &pos_str[2..]; // Skip "p="
    let vel_str = &vel_str[2..]; // Skip "v="
    
    let (x, y) = pos_str.split_once(',').unwrap();
    let (vx, vy) = vel_str.split_once(',').unwrap();

    (
        (x.parse().unwrap(), y.parse().unwrap()),
        (vx.parse().unwrap(), vy.parse().unwrap())
    )
}

fn simulate_step(robots: &mut Vec<Robot>, width: i32, height: i32) {
    // Process robots in chunks for better cache utilization
    for chunk in robots.chunks_mut(32) {
        for robot in chunk {
            let mut new_x = robot.0.0 + robot.1.0;
            let mut new_y = robot.0.1 + robot.1.1;
            
            // Handle wrapping with branches instead of rem_euclid
            if new_x >= width {
                new_x -= width;
            } else if new_x < 0 {
                new_x += width;
            }
            
            if new_y >= height {
                new_y -= height;
            } else if new_y < 0 {
                new_y += height;
            }
            
            robot.0.0 = new_x;
            robot.0.1 = new_y;
        }
    }
}

fn count_robots_in_quadrants(robots: &[Robot], width: i32, height: i32) -> (usize, usize, usize, usize) {
    let mid_x = width / 2;
    let mid_y = height / 2;
    let mut counts = (0, 0, 0, 0);

    for robot in robots {
        if robot.0.0 != mid_x && robot.0.1 != mid_y {
            match (robot.0.0 < mid_x, robot.0.1 < mid_y) {
                (true, true) => counts.0 += 1,
                (false, true) => counts.1 += 1,
                (true, false) => counts.2 += 1,
                (false, false) => counts.3 += 1,
            }
        }
    }
    
    counts
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input.lines().map(parse_robot).collect();
    let width = 101;
    let height = 103;

    // Simulate for 100 steps
    for _ in 0..100 {
        simulate_step(&mut robots, width, height);
    }

    let (q1, q2, q3, q4) = count_robots_in_quadrants(&robots, width, height);
    Some((q1 * q2 * q3 * q4) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<Robot> = input.lines().map(parse_robot).collect();
    let width = 101;
    let height = 103;
    
    for step in 1.. {
        simulate_step(&mut robots, width, height);
        
        // Check for aligned robots (16 or more in a row)
        let mut positions = vec![0; (width * height) as usize];
        for robot in &robots {
            let idx = (robot.0.1 * width + robot.0.0) as usize;
            positions[idx] += 1;
        }
        
        for y in 0..height {
            for x in 0..width-16 {
                let mut aligned = true;
                for i in 0..16 {
                    if positions[(y * width + x + i) as usize] == 0 {
                        aligned = false;
                        break;
                    }
                }
                if aligned {
                    return Some(step);
                }
            }
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
        assert_eq!(result, Some(21));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
