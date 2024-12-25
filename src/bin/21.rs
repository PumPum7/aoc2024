use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(21);

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from((x, y): (T, T)) -> Self {
        Vec2 { x, y }
    }
}

const KEYPAD_NUMERIC: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    [' ', '0', 'A'],
];
const KEYPAD_DIRECTION: [[char; 3]; 2] = [[' ', '^', 'A'], ['<', 'v', '>']];

fn hash(pos: Vec2<usize>, next: Vec2<usize>, n_keypads: usize) -> u64 {
    let mut hash = 0;
    hash |= pos.x as u64;
    hash |= (pos.y as u64) << 8;
    hash |= (next.x as u64) << 16;
    hash |= (next.y as u64) << 24;
    hash |= (n_keypads as u64) << 32;
    hash
}

fn cost_step(
    memo: &mut HashMap<u64, usize>,
    pos: Vec2<usize>,
    next: Vec2<usize>,
    keypads: &[&[[char; 3]]],
    code: &[u8],
) -> usize {
    let hash_val = hash(pos, next, keypads.len());
    if let Some(&cost) = memo.get(&hash_val) {
        return cost;
    }

    let mut cost = usize::MAX;
    let mut q = VecDeque::new();
    q.push_back((pos, Vec::new()));
    let keypad = keypads[0];

    // Calculate Manhattan distance for early pruning
    let manhattan =
        (pos.x as isize - next.x as isize).abs() + (pos.y as isize - next.y as isize).abs();
    if manhattan as usize > code.len() {
        return usize::MAX;
    }

    while let Some((mut p, mut presses)) = q.pop_front() {
        if p == next {
            presses.push(b'A');
            cost = cost.min(cost_recursive(&presses, &keypads[1..], memo));
            continue;
        }
        if keypad[p.y][p.x] == ' ' {
            continue;
        }

        match p.x.cmp(&next.x) {
            Ordering::Less => {
                p.x += 1;
                let mut new_presses = presses.clone();
                new_presses.push(b'>');
                q.push_back((p, new_presses));
                p.x -= 1;
            }
            Ordering::Greater => {
                p.x -= 1;
                let mut new_presses = presses.clone();
                new_presses.push(b'<');
                q.push_back((p, new_presses));
                p.x += 1;
            }
            Ordering::Equal => {}
        }
        match p.y.cmp(&next.y) {
            Ordering::Less => {
                p.y += 1;
                let mut new_presses = presses.clone();
                new_presses.push(b'v');
                q.push_back((p, new_presses));
                p.y -= 1;
            }
            Ordering::Greater => {
                p.y -= 1;
                let mut new_presses = presses.clone();
                new_presses.push(b'^');
                q.push_back((p, new_presses));
                p.y += 1;
            }
            Ordering::Equal => {}
        }
    }

    memo.insert(hash_val, cost);
    cost
}

fn cost_recursive(
    presses: &[u8],
    keypads: &[&[[char; 3]]],
    memo: &mut HashMap<u64, usize>,
) -> usize {
    if keypads.is_empty() {
        return presses.len();
    }

    let keypad = keypads[0];
    let mut pos: Vec2<usize> = keypad
        .iter()
        .enumerate()
        .find_map(|(i, row)| row.iter().position(|&c| c == 'A').map(|j| (j, i)))
        .unwrap()
        .into();

    let mut cost = 0;
    for &key in presses {
        let next_pos: Vec2<usize> = keypad
            .iter()
            .enumerate()
            .find_map(|(i, row)| row.iter().position(|&ch| ch == key as char).map(|j| (j, i)))
            .unwrap()
            .into();
        cost += cost_step(memo, pos, next_pos, keypads, presses);
        pos = next_pos;
    }
    cost
}

fn complexity(code: &str, keypads: &[&[[char; 3]]], memo: &mut HashMap<u64, usize>) -> usize {
    let bytes = code.as_bytes();
    let cost = cost_recursive(bytes, keypads, memo);
    cost * std::str::from_utf8(&bytes[..bytes.len() - 1])
        .unwrap()
        .parse::<usize>()
        .unwrap()
}

fn part_one(input: &str) -> Option<u64> {
    let mut keypads: Vec<&[[char; 3]]> = Vec::new();
    keypads.push(&KEYPAD_NUMERIC);
    for _ in 0..2 {
        keypads.push(&KEYPAD_DIRECTION);
    }

    let mut memo = HashMap::new();
    let mut total = 0;

    for line in input.lines() {
        total += complexity(line, &keypads, &mut memo) as u64;
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut keypads: Vec<&[[char; 3]]> = Vec::new();
    keypads.push(&KEYPAD_NUMERIC);
    for _ in 0..25 {
        keypads.push(&KEYPAD_DIRECTION);
    }

    let mut memo = HashMap::new();
    let mut total = 0;

    for line in input.lines() {
        total += complexity(line, &keypads, &mut memo) as u64;
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
