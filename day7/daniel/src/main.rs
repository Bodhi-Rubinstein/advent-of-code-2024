use std::{env, fs, str::FromStr};

struct OperatorMapping {
    operators: Vec<u8>,
}

impl OperatorMapping {
    fn new(num_operators: i32) -> Self {
        return OperatorMapping {
            operators: vec![0; num_operators as usize],
        };
    }

    fn next(&mut self) -> bool {
        let op_len = self.operators.len();
        self.operators[op_len - 1] += 1;

        for i in (0..self.operators.len()).rev() {
            if self.operators[i] > 2 {
                if i == 0 {
                    return false;
                }
                self.operators[i] = 0;
                self.operators[i - 1] += 1;
            }
        }

        return true;
    }
}

#[derive(Debug)]
struct CalibrationEquation {
    test_value: i128,
    nums: Vec<i128>,
}

#[derive(Debug)]
struct CalibrationEquationError;

impl FromStr for CalibrationEquation {
    type Err = CalibrationEquationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        let part1: i128 = parts[0].parse().unwrap();
        let part2: Vec<i128> = parts[1].split(" ").map(|x| x.parse().unwrap()).collect();

        return Ok(CalibrationEquation {
            test_value: part1,
            nums: part2,
        });
    }
}

impl CalibrationEquation {
    fn valid(&self) -> bool {
        let mut new_mapping = OperatorMapping::new(self.nums.len() as i32);
        loop {
            // each operator
            let mut curr_result = self.nums[0];

            for j in 1..self.nums.len() {
                if new_mapping.operators[j] == 0 {
                    curr_result += self.nums[j];
                } else if new_mapping.operators[j] == 1 {
                    curr_result *= self.nums[j];
                } else {
                    let mut result_str: String = curr_result.to_string();
                    let next_str = self.nums[j].to_string();

                    result_str.push_str(&next_str);
                    curr_result = result_str.parse().unwrap();
                }
            }
            if curr_result == self.test_value {
                return true;
            }

            if !new_mapping.next() {
                break;
            }
        }

        return false;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();
    let part1: i128 = file
        .lines()
        .map(|line| CalibrationEquation::from_str(line).unwrap())
        .filter(|equation| equation.valid())
        .map(|equation| equation.test_value)
        .sum();

    println!("{}", part1);
}
