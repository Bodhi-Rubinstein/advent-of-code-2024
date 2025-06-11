use std::env;
use std::fs;

fn is_correct_ordering(ordering: &Vec<i32>, rules: &Vec<Vec<i32>>) -> bool {
    for i in 1..ordering.len() {
        let relevant_rule: &Vec<&Vec<i32>> = &rules
            .iter()
            .filter(|x| {
                (x[0] == ordering[i] && x[1] == ordering[i - 1])
                    || (x[1] == ordering[i] && x[0] == ordering[i - 1])
            })
            .collect();

        if relevant_rule[0][0] != ordering[i - 1] {
            return false;
        }
    }

    return true;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_string = fs::read_to_string(&args[1]).unwrap();
    let parts = file_string.split("\n\n").collect::<Vec<&str>>();

    let part1: Vec<Vec<i32>> = parts[0]
        .split("\n")
        .map(|x| x.split("|").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();

    let mut part2: Vec<Vec<i32>> = parts[1]
        .split("\n")
        .map(|x| {
            x.split(",")
                .map(|x| x.parse::<i32>().unwrap_or(-1))
                .filter(|x| *x != -1)
                .collect()
        })
        .filter(|x: &Vec<i32>| x.len() > 0)
        .collect();

    let mut sum = 0;
    let mut p2_sum = 0;
    for page_update in (&mut part2).iter_mut() {
        if is_correct_ordering(page_update, &part1) {
            if page_update.len() % 2 == 0 {
                panic!("AAHAAHA {}", page_update.len())
            }
            sum += page_update[page_update.len() / 2]
        } else {
            page_update.sort_by(|a, b| {
                let entry: &Vec<&Vec<i32>> = &part1
                    .iter()
                    .filter(|x| (x[0] == *a && x[1] == *b) || (x[1] == *a && x[0] == *b))
                    .collect();

                if entry[0][0] == *a {
                    return std::cmp::Ordering::Less;
                }
                return std::cmp::Ordering::Greater;
            });

            p2_sum += page_update[page_update.len() / 2]
        }
    }

    println!("{}", sum);
    println!("{}", p2_sum);
}
