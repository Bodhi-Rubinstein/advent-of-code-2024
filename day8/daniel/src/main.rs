use std::{
    collections::HashSet,
    env,
    fs::read_to_string,
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
struct Antenna {
    position: Coordinate,
    frequency: char,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: String = read_to_string(&args[1]).unwrap();
    let lines: Vec<&str> = file.lines().collect();

    let max_y_bound: i32 = lines.len().try_into().unwrap();
    let max_x_bound: i32 = lines[0].len().try_into().unwrap();

    // print!("{} {}", max_x_bound, max_y_bound);

    let mut antennas: Vec<Antenna> = Vec::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '.' => continue,
                c => antennas.push(Antenna {
                    position: Coordinate {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                    frequency: c,
                }),
            }
        }
    }

    antennas.sort_by(|a, b| a.frequency.cmp(&b.frequency));

    // println!("{:?} {}", antennas, antennas.len());

    let mut antinodes: HashSet<Coordinate> = HashSet::new();

    let mut curr_freq = antennas[0].frequency;
    let mut start_ind = 0;
    let mut end_ind = 1;
    loop {
        while end_ind < antennas.len() && antennas[end_ind].frequency == curr_freq {
            end_ind += 1;
        }

        // we have no more!
        if end_ind - start_ind <= 1 && end_ind >= antennas.len() {
            break;
        }

        for i in start_ind..end_ind {
            for j in i + 1..end_ind {
                let diff = antennas[j].position - antennas[i].position;

                let mut multiplier = 1;

                antinodes.insert(antennas[i].position);
                antinodes.insert(antennas[j].position);

                loop {
                    let mul_diff = Coordinate {
                        x: diff.x * multiplier,
                        y: diff.y * multiplier,
                    };

                    let pos1 = antennas[i].position - mul_diff;
                    let pos2 = antennas[j].position + mul_diff;

                    let p1_valid =
                        pos1.x >= 0 && pos1.x < max_x_bound && pos1.y >= 0 && pos1.y < max_y_bound;
                    let p2_valid =
                        pos2.x >= 0 && pos2.x < max_x_bound && pos2.y >= 0 && pos2.y < max_y_bound;

                    if p1_valid {
                        antinodes.insert(pos1);
                    }
                    if p2_valid {
                        antinodes.insert(pos2);
                    }

                    if !p1_valid && !p2_valid {
                        break;
                    }

                    multiplier += 1;
                }
            }
        }

        start_ind = end_ind;
        end_ind = start_ind + 1;
        if start_ind >= antennas.len() {
            break;
        }
        curr_freq = antennas[start_ind].frequency;
    }

    // for row in 0..lines.len() {
    //     for col in 0..lines[row].len() {
    //         if antinodes.contains(&Coordinate {
    //             x: col.try_into().unwrap(),
    //             y: row.try_into().unwrap(),
    //         }) {
    //             print!("#");
    //         } else {
    //             print!("{}", lines[row].chars().nth(col).unwrap());
    //         }
    //     }
    //     println!();
    // }

    println!("{}", antinodes.len());
}
