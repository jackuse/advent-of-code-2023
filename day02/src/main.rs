use regex::Regex;
use shared::read_file;

fn main() {
    let filename = "day02/src/input.txt";
    // let filename = "day02/src/input1_demo.txt";
    let data = read_file(filename);

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

#[derive(Debug)]
struct Turn {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

impl Turn {
    pub fn init(line: &String) -> Self {
        let cubes: Vec<&str> = line.split(",").map(|c| c.trim()).collect();
        // println!("{:?}", cubes);
        Turn {
            red: Turn::get_by_color(&cubes, "red"),
            green: Turn::get_by_color(&cubes, "green"),
            blue: Turn::get_by_color(&cubes, "blue"),
        }
    }

    fn get_by_color(cubes: &Vec<&str>, color: &str) -> usize {
        cubes
            .iter()
            .filter(|c| c.contains(color))
            .next()
            .unwrap_or(&"0")
            .split_whitespace()
            .next()
            .unwrap()
            .to_string()
            .parse::<usize>()
            .unwrap_or(0)
    }
}

#[derive(Debug)]
struct Game {
    pub id: usize,
    pub turns: Vec<Turn>,
}

impl Game {
    pub fn init(line: &String) -> Self {
        let mut game = Game {
            id: 0,
            turns: vec![],
        };

        let reg_game_id = Regex::new(r"Game (\d*):").unwrap();

        game.id = reg_game_id
            .captures(&line)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string()
            .parse()
            .unwrap();

        let parties_line = line.split(":").last().unwrap().to_string();
        let parties = parties_line.split(";").collect::<Vec<&str>>();
        game.turns = parties.iter().map(|p| Turn::init(&p.to_string())).collect();
        // println!("parties {:?}", parties);

        game
    }

    pub fn can_be_win(&self, bag: Turn) -> bool {
        let mut res = true;
        self.turns.iter().for_each(|t| {
            if t.blue > bag.blue || t.red > bag.red || t.green > bag.green {
                res = false
            }
        });
        res
    }

    pub fn power(&self) -> usize{
        let mut min_bag = Turn {blue: 0, red: 0, green: 0};

        self.turns.iter().for_each(|t| {
            if t.red > min_bag.red {
                min_bag.red = t.red
            }
            if t.green > min_bag.green {
                min_bag.green = t.green
            }
            if t.blue > min_bag.blue {
                min_bag.blue = t.blue
            }
        });

        min_bag.red * min_bag.green * min_bag.blue
    }
}

fn part1(input: Vec<String>) -> usize {
    let mut values: Vec<usize> = vec![];
    for line in input {
        let game = Game::init(&line);
        // println!("id {}", game.id);
        // println!("turns {:?}", game.turns);
        if game.can_be_win(Turn {
            red: 12,
            green: 13,
            blue: 14,
        }) {
            values.push(game.id);
        }
    }

    values.into_iter().sum()
}

fn part2(input: Vec<String>) -> usize {
    let mut values: Vec<usize> = vec![];
    for line in input {
        let game = Game::init(&line);
        values.push(game.power());
    }

    values.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_pass_part1_demo() {
        let filename = "src/input1_demo.txt";
        let res = part1(read_file(filename));

        assert_eq!(res, 8);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/input1_demo.txt";
        let res = part2(read_file(filename));

        assert_eq!(res, 2286);
    }
}
