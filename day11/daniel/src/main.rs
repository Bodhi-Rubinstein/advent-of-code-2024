use std::{collections::HashMap, env, fs::read_to_string};

fn map_get_next_nums(map: &mut HashMap<u64, u64>) {
    let old_map = map.clone();
    let curr_keys: Vec<&u64> = old_map
        .keys()
        .filter(|key| old_map.get(*key) != Some(&0))
        .collect();

    for num in curr_keys {
        let num_str = num.to_string();
        if *num == 0 {
            let curr_val = map.get(&1);

            match curr_val {
                None => map.insert(1, *old_map.get(num).unwrap()),
                Some(n) => map.insert(1, n + *old_map.get(num).unwrap()),
            };
        } else if num_str.len() % 2 == 0 {
            let left_num: u64 = num_str[0..num_str.len() / 2].parse().unwrap();
            let right_num: u64 = num_str[num_str.len() / 2..num_str.len()].parse().unwrap();

            let curr_left_val = map.get(&left_num);
            match curr_left_val {
                None => map.insert(left_num, *old_map.get(num).unwrap()),
                Some(n) => map.insert(left_num, n + *old_map.get(num).unwrap()),
            };

            let curr_right_val = map.get(&right_num);
            match curr_right_val {
                None => map.insert(right_num, *old_map.get(num).unwrap()),
                Some(n) => map.insert(right_num, n + *old_map.get(num).unwrap()),
            };
        } else {
            let curr_val = map.get(&(*num * 2024));

            match curr_val {
                None => map.insert(*num * 2024, *old_map.get(num).unwrap()),
                Some(n) => map.insert(*num * 2024, n + *old_map.get(num).unwrap()),
            };
        }

        let curr_val = map.get(num).unwrap();
        map.insert(*num, curr_val - *old_map.get(num).unwrap());
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();
    let nums: Vec<u64> = file.split(" ").map(|x| x.trim().parse().unwrap()).collect();

    let mut map: HashMap<u64, u64> = HashMap::new();

    for num in nums {
        let curr_val = map.get(&num);

        map.insert(num, *curr_val.unwrap_or(&0) + 1);
    }

    for i in 0..75 {
        map_get_next_nums(&mut map);
        if i == 24 {
            println!("{:?}", map.values().sum::<u64>());
        }
    }

    println!("{:?}", map.values().sum::<u64>());
}
