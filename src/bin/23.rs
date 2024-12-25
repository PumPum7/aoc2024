use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    // Parse input and build graph
    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let mut triangle_count = 0;

    // For each node
    for &node1 in graph.keys() {
        // For each neighbor of node1
        for &node2 in &graph[node1] {
            // For each neighbor of node2
            for &node3 in &graph[node2] {
                // Check if node3 connects back to node1 to form triangle
                if node3 != node1 && graph[node3].contains(node1) {
                    // Check if any node starts with 't'
                    if node1.starts_with('t') || node2.starts_with('t') || node3.starts_with('t') {
                        triangle_count += 1;
                    }
                }
            }
        }
    }

    // Each triangle is counted 6 times (3! permutations)
    Some(triangle_count / 6)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }

    let nodes: Vec<&str> = graph.keys().copied().collect();
    let mut max_clique: HashSet<&str> = HashSet::new();

    // Try all possible combinations of nodes
    for &start_node in &nodes {
        let mut current_clique = HashSet::from([start_node]);
        let mut candidates: HashSet<&str> = graph[start_node].clone();

        // Recursively grow the clique
        find_max_clique(
            &graph,
            &mut current_clique,
            &mut candidates,
            &mut max_clique,
        );
    }

    let mut result: Vec<&str> = max_clique.into_iter().collect();
    result.sort_unstable();
    Some(result.join(","))
}

fn find_max_clique<'a>(
    graph: &HashMap<&'a str, HashSet<&'a str>>,
    current: &mut HashSet<&'a str>,
    candidates: &mut HashSet<&'a str>,
    max_clique: &mut HashSet<&'a str>,
) {
    if current.len() > max_clique.len() {
        *max_clique = current.clone();
    }

    let mut candidates = candidates.clone();
    while let Some(&node) = candidates.iter().next() {
        candidates.remove(&node);

        // Check if node is connected to all nodes in current clique
        if current.iter().all(|&n| graph[n].contains(node)) {
            current.insert(node);
            let mut new_candidates: HashSet<&str> = candidates
                .iter()
                .filter(|&&c| graph[node].contains(c))
                .copied()
                .collect();

            find_max_clique(graph, current, &mut new_candidates, max_clique);
            current.remove(&node);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
