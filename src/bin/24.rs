use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(24);

type Gate<'a> = (&'a str, &'a str, &'a str);
type GateMap<'a> = HashMap<&'a str, Gate<'a>>;

#[derive(Debug)]
enum GateOperation {
    And,
    Xor,
    Or,
}

impl From<&str> for GateOperation {
    fn from(op: &str) -> Self {
        match op {
            "AND" => Self::And,
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            _ => panic!("Invalid gate operation: {}", op),
        }
    }
}

fn evaluate_gate<'a>(gates: &GateMap<'a>, signals: &mut HashMap<&'a str, bool>, node: &'a str) {
    if signals.contains_key(node) {
        return;
    }

    let (input1, op, input2) = gates[node];
    evaluate_gate(gates, signals, input1);
    evaluate_gate(gates, signals, input2);

    let result = match GateOperation::from(op) {
        GateOperation::And => signals[input1] && signals[input2],
        GateOperation::Xor => signals[input1] != signals[input2],
        GateOperation::Or => signals[input1] || signals[input2],
    };
    signals.insert(node, result);
}

fn calculate_signal_value(signals: &HashMap<&str, bool>, prefix: char) -> usize {
    signals
        .iter()
        .filter(|(&name, _)| name.starts_with(prefix))
        .filter_map(|(name, &value)| {
            value.then(|| {
                name[1..]
                    .parse::<usize>()
                    .map(|n| 1 << n)
                    .unwrap_or_default()
            })
        })
        .sum()
}

fn parse_input(input: &str) -> Option<(HashMap<&str, bool>, &str)> {
    let (initial_signals, circuit) = input.split_once("\n\n")?;

    let initial_states = initial_signals
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ")?;
            Some((name, value == "1"))
        })
        .collect::<Option<HashMap<_, _>>>()?;

    Some((initial_states, circuit))
}

fn parse_gates(circuit: &str) -> GateMap {
    circuit
        .lines()
        .filter_map(|line| {
            let (in1, op, in2, _, out) = line.split(' ').collect_tuple()?;
            Some((out, (in1, op, in2)))
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut signals, circuit) = parse_input(input)?;
    let gates = parse_gates(circuit);

    gates
        .keys()
        .for_each(|&node| evaluate_gate(&gates, &mut signals, node));
    Some(calculate_signal_value(&signals, 'z'))
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, circuit) = input.split_once("\n\n")?;
    let gates = parse_gates(circuit);

    let mut incorrect_gates = Vec::new();
    let mut categories: HashMap<&str, Vec<(&str, Gate)>> = HashMap::new();

    // Categorize gates
    for (&signal, &gate @ (in1, op, in2)) in &gates {
        let category = match (
            in1.starts_with(['x', 'y']) || in2.starts_with(['x', 'y']),
            op,
        ) {
            (true, "AND") => "xy_carry",
            (true, "XOR") => "xy_sum",
            (false, "AND") => "full_carry",
            (false, "XOR") => "outputs",
            (false, "OR") => "carry",
            _ => continue,
        };

        categories.entry(category).or_default().push((signal, gate));
    }

    // Process each category
    process_outputs(&categories, &mut incorrect_gates);
    process_xy_sums(&categories, &mut incorrect_gates);
    process_xy_carries(&categories, &mut incorrect_gates);
    process_carries(&categories, &mut incorrect_gates);
    process_full_carries(&categories, &mut incorrect_gates);

    Some(incorrect_gates.into_iter().sorted().join(","))
}

fn process_outputs<'a>(
    categories: &HashMap<&str, Vec<(&'a str, (&'a str, &str, &'a str))>>,
    incorrect: &mut Vec<&'a str>,
) {
    if let Some(outputs) = categories.get("outputs") {
        incorrect.extend(
            outputs
                .iter()
                .filter(|&(signal, _)| !signal.starts_with('z'))
                .map(|&(signal, _)| signal),
        );
    }
}

fn process_xy_sums<'a>(
    categories: &HashMap<&str, Vec<(&'a str, (&'a str, &str, &'a str))>>,
    incorrect: &mut Vec<&'a str>,
) {
    if let Some(xy_sums) = categories.get("xy_sum") {
        let default = Vec::new();
        let outputs = categories.get("outputs").unwrap_or(&default);
        for &(signal, (in1, _, _)) in xy_sums {
            if in1 == "x00" || in1 == "y00" {
                continue;
            }
            let used_in_output = outputs
                .iter()
                .any(|&(_, (i1, _, i2))| i1 == signal || i2 == signal);
            if signal.starts_with('z') || !used_in_output {
                incorrect.push(signal);
            }
        }
    }
}

fn process_xy_carries<'a>(
    categories: &HashMap<&str, Vec<(&'a str, (&'a str, &str, &'a str))>>,
    incorrect: &mut Vec<&'a str>,
) {
    if let Some(xy_carries) = categories.get("xy_carry") {
        let default = Vec::new();
        let carries = categories.get("carry").unwrap_or(&default);
        for &(signal, (in1, _, _)) in xy_carries {
            if in1 == "x00" || in1 == "y00" {
                continue;
            }
            let used_in_carry = carries
                .iter()
                .any(|&(_, (i1, _, i2))| i1 == signal || i2 == signal);
            if signal.starts_with('z') || !used_in_carry {
                incorrect.push(signal);
            }
        }
    }
}

fn process_carries<'a>(
    categories: &HashMap<&str, Vec<(&'a str, (&'a str, &str, &'a str))>>,
    incorrect: &mut Vec<&'a str>,
) {
    if let Some(carries) = categories.get("carry") {
        incorrect.extend(
            carries
                .iter()
                .filter(|&&(signal, _)| signal.starts_with('z') && signal != "z45")
                .map(|&(signal, _)| signal),
        );
    }
}

fn process_full_carries<'a>(
    categories: &HashMap<&str, Vec<(&'a str, (&'a str, &str, &'a str))>>,
    incorrect: &mut Vec<&'a str>,
) {
    if let Some(full_carries) = categories.get("full_carry") {
        incorrect.extend(
            full_carries
                .iter()
                .filter(|&&(signal, _)| signal.starts_with('z'))
                .map(|&(signal, _)| signal),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            Some("ffh,mjb,tgd,wpb,z02,z03,z05,z06,z07,z08,z10,z11".to_string())
        );
    }
}
