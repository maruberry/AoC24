use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6, part1)]
fn get_input_1(input: &str) -> (Vec<Vec<u128>>, Vec<char>) {
    let mut lines: Vec<&str> = input.lines().collect();
    let symbols = lines.pop().unwrap().chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
    let nums: Vec<Vec<u128>> = lines.iter().map(|l| l.split_whitespace().map(|w| w.parse().unwrap()).collect()).collect();

    (nums, symbols)
}


//8342588838713 too low
//
#[aoc_generator(day6, part2)]
fn get_input_2(input: &str) -> (Vec<Vec<u128>>, Vec<char>) {
    println!("input {:#?}", input);
    let mut lines: Vec<&str> = input.split(|f| f == '\n').collect();
    println!("lines {:#?}", lines);
    let symbol_line = lines.pop().unwrap();
    //let symbol_line = &(symbol_line_init.to_string() + " ");
    let symbols_whitespace: Vec<&str> = symbol_line[1..].split(|l: char| !l.is_whitespace()).collect::<Vec<_>>();
    let symbols = symbol_line.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>();
    println!("{:#?}", symbols_whitespace);
    println!("length {}", symbols_whitespace.len());
    println!("lines {:#?}", lines);
    let nums: Vec<Vec<&str>> = lines.iter()
        .map(|l| {
            let mut temp = vec![];
            let mut cut = 0;
            for n in symbols_whitespace[..symbols_whitespace.len() - 1].iter() {
                //println!("{}", &l[cut..cut + n.len()]);
                temp.push(&l[cut..cut + n.len()]);
                cut += n.len() + 1;
            }
            temp
        }).collect();
    //println!("{:#?}", nums);
    let mut nums_correct: Vec<Vec<u128>> = vec![];
    for i in 0..nums[1].len() {
        let mut temp_str = vec!["".to_string(); lines.len()];
        for j in 0..nums.len() {
            for (r, c) in nums[j][i].chars().enumerate() {
                temp_str[r] += &c.to_string();
            }
        }
        println!("{:?}", temp_str);
        nums_correct.push(temp_str.iter().filter(|s| s.len() > 0).map(|s| s.trim().parse::<u128>().unwrap()).collect());
    }

    (nums_correct, symbols)
}

#[aoc(day6, part1)]
pub fn dec6_1(input: &(Vec<Vec<u128>>, Vec<char>)) -> u128 {
    let mut answer = 0;
    let (nums, symbols) = input;

    for i in 0..nums[0].len() {
        let mut temp = 0;
        let multiply = symbols[i] == '*';
        if multiply {
            temp = 1;
        }
        for j in 0..nums.len() {
            if multiply {
                temp *= nums[j][i];
            } else {
                temp += nums[j][i];
            }
        }
        answer += temp;
    }

    answer
}

#[aoc(day6, part2)]
pub fn dec6_2(input: &(Vec<Vec<u128>>, Vec<char>)) -> u128 {
    let mut answer = 0;
    let (nums, symbols) = input;

    println!("\nPROBLEMS \n");
    for (i, problem) in nums.iter().enumerate() {
        println!("{} : {:?}", symbols[i], problem);
        if symbols[i] == '*' {
            answer += problem.iter().product::<u128>();
        } else {
            answer += problem.iter().sum::<u128>();
        }
    }

    answer
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_TEST: &str =
    "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +   ";
    #[test]
    fn part_1() {
        let parse_input = get_input_1(INPUT_TEST);
        let answer = dec6_1(&parse_input);
        assert_eq!(4277556, answer);
    }

    #[test]
    fn part_2() {
        let parse_input = get_input_2(INPUT_TEST);
        let answer = dec6_2(&parse_input);
        assert_eq!(3263827, answer);
    }
}