use regex::{Match, Regex};
use shared::read_file;
use std::cmp;
use std::collections::HashMap;

static DAY: &str = "day03/";
pub static DEMO_FILENAME: &str = "src/input1_demo.txt";
pub static PUZZLE_FILENAME: &str = "src/input.txt";

#[derive(Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug)]
struct PartNo {
    pub nb: usize,
    pub start: Point,
    pub end: Point,
}

impl PartNo {
    pub fn init(part_match: regex::Match<'_>, line: usize) -> Self {
        PartNo {
            nb: part_match.as_str().parse().unwrap_or(0),
            start: Point {
                x: line,
                y: part_match.start(),
            },
            end: Point {
                x: line,
                y: part_match.end() - 1,
            },
        }
    }
}

fn main() {
    // let filename = format!("{}{}", DAY, PUZZLE_FILENAME);
    let filename = format!("{}{}", DAY, DEMO_FILENAME);
    let data = read_file(filename.as_str());

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> usize {
    let reg_nb = Regex::new(r"(\d*)").unwrap();

    let mut part_list = vec![];
    let mut matrix = vec![];
    let mut x = 0;
    for line in input {
        let mut m_line = vec![];
        line.chars().for_each(|c| m_line.push(c.to_string()));
        matrix.push(m_line);

        for nb in reg_nb.find_iter(&line) {
            if nb.len() > 0 {
                part_list.push(PartNo::init(nb, x))
            }
        }

        x += 1;
    }
    // println!("{:?}", part_list);

    let mut gear_list : HashMap<String, usize> = HashMap::new();

    let mut part_list_valid = vec![];
    part_list.iter().for_each(|p| {
        let mut is_valid = false;

        let start_x = if p.start.x > 1 {
            p.start.x - 1
        } else {
            p.start.x
        };
        let start_y = if p.start.y > 1 {
            p.start.y - 1
        } else {
            p.start.y
        };
        let end_x = cmp::min(p.end.x + 2, matrix[0].len());
        let end_y = cmp::min(p.end.y + 2, matrix.len());
        for i in start_x..end_x {
            for j in start_y..end_y {
                let char = matrix[i][j].chars().next().unwrap();
                // println!("i/j : {}-{}", i, j);
                // println!("char : {}", char);
                if !char.is_digit(10) && !char.eq(&'.') {
                    if char.eq(&'*') {
                        let key = format!("{}-{}", i, j);
                        if gear_list.contains_key(&key) {
                            let prev = gear_list.get(&key).unwrap();
                            gear_list.insert(key, *prev+ 1);
                        } else{
                            gear_list.insert(key, 1);
                        }
                    }
                    // println!("char : {}", char);
                    is_valid = true;
                }
            }
        }

        println!("gear_list : {:?}", gear_list);

        if is_valid {
            part_list_valid.push(p)
        }
    });

    // println!(
    //     "part_list_valid {:?}",
    //     part_list_valid.iter().map(|p| p.nb).collect::<Vec<usize>>()
    // );

    part_list_valid.iter().map(|p| p.nb).sum()
}

fn part2(input: Vec<String>) -> usize {
    let reg_gear = Regex::new(r"\*").unwrap();
    let reg_nb = Regex::new(r"(\d*)").unwrap();

    let mut part_list = vec![];
    let mut gear_list = vec![];
    let mut matrix = vec![];
    let mut x = 0;
    for line in input {
        let mut m_line = vec![];
        line.chars().for_each(|c| m_line.push(c.to_string()));
        matrix.push(m_line);

        for nb in reg_nb.find_iter(&line) {
            if nb.len() > 0 {
                part_list.push(PartNo::init(nb, x))
            }
        }

        if reg_gear.is_match(&line) {
            let gear = reg_gear.find_iter(&line).next().unwrap();
            gear_list.push(Point {
                x: x,
                y: gear.start(),
            })
        }

        x += 1;
    }
    // println!("{:?}", part_list);
    println!("gear_list {:?}", gear_list);

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/input1_demo.txt";
        let res = part1(read_file(filename));

        assert_eq!(res, 4361);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/input1_demo.txt";
        let res = part2(read_file(filename));

        assert_eq!(res, 467835);
    }
}
