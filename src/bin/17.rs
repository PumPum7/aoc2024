advent_of_code::solution!(17);

#[derive(Debug)]
struct Computer {
    registers: [i64; 3],  // A, B, C
    instruction_pointer: usize,
    program: Vec<i64>,
}

impl Computer {
    fn new(program: Vec<i64>, a: i64, b: i64, c: i64) -> Self {
        Computer {
            registers: [a, b, c],
            instruction_pointer: 0,
            program,
        }
    }

    fn get_combo_value(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.registers[0],  // A
            5 => self.registers[1],  // B
            6 => self.registers[2],  // C
            _ => panic!("Invalid combo operand: {}", operand),
        }
    }

    fn run(&mut self) -> Vec<i64> {
        let mut output = Vec::with_capacity(self.program.len() / 2);  // Pre-allocate with estimated size
        
        while self.instruction_pointer < self.program.len() {
            // Read both values at once to avoid multiple array accesses
            let (opcode, operand) = (
                self.program[self.instruction_pointer],
                self.program[self.instruction_pointer + 1]
            );
            
            match opcode {
                0 => self.registers[0] >>= self.get_combo_value(operand),  // adv
                1 => { // bxl
                    self.registers[1] ^= operand;
                }
                2 => { // bst
                    self.registers[1] = self.get_combo_value(operand) % 8;
                }
                3 => { // jnz
                    if self.registers[0] != 0 {
                        self.instruction_pointer = operand as usize;
                        continue;
                    }
                }
                4 => { // bxc
                    self.registers[1] ^= self.registers[2];
                }
                5 => { // out
                    output.push(self.get_combo_value(operand) % 8);
                }
                6 => { // bdv
                    let power = self.get_combo_value(operand);
                    self.registers[1] = self.registers[0] / (1 << power);
                }
                7 => { // cdv
                    let power = self.get_combo_value(operand);
                    self.registers[2] = self.registers[0] / (1 << power);
                }
                _ => panic!("Invalid opcode: {}", opcode),
            }
            
            self.instruction_pointer += 2;
        }
        
        output
    }
}

pub fn part_one(input: &str) -> Option<String> {
    // Pre-allocate vector with capacity
    let mut numbers = Vec::with_capacity(16);
    
    for line in input.lines().filter(|line| !line.is_empty()) {
        if line.starts_with("Register A:") {
            numbers.push(line.split(": ").nth(1).unwrap().parse().unwrap());
        } else if line.starts_with("Program:") {
            numbers.extend(
                line.split(": ")
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|n| n.trim().parse::<i64>().unwrap())
            );
        }
    }
    let initial_a = numbers[0];
    let program = numbers[1..].to_vec();
    
    let mut computer = Computer::new(program, initial_a, 0, 0);
    let output = computer.run();
    
    Some(output.iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(","))
}

pub fn part_two(input: &str) -> Option<String> {
    // Parse input similar to part_one
    let numbers: Vec<i64> = input.lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            if line.starts_with("Register A:") {
                vec![line.split(": ").nth(1).unwrap().parse().unwrap()]
            } else if line.starts_with("Program:") {
                line.split(": ").nth(1).unwrap()
                    .split(',')
                    .map(|n| n.trim().parse().unwrap())
                    .collect()
            } else {
                vec![]
            }
        })
        .collect();

    let program = numbers[1..].to_vec();
    
    // Initialize factors array to track digits in different positions
    let mut factors = vec![0; program.len()];
    
    loop {
        // Calculate initial A value based on powers of 8
        let mut init_a: i64 = 0;
        for (i, &f) in factors.iter().enumerate() {
            init_a += 8_i64.pow(i as u32) * f;
        }
        
        let mut computer = Computer::new(program.clone(), init_a, 0, 0);
        let output = computer.run();
        
        if output == program {
            return Some(init_a.to_string());
        }

        // Update factors based on mismatch position
        for i in (0..program.len()).rev() {
            if output.len() <= i || output[i] != program[i] {
                factors[i] += 1;
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some("117440".to_string()));
    }
}
