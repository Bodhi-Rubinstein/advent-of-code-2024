use std::{collections::HashSet, env, fs::read_to_string};

fn get_trailhead_score(
    board: &Vec<Vec<i32>>,
    x: usize,
    y: usize,
    peaks: &mut HashSet<(usize, usize)>,
) -> (i32, i32) {
    if board[y][x] == 9 {
        if !peaks.contains(&(x, y)) {
            peaks.insert((x, y));
            return (1, 1);
        }

        return (0, 1);
    }

    let mut total_p1: i32 = 0;
    let mut total_p2: i32 = 0;
    if y + 1 < board.len() && board[y + 1][x] == board[y][x] + 1 {
        let (p1, p2) = get_trailhead_score(board, x, y + 1, peaks);
        total_p1 += p1;
        total_p2 += p2;
    }
    if y > 0 && board[y - 1][x] == board[y][x] + 1 {
        let (p1, p2) = get_trailhead_score(board, x, y - 1, peaks);
        total_p1 += p1;
        total_p2 += p2;
    }
    if x + 1 < board[y].len() && board[y][x + 1] == board[y][x] + 1 {
        let (p1, p2) = get_trailhead_score(board, x + 1, y, peaks);
        total_p1 += p1;
        total_p2 += p2;
    }
    if x > 0 && board[y][x - 1] == board[y][x] + 1 {
        let (p1, p2) = get_trailhead_score(board, x - 1, y, peaks);
        total_p1 += p1;
        total_p2 += p2;
    }

    return (total_p1, total_p2);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = read_to_string(&args[1]).unwrap();
    let board: Vec<Vec<i32>> = file
        .lines()
        .map(|x| x.chars().map(|y| (y as i32 - 48)).collect())
        .collect();

    let mut total_score_p1 = 0;
    let mut total_score_p2 = 0;
    for y in 0..board.len() {
        for x in 0..board[y].len() {
            if board[y][x] == 0 {
                // rec search
                let mut peaks: HashSet<(usize, usize)> = HashSet::new();
                let score = get_trailhead_score(&board, x, y, &mut peaks);
                total_score_p1 += score.0;
                total_score_p2 += score.1;
            }
        }
    }
    println!("{}", total_score_p1);
    println!("{}", total_score_p2);
}
