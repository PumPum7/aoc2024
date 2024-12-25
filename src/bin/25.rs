advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut locks, mut keys) = (Vec::with_capacity(3500), Vec::with_capacity(3500));
    
    let mut sections = input.split("\n\n");
    while let Some(section) = sections.next() {
        if section.is_empty() { continue; }
        
        let first_char = section.as_bytes()[0];
        let grid = section.lines().map(str::as_bytes).collect::<Vec<_>>();
        
        if first_char == b'#' {
            locks.push(grid);
        } else {
            keys.push(grid);
        }
    }

    Some(locks.iter()
        .map(|lock| {
            keys.iter()
                .filter(|key| is_compatible(lock, key))
                .count() as u32
        })
        .sum())
}

#[inline(always)]
fn is_compatible(lock: &[&[u8]], key: &[&[u8]]) -> bool {
    if lock.len() != key.len() { return false; }
    
    !lock.iter().enumerate().any(|(y, row)| {
        if row.len() != key[y].len() { return true; }
        
        row.iter()
            .enumerate()
            .any(|(x, &cell)| cell == b'#' && key[y][x] == b'#')
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
