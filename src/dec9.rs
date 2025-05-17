use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9, part1)]
fn get_input_one(input_str: &str) -> Vec<String>{ 
    let mut input: Vec<String> = vec![];
    let mut id_counter = 0;
    for (i, c) in input_str.chars().enumerate() {
        if i % 2 == 0{
            let cnt  = c.to_digit(10).unwrap() as usize;
            let id_num = id_counter.to_string();
            let mut add = vec![id_num; cnt];
            input.append(&mut add);
            id_counter += 1;
        }
        else {
            let cnt  = c.to_digit(10).unwrap() as usize;
            let mut add = vec![".".to_string(); cnt];
            input.append(&mut add);
        }
    }

    return input;
}

#[aoc_generator(day9, part2)]
fn get_input_two(input: &str) -> String{ 
    input.to_string()
}

#[aoc(day9, part1)]
fn dec9_1(input: &Vec<String>) -> u128{

    let mut answer = input.clone();
    let mut change_cnt = 0;
    let len = input.len() - 1;
    for (i, id) in input.iter().enumerate() {
        while answer[len - change_cnt] == ".".to_string() {
            change_cnt += 1;
        }
        if id == &".".to_string() {
            answer.swap(i, len - change_cnt);
            change_cnt += 1;
        }
        if i > len - change_cnt {
            break;
        }
    }
    let mut smth = answer.split(|x| x == &".".to_string());
    let final_an = smth.next().unwrap();
    calc_answer(final_an)
}

#[aoc(day9, part2)]
fn dec9_2(input: &String) -> u128{
    println!("if this works it is actually and genuinely insane lmao");
    let mut input_vec: Vec<usize> = input.chars().map(|x| x.to_digit(10).unwrap() as usize).collect();
    let mut answer_vec: Vec<u128> = vec![];
    let mut id_counter: u128 = 0;
    for (i, c) in input_vec.iter().enumerate() {
        if i % 2 == 0{
            let mut add = vec![id_counter; *c];
            answer_vec.append(&mut add);
            id_counter += 1;
        }
        else {
            let mut add = vec![0; *c];
            answer_vec.append(&mut add);
        }
    }

    //Tuple is (Amount of space taken up, id that occupies it)
    for (id, id_len) in input_vec.clone().iter().enumerate().rev()
                        .filter(|(i, _c)| i % 2 == 0) {
        let num_spot: usize = input_vec[0..id].iter().sum::<usize>(); //the actual index in the answer vector
        let found_empty_spot = input_vec[0..id].iter()
            .enumerate()
            .position(|(i, c)| c >= id_len && i % 2 == 1);

        match found_empty_spot {
            Some(pos) => {
                let empty_spot: usize = input_vec[0..pos].iter().sum::<usize>();
                let mut temp = vec![0; *id_len];
                temp.swap_with_slice(&mut answer_vec[num_spot..num_spot+id_len]);
                temp.swap_with_slice(&mut answer_vec[empty_spot..empty_spot+id_len]);
                input_vec[pos] = input_vec[pos] - id_len;
                input_vec[pos - 1] = input_vec[pos-1] + id_len;
            },
            None => ()
        }
    }
    return calc_answer_two(answer_vec);
}

fn calc_answer_two(ans_string: Vec<u128>) -> u128 {
    return ans_string
        .iter()
        .enumerate()
        .map(|(x, y)| {
            return x as u128 * y;
        })
        .sum::<u128>()
}

fn calc_answer(ans_string: &[String]) -> u128 {
    ans_string.iter()
        .map(|x| {
            if x == &".".to_string() {
                return "0".to_string();
            }
            else {
                return x.clone();
            }
        })
        .enumerate()
        .map(|(x, y)| {
            let smth = x as u128 * y.parse::<u128>().unwrap();
            return smth
        })
        .sum()
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = get_input_one(TEST_INPUT);
        let ans = dec9_1(&lines);
        assert_eq!(1928, ans);
    }

    #[test]
    fn test_part2() {
        let lines = get_input_two(TEST_INPUT);
        let ans = dec9_2(&lines);
        assert_eq!(2858, ans);
    }

    const TEST_INPUT:&str = "2333133121414131402";
}