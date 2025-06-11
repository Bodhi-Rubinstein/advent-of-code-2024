use std::{collections::HashSet, env, fs::read_to_string};

fn classify_square(
    garden: &Vec<Vec<char>>,
    target: char,
    row: usize,
    col: usize,
    found: &mut HashSet<(usize, usize)>,
    squares: &mut HashSet<(usize, usize)>,
    left_edges: &mut HashSet<(usize, usize)>,
    right_edges: &mut HashSet<(usize, usize)>,
    top_edges: &mut HashSet<(usize, usize)>,
    bottom_edges: &mut HashSet<(usize, usize)>,
) -> i32 {
    let mut score = 0;
    let row_dirs = [-1, 1, 0, 0];
    let col_dirs = [0, 0, -1, 1];

    for i in 0..4 {
        let offset_row = row as isize + row_dirs[i];
        let offset_col = col as isize + col_dirs[i];

        if offset_row < 0
            || offset_row >= garden.len() as isize
            || offset_col < 0
            || offset_col >= garden[row].len() as isize
        {
            score += 1;
            match i {
                0 => top_edges.insert((row, col)),
                1 => bottom_edges.insert((row, col)),
                2 => left_edges.insert((row, col)),
                3 => right_edges.insert((row, col)),
                _ => false,
            };
            continue;
        }

        if garden[offset_row as usize][offset_col as usize] != target {
            match i {
                0 => top_edges.insert((row, col)),
                1 => bottom_edges.insert((row, col)),
                2 => left_edges.insert((row, col)),
                3 => right_edges.insert((row, col)),
                _ => false,
            };
            score += 1;
        } else if !squares.contains(&(offset_row as usize, offset_col as usize)) {
            found.insert((offset_row as usize, offset_col as usize));
        }
    }

    return score;
}

fn remove_side(
    row: usize,
    col: usize,
    garden: &Vec<Vec<char>>,
    edge_vec: &mut Vec<(usize, usize)>,
    edges: &HashSet<(usize, usize)>,
) {
    let mut side_squares: HashSet<(usize, usize)> = HashSet::new();
    let mut new_squares: Vec<(usize, usize)> = vec![(row, col)];

    let row_dirs = [-1, 1, 0, 0];
    let col_dirs = [0, 0, -1, 1];
    let target = garden[row][col];

    let mut horizontal_side = true;
    let mut first_run = true;

    loop {
        let mut found: HashSet<(usize, usize)> = HashSet::new();

        for new_square in &new_squares {
            if !horizontal_side || first_run {
                first_run = false;
                for i in 0..2 {
                    let offset_row = new_square.0 as isize + row_dirs[i];
                    let offset_col = new_square.1 as isize + col_dirs[i];

                    if edges.contains(&(offset_row as usize, offset_col as usize))
                        && !side_squares.contains(&(offset_row as usize, offset_col as usize))
                        && garden[offset_row as usize][offset_col as usize] == target
                    {
                        found.insert((offset_row as usize, offset_col as usize));
                        horizontal_side = false;
                    }
                }
            }
            if horizontal_side {
                for i in 2..4 {
                    let offset_row = new_square.0 as isize + row_dirs[i];
                    let offset_col = new_square.1 as isize + col_dirs[i];

                    if edges.contains(&(offset_row as usize, offset_col as usize))
                        && !side_squares.contains(&(offset_row as usize, offset_col as usize))
                        && garden[offset_row as usize][offset_col as usize] == target
                    {
                        found.insert((offset_row as usize, offset_col as usize));
                    }
                }
            }

            side_squares.insert(*new_square);
            let index = edge_vec
                .iter()
                .position(|x| x.0 == new_square.0 && x.1 == new_square.1);

            if index != None {
                edge_vec.swap_remove(index.unwrap());
            }
        }

        if found.len() == 0 {
            break;
        }

        new_squares.clear();
        new_squares.extend(found.iter());
    }
}

fn measure_region(
    garden: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    visited: &mut Vec<Vec<bool>>,
) -> (i32, i32) {
    let target = garden[row][col];
    let mut squares: HashSet<(usize, usize)> = HashSet::new();
    let mut new_squares: Vec<(usize, usize)> = vec![(row, col)];
    let mut region_cost = 0;
    let mut left_edges: HashSet<(usize, usize)> = HashSet::new();
    let mut right_edges: HashSet<(usize, usize)> = HashSet::new();
    let mut top_edges: HashSet<(usize, usize)> = HashSet::new();
    let mut bottom_edges: HashSet<(usize, usize)> = HashSet::new();

    loop {
        let mut found: HashSet<(usize, usize)> = HashSet::new();

        for new_square in &new_squares {
            let score = classify_square(
                garden,
                target,
                new_square.0,
                new_square.1,
                &mut found,
                &mut squares,
                &mut left_edges,
                &mut right_edges,
                &mut top_edges,
                &mut bottom_edges,
            );
            region_cost += score;
            squares.insert(*new_square);
            visited[new_square.0][new_square.1] = true;
        }

        if found.len() == 0 {
            break;
        }

        new_squares.clear();
        new_squares.extend(found.iter());
    }

    let mut sides = 0;

    let mut left_edge_vec: Vec<(usize, usize)> = left_edges.clone().into_iter().collect();
    loop {
        if left_edge_vec.len() == 0 {
            break;
        }
        sides += 1;
        let first = left_edge_vec.pop();
        match first {
            None => (),
            Some(side) => remove_side(side.0, side.1, &garden, &mut left_edge_vec, &left_edges),
        }
    }

    let mut right_edge_vec: Vec<(usize, usize)> = right_edges.clone().into_iter().collect();
    loop {
        if right_edge_vec.len() == 0 {
            break;
        }
        sides += 1;
        let first = right_edge_vec.pop();
        match first {
            None => (),
            Some(side) => remove_side(side.0, side.1, &garden, &mut right_edge_vec, &right_edges),
        }
    }

    let mut top_edge_vec: Vec<(usize, usize)> = top_edges.clone().into_iter().collect();
    loop {
        if top_edge_vec.len() == 0 {
            break;
        }
        sides += 1;
        let first = top_edge_vec.pop();
        match first {
            None => (),
            Some(side) => remove_side(side.0, side.1, &garden, &mut top_edge_vec, &top_edges),
        }
    }

    let mut bottom_edge_vec: Vec<(usize, usize)> = bottom_edges.clone().into_iter().collect();
    loop {
        if bottom_edge_vec.len() == 0 {
            break;
        }
        sides += 1;
        let first = bottom_edge_vec.pop();
        match first {
            None => (),
            Some(side) => remove_side(side.0, side.1, &garden, &mut bottom_edge_vec, &bottom_edges),
        }
    }

    region_cost *= squares.len() as i32;
    let p2_region_cost = sides * squares.len() as i32;
    return (region_cost, p2_region_cost);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();
    let garden: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; garden[0].len()]; garden.len()];

    let mut p1_cost_total = 0;
    let mut p2_cost_total = 0;
    for row in 0..visited.len() {
        for col in 0..visited[row].len() {
            if visited[row][col] {
                continue;
            }
            let (p1_cost, p2_cost) = measure_region(&garden, row, col, &mut visited);
            p1_cost_total += p1_cost;
            p2_cost_total += p2_cost;
        }
    }

    println!("{}", p1_cost_total);
    println!("{}", p2_cost_total);
}
