use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Report {
    data: Vec<i32>,
}

impl Report {
    fn is_safe_p1(&self) -> i32 {
        for i in 1..self.data.len() {
            if (self.data[i] - self.data[i - 1]).abs() > 3 || self.data[i] - self.data[i - 1] == 0 {
                return 0;
            }

            if i > 1
                && (self.data[i - 1] - self.data[i - 2]).is_positive()
                    != (self.data[i] - self.data[i - 1]).is_positive()
            {
                return 0;
            }
        }

        return 1;
    }
    fn is_safe(&self) -> i32 {
        for skip in -1..self.data.len() as i32 {
            let filtered_data = self.data.iter().enumerate().filter_map(|(i, &e)| {
                if i as i32 != skip {
                    Some(e)
                } else {
                    None
                }
            });

            let mut is_unsafe = false;
            let mut curr = -1;
            let mut last = -1;
            let mut last_last;

            for e in filtered_data {
                last_last = last;
                last = curr;
                curr = e;

                if last != -1 && ((curr - last).abs() > 3 || curr - last == 0) {
                    is_unsafe = true;
                    break;
                }

                if last_last != -1
                    && (last - last_last).is_positive() != (curr - last).is_positive()
                {
                    is_unsafe = true;
                    break;
                }
            }
            if !is_unsafe {
                return 1;
            }
        }

        return 0;
    }
}

#[derive(Debug)]
struct ReportError;

impl FromStr for Report {
    type Err = ReportError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Vec<i32> = s.split(" ").map(|x| i32::from_str(x).unwrap()).collect();
        return Ok(Report { data: digits });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: String = fs::read_to_string(args[1].as_str()).unwrap();

    let num_safe_p1: i32 = input
        .lines()
        .map(|x| Report::from_str(x).unwrap())
        .map(|x| x.is_safe_p1())
        .sum();
    println!("{}", num_safe_p1); // PART ONE

    let num_safe_p2: i32 = input
        .lines()
        .map(|x| Report::from_str(x).unwrap())
        .map(|x| x.is_safe())
        .sum();
    println!("{}", num_safe_p2); // PART TWO
}
