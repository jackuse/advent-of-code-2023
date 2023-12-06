use shared::read_file;

fn main() {
    let filename = "day01/src/input.txt";
    // let filename = "day01/src/input1_demo.txt";
    let data = read_file(filename);

    let res = part1(data.clone());
    println!("PART 1 : {}", res);

    let res = part2(data.clone());
    println!("PART 2 : {}", res);
}

fn part1(input: Vec<String>) -> u32 {
    let mut values: Vec<u32> = vec![];
    for line in input {
        let all_nb: String = line.chars().filter(|c| c.is_numeric()).collect();
        let nb = format!(
            "{}{}",
            all_nb.chars().next().unwrap_or(' '),
            all_nb.chars().last().unwrap_or(' ')
        );
        values.push(nb.parse::<u32>().unwrap_or(0));
    }

    values.into_iter().sum()
}
fn part2(input: Vec<String>) -> u32 {
    let mut values: Vec<u32> = vec![];
    for line in input {
        let line2 = line
            .replace("oneight", "18")
            .replace("twone", "21")
            .replace("threeight", "38")
            .replace("fiveight", "58")
            .replace("sevenine", "79")
            .replace("eightwo", "82")
            .replace("eighthree", "83")
            .replace("nineight", "98")
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");
        // println!("line2 : {}", line2);
        let all_nb: String = line2.chars().filter(|c| c.is_numeric()).collect();
        let nb = format!(
            "{}{}",
            all_nb.chars().next().unwrap(),
            all_nb.chars().last().unwrap()
        );
        // println!("nb : {}", nb);
        values.push(nb.parse::<u32>().unwrap());
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

        assert_eq!(res, 142);
    }

    #[test]
    fn should_pass_part2_demo() {
        let filename = "src/input2_demo.txt";
        let res = part2(read_file(filename));

        assert_eq!(res, 281);
    }
}
