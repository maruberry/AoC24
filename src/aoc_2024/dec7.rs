use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn get_input(input: &str) -> Vec<Vec<u64>> { 
    let mut output: Vec<Vec<u64>> = Vec::new();
    let no_colon = input.replace(":", "");
    for line in no_colon.lines() {
        let split_line: Vec<u64> = line.split(" ")
                    .map(|x| x.parse::<u64>()
                    .expect("This is not a number"))
                    .collect();
        output.push(split_line)
    }
    return output;
}

#[aoc(day7, part1)]
fn dec7_1(input: &Vec<Vec<u64>>) -> u64{
    let mut final_answer = 0u64;
    for calc_num in input.iter() {
        let operations = calc_num.len() - 2;
        let mut counter = vec![0; operations];
        loop {
            let temp = add_n_multiply(&calc_num[1..].to_vec(), counter.clone());
            //println!("temp: {temp}, answer: {}", calc_num[0]);
            if temp == calc_num[0] {
                final_answer += calc_num[0];
                break;
            }
            if counter.iter().all(|x| x == &1) {
                break;
            }
            counter = array_countup(counter.clone(), operations, &1);
        }  
    }
    println!("Final answer is {final_answer}");
    return final_answer;
}

#[aoc(day7, part2)]
fn dec7_2(input: &Vec<Vec<u64>>) -> u64{
    let mut final_answer = 0u64;
    for calc_num in input.iter() {
        let operations = calc_num.len() - 2;
        let mut counter = vec![0; operations];
        loop {
            let temp = add_n_multiply(&calc_num[1..].to_vec(), counter.clone());
            if temp == calc_num[0] {
                final_answer += calc_num[0];
                break;
            }
            if counter.iter().all(|x| x == &2) {
                break;
            }
            counter = array_countup(counter.clone(), operations, &2);
        }  
    }
    println!("Final answer is {final_answer}");
    return final_answer;
}

fn add_n_multiply(calc_num: &Vec<u64>, bin_count: Vec<u64>) -> u64{
    if calc_num.len() == 1{
        return calc_num[0];
    }
    let mut temp_answer = calc_num[0];
    for (i, elem) in calc_num[1..].iter().enumerate(){
        let bit = bin_count[i];
        match bit {
            1 => temp_answer *= elem,
            0 => temp_answer += elem,
            2 => {
                let a = temp_answer.to_string();
                let b = elem.to_string();
                let new = a + &b;
                temp_answer = new.parse::<u64>().unwrap();
            },
            _ => println!("Something went wrong, unknown bit")
        }
    }
    return temp_answer 
}

fn array_countup(mut input: Vec<u64>, len: usize, count_to: &u64) -> Vec<u64>{
    for (i,elem) in input.clone().iter().rev().enumerate(){
        if elem < count_to{
            input[len-1-i] += 1;
            break;
        }
        else if elem == count_to{
            input[len-1-i] = 0;
        }
    }
    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ternary_count() {
        let input: Vec<u64> = vec![0, 0, 3, 3, 3, 3];
        let len = input.len();
        let result = array_countup(input, len, &3);
        assert_eq!(result, vec![0, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn binary_count() {
        let input: Vec<u64> = vec![0, 0, 2, 2, 2, 2];
        let len = input.len();
        let result = array_countup(input, len, &2);
        assert_eq!(result, vec![0, 1, 0, 0, 0, 0]);
    }
    const INPUT_TEST: &str = 
"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    #[test]
    fn part_1() {
        let parse_input = get_input(INPUT_TEST);
        let answer = dec7_1(&parse_input);
        assert_eq!(3749, answer);
    }

    #[test]
    fn part_2() {
        let parse_input = get_input(INPUT_TEST);
        let answer = dec7_2(&parse_input);
        assert_eq!(11387, answer);
    }
}