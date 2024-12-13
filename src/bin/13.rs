advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<i64> {
    let mut total_tokens = 0;

    // Process each machine
    for machine in input.trim().split("\n\n") {
        let lines: Vec<&str> = machine.lines().collect();
        
        // Parse button movements
        let (ax, ay) = parse_button(lines[0], "Button A: X+", ", Y+");
        let (bx, by) = parse_button(lines[1], "Button B: X+", ", Y+");
        
        // Parse prize location
        let (target_x, target_y) = parse_prize(lines[2], "Prize: X=", ", Y=");

        // Use the same solver as part 2
        if let Some(tokens) = find_solution_v2(
            ax as i64, ay as i64, 
            bx as i64, by as i64, 
            target_x as i64, target_y as i64
        ) {
            total_tokens += tokens;
        }
    }

    Some(total_tokens)
}

fn parse_button(line: &str, x_prefix: &str, y_prefix: &str) -> (i32, i32) {
    let x = line[x_prefix.len()..].split(y_prefix).next().unwrap().parse::<i32>().unwrap();
    let y = line[line.find(y_prefix).unwrap() + y_prefix.len()..].parse::<i32>().unwrap();
    (x, y)
}

fn parse_prize(line: &str, x_prefix: &str, y_prefix: &str) -> (i32, i32) {
    let x = line[x_prefix.len()..].split(y_prefix).next().unwrap().parse::<i32>().unwrap();
    let y = line[line.find(y_prefix).unwrap() + y_prefix.len()..].parse::<i32>().unwrap();
    (x, y)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut total_tokens: i64 = 0;

    // Process each machine
    for machine in input.trim().split("\n\n") {
        let lines: Vec<&str> = machine.lines().collect();
        
        // Parse button movements (same as part 1)
        let (ax, ay) = parse_button(lines[0], "Button A: X+", ", Y+");
        let (bx, by) = parse_button(lines[1], "Button B: X+", ", Y+");
        
        // Parse prize location and add the offset
        let (base_x, base_y) = parse_prize(lines[2], "Prize: X=", ", Y=");
        let target_x = (base_x as i64) + 10_000_000_000_000_i64;
        let target_y = (base_y as i64) + 10_000_000_000_000_i64;

        // Try to find a solution using extended Euclidean algorithm
        if let Some(tokens) = find_solution_v2(ax as i64, ay as i64, bx as i64, by as i64, target_x, target_y) {
            total_tokens += tokens;
        }
    }

    Some(total_tokens)
}

// Extended Euclidean Algorithm to find GCD and Bézout's identity coefficients
fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (gcd, x1, y1) = extended_gcd(b, a % b);
        (gcd, y1, x1 - (a / b) * y1)
    }
}

// Find a particular solution to the Diophantine equation
fn find_particular_solution(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    let (gcd, x0, y0) = extended_gcd(a, b);
    if c % gcd != 0 {
        return None;
    }
    let factor = c / gcd;
    Some((x0 * factor, y0 * factor))
}

fn find_solution_v2(ax: i64, ay: i64, bx: i64, by: i64, target_x: i64, target_y: i64) -> Option<i64> {
    // Solve system of equations:
    // ax * A + bx * B = target_x
    // ay * A + by * B = target_y
    
    // Multiply first equation by by and second by bx
    // (ax*by)A + (bx*by)B = target_x * by
    // (ay*bx)A + (by*bx)B = target_y * bx
    
    // Subtract to eliminate B
    // (ax*by - ay*bx)A = target_x * by - target_y * bx
    
    let coeff_a = ax * by - ay * bx;
    let target = target_x * by - target_y * bx;
    
    if let Some((a, _)) = find_particular_solution(coeff_a, 0, target) {
        // Substitute back to find B
        let b = (target_x - ax * a) / bx;
        
        // Check if solution is valid (positive integers)
        if a >= 0 && b >= 0 && ax * a + bx * b == target_x && ay * a + by * b == target_y {
            return Some(3 * a + b);
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
