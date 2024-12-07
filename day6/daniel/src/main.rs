use std::collections::HashSet;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::os;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn advance<'a>(
    area_map: &mut Vec<Vec<char>>,
    guard_dir: &'a mut Direction,
    squares: &mut HashSet<(usize, usize)>,
) -> bool {
    let mut pos_x = None;
    let mut pos_y = None;
    for y in 0..area_map.len() {
        for x in 0..area_map[y].len() {
            if area_map[y][x] == '^' {
                pos_x = Some(x);
                pos_y = Some(y);
                break;
            }
        }
    }

    if pos_x.is_none() || pos_y.is_none() {
        panic!("Player missing from board!");
    }

    squares.insert((pos_x.unwrap(), pos_y.unwrap()));

    let (target_y, target_x) = match guard_dir {
        Direction::UP => (pos_y.unwrap() as isize - 1, pos_x.unwrap() as isize),
        Direction::DOWN => (pos_y.unwrap() as isize + 1, pos_x.unwrap() as isize),
        Direction::LEFT => (pos_y.unwrap() as isize, pos_x.unwrap() as isize - 1),
        Direction::RIGHT => (pos_y.unwrap() as isize, pos_x.unwrap() as isize + 1),
    };

    if target_x < 0
        || target_x >= area_map[0].len() as isize
        || target_y < 0
        || target_y >= area_map.len() as isize
    {
        return false;
    }

    // let mut map_x = 0;
    // let mut map_y = 0;

    if area_map[target_y as usize][target_x as usize] == '.' {
        area_map[pos_y.unwrap()][pos_x.unwrap()] = '.';
        area_map[target_y as usize][target_x as usize] = '^';
        // map_x = target_x;
        // map_y = target_y;
    } else if area_map[target_y as usize][target_x as usize] == '#'
        || area_map[target_y as usize][target_x as usize] == '0'
    {
        // map_x = pos_x.unwrap() as isize;
        // map_y = pos_y.unwrap() as isize;

        *guard_dir = match *guard_dir {
            Direction::UP => Direction::RIGHT,
            Direction::RIGHT => Direction::DOWN,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::UP,
        }
    }

    // print!("\x1B[2J\x1B[1;1H");
    // io::stdout().flush().unwrap();
    // for y_ind in map_y - 10..map_y + 10 {
    //     for x_ind in map_x - 20..map_x + 20 {
    //         if y_ind < 0
    //             || x_ind < 0
    //             || y_ind >= area_map.len() as isize
    //             || x_ind >= area_map[0].len() as isize
    //         {
    //             print!(" ");
    //             continue;
    //         } else if squares.contains(&(x_ind as usize, y_ind as usize))
    //             && !(x_ind == map_x && y_ind == map_y)
    //         {
    //             print!("@");
    //             continue;
    //         }

    //         print!("{}", area_map[y_ind as usize][x_ind as usize]);
    //     }

    //     println!();
    // }
    // println!("Squares: {}", squares.len());

    // if target_y == map_y && target_x == map_x {
    //     thread::sleep(Duration::from_millis(100));
    // } else {
    //     thread::sleep(Duration::from_millis(250));
    // }

    // create minimap
    // println!(
    //     "pos_x: {}, pos_y: {}, target_x: {}, target_y: {}, dir: {:?}, squares: {}",
    //     pos_x.unwrap(),
    //     pos_y.unwrap(),
    //     target_x,
    //     target_y,
    //     guard_dir,
    //     squares.len(),
    // );

    return true;
}

fn count_potential_loops(area_map: &Vec<Vec<char>>) -> i32 {
    //theoretical max new squares
    let max_squares = area_map[0].len() * 2 + area_map.len() * 2 - 2;
    let mut num_loops = 0;

    for y in 0..area_map.len() {
        for x in 0..area_map[y].len() {
            // print!("\x1B[2J\x1B[1;1H");
            // io::stdout().flush().unwrap();
            // println!(
            //     "{}/{}...",
            //     y * area_map.len() + x,
            //     area_map.len() * area_map[0].len()
            // );
            if area_map[y][x] == '#' {
                continue;
            } else if area_map[y][x] == '^' {
                continue;
            }

            let mut iters_since = 0;
            let mut last_val = 0;
            let mut cloned_map = area_map.clone();
            cloned_map[y][x] = '0';
            let mut guard_dir = Direction::UP;
            let mut squares: HashSet<(usize, usize)> = HashSet::new();

            while advance(&mut cloned_map, &mut guard_dir, &mut squares) {
                iters_since += 1;

                if last_val != squares.len() {
                    last_val = squares.len();
                    iters_since = 0;
                }

                if iters_since > max_squares {
                    num_loops += 1;
                    break;
                }
            }
        }
    }

    return num_loops;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();
    let mut area_map: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();
    let mut guard_dir: Direction = Direction::UP;
    let mut squares: HashSet<(usize, usize)> = HashSet::new();

    while advance(&mut area_map, &mut guard_dir, &mut squares) {
        // yayayayayay
        // for y in &area_map {
        //     for x in y {
        //         print!("{}", x);
        //     }
        //     println!();
        // }
        // println!("{:?}", squares);
        // println!("{:?}\n", guard_dir);
    }

    println!("{}", squares.len());

    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).unwrap();
    let area_map: Vec<Vec<char>> = file.lines().map(|x| x.chars().collect()).collect();
    let num_possible_loops = count_potential_loops(&area_map);

    println!("{}", num_possible_loops);
}
