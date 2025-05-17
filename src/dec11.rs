use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[aoc_generator(day11)]
fn get_input(input: &str) -> Vec<String>{ 
    return input.split(' ').map(|s| s.to_string()).collect();
    
}

#[aoc(day11, part1)]
fn dec11_1(input: &Vec<String>) -> usize{
    let mut known: HashMap<u128, Next> = HashMap::new(); //Key = num, Value = how many next and what is in there
    let mut layers: HashMap<u128, usize> = HashMap::new();
    
    for stone in input {
        let parsed: u128 = stone.parse().unwrap();
        layers.insert(parsed, 1);
    }
    let l = calc_on(&mut known, layers, 0, 25);
    l.iter().map(|(_n, cnt)| *cnt).sum()
}


struct Next {
    n_nums: Vec<u128>,
    how_many: usize,
}

#[aoc(day11, part2)]
fn dec11_2(input: &Vec<String>) -> usize{
    //List of all our known numbers
    let mut known: HashMap<u128, Next> = HashMap::new(); //Key = num, Value = how many next and what is in there
    let mut layers: HashMap<u128, usize> = HashMap::new();
    
    for stone in input {
        let parsed: u128 = stone.parse().unwrap();
        layers.insert(parsed, 1);
    }
    let l = calc_on(&mut known, layers, 0, 75);
    l.iter().map(|(_n, cnt)| *cnt).sum()
}

fn calc_on(known: &mut HashMap<u128, Next>, layer_map: HashMap<u128, usize>, layer: usize, stop_at: usize) -> HashMap<u128, usize>{
    if layer == stop_at {return layer_map;}
    let mut nlayer: HashMap<u128, usize> = HashMap::new();
    //println!("current layer {:?}", layers[layer]);
    for (val, cnt) in layer_map {
        let next = &known.entry(val).or_insert(calc_next(val)).n_nums;
        for nval in next {
            let heh = nlayer.entry(*nval).or_insert(0);
            *heh += cnt;
        } 
    }
    calc_on(known, nlayer, layer + 1, stop_at)
}

fn calc_next(num: u128) -> Next{
    if num == 0u128 {
        return Next {
            n_nums: vec![1],
            how_many: 0,
        }
    }
    else if num.to_string().len() % 2 == 0 {
        let str = num.to_string();
        let (a, mut b) = str.split_at(str.len()/2);
        b = b.trim_start_matches('0');
        if b == "" { b = "0"}
        return Next {
            n_nums: vec![a.parse().unwrap(), b.parse().unwrap()],
            how_many: 0,
        }
    }
    else {
        //println!("trying to multiply with {}", answer[i]);
        return Next {
            n_nums: vec![num * 2024u128],
            how_many: 0,
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_filtered = get_input(TEST_INPUT);
        let ans = dec11_1(&input_filtered);
        assert_eq!(55312, ans);
    }

    const TEST_INPUT:&str = "125 17";
}