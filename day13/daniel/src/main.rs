use std::{env, fs::read_to_string, str::FromStr};

#[derive(Debug)]
struct ClawMachine {
    a: (i32, i32),
    b: (i32, i32),
    prize: (i32, i32),
}

fn parse_button_line_x(s: &str) -> i32 {
    return s
        .split(" ")
        .nth(2)
        .unwrap()
        .trim_end_matches(',')
        .split("+")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
}
fn parse_button_line_y(s: &str) -> i32 {
    return s
        .split(" ")
        .nth(3)
        .unwrap()
        .split("+")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
}

fn parse_prize_line_x(s: &str) -> i32 {
    return s
        .split(" ")
        .nth(1)
        .unwrap()
        .trim_end_matches(',')
        .split("=")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
}

fn parse_prize_line_y(s: &str) -> i32 {
    return s
        .split(" ")
        .nth(2)
        .unwrap()
        .split("=")
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();
}

#[derive(Debug)]
struct ClawMachineError {}

impl FromStr for ClawMachine {
    type Err = ClawMachineError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        return Ok(ClawMachine {
            a: (parse_button_line_x(lines[0]), parse_button_line_y(lines[0])),
            b: (parse_button_line_x(lines[1]), parse_button_line_y(lines[1])),
            prize: (parse_prize_line_x(lines[2]), parse_prize_line_y(lines[2])),
        });
    }
}

impl ClawMachine {
    fn brute_force(&self) -> Option<(i32, i32, i32)> {
        // A presses, B presses, Token cost
        let mut solutions: Vec<(i32, i32, i32)> = Vec::new();
        for a in 1..101 {
            // claw position after "a" A button presses
            let claw_pos = (self.a.0 * a, self.a.1 * a);

            // we have gone past the prize
            if claw_pos.0 > self.prize.0 || claw_pos.1 > self.prize.1 {
                break;
            }

            // b presses for x left
            let b_x_left: f32 = (self.prize.0 as f32 - claw_pos.0 as f32) / self.b.0 as f32;

            // there is not an even number of b presses left
            if b_x_left != b_x_left.floor() {
                continue;
            }

            let b_y_left: f32 = (self.prize.1 as f32 - claw_pos.1 as f32) / self.b.1 as f32;
            if b_y_left != b_y_left.floor() {
                continue;
            }

            // uneven presses required for each axis
            if b_x_left != b_y_left {
                continue;
            }

            // 10_000_000_000_000

            // get cost
            let cost = 3 * a + b_x_left as i32;

            // yay solution
            solutions.push((a, b_x_left as i32, cost));
        }

        if solutions.len() == 0 {
            return None;
        }

        solutions.sort_by(|a, b| a.2.cmp(&b.2));

        return Some(solutions[0]);
    }

    fn brute_force_p2(&self) -> Option<(i64, i64, i64)> {
        let offset: i64 = 10_000_000_000_000;
        let prize_x = offset + self.prize.0 as i64;
        let prize_y = offset + self.prize.1 as i64;

        let b: f64 = (prize_y * self.a.0 as i64 - prize_x * self.a.1 as i64) as f64
            / (self.b.1 as i64 * self.a.0 as i64 - self.a.1 as i64 * self.b.0 as i64) as f64;

        if b != b.floor() {
            return None;
        }

        let a: f64 = (prize_x - self.b.0 as i64 * b as i64) as f64 / self.a.0 as f64;

        if a != a.floor() {
            return None;
        }

        let cost = 3 * a as i64 + b as i64;

        return Some((a as i64, b as i64, cost));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();

    let machines: Vec<ClawMachine> = file
        .split("\n\n")
        .map(|x| ClawMachine::from_str(x).unwrap())
        .collect();

    let mut total_cost: i64 = 0;
    for machine in machines.iter() {
        match machine.brute_force() {
            Some((_, _, cost)) => total_cost += cost as i64,
            None => (),
        }
    }
    println!("{}", total_cost);

    total_cost = 0;
    for machine in machines.iter() {
        match machine.brute_force_p2() {
            Some((_, _, cost)) => total_cost += cost,
            None => (),
        }
    }
    println!("{}", total_cost);
}
