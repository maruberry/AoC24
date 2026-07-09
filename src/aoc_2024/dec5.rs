use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day5)]
fn get_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>){
    let (rules_str, updates_str) = input.split("\n\n").next_tuple().unwrap();

    let rules = rules_str.lines().map(|x| x.split("|").map(|y| y.parse().unwrap()).next_tuple().unwrap()).collect();
    let updates = updates_str.lines().map(|x| x.split(",").map(|y| y.parse().unwrap()).collect()).collect();
    return (rules, updates);
}

#[aoc(day5, part1)]
fn dec5_1(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32{ 
    let mut answer: i32 = 0;
    for update in input.1.clone() {
        let result = is_sorted_correct(input.0.clone(), update.clone());
        if result {
            answer += update[(update.len()-1)/2];
        }
    }

    return answer;
}

#[aoc(day5, part2)]
fn dec5_2(input: &(Vec<(i32, i32)>, Vec<Vec<i32>>)) -> i32{ 
    let mut answer: i32 = 0;
    let sorted_rules = input.0.clone();
    //sorted_rules.sort_by(|a,b| (a.0).cmp(&b.0));
    for update in input.1.clone() {
        let result = is_sorted_correct(sorted_rules.clone(), update.clone());
        if !result {
            let mut sort_update = sort_correct(sorted_rules.clone(), update.clone());
            sort_update = sort_correct(sorted_rules.clone(), sort_update.clone());
            answer += sort_update[(sort_update.len()-1)/2];
        }
    }

    return answer;
}

fn is_sorted_correct(rules: Vec<(i32, i32)>, update: Vec<i32>) -> bool{
    for rule in &rules {        
        //println!("RULE {} {}", rule.0, rule.1);
        if update.contains(&rule.0) && update.contains(&rule.1) {
            if update.iter().position(|r| *r == rule.0) > update.iter().position(|r| *r == rule.1) {
                return false;
            }
        }
    }

    return true;
}

fn sort_correct(rules: Vec<(i32, i32)>, update: Vec<i32>) -> Vec<i32> {
    let mut new = update.clone();
    for rule in &rules {        
        if new.contains(&rule.0) && new.contains(&rule.1) {
            let pos1 = new.iter().position(|r| *r == rule.0).expect("Not here");
            let pos2 = new.iter().position(|r| *r == rule.1).expect("There is nothing here");
            if pos1 > pos2 {
                //println!("LINE {:?} RULE {:?}", new, rule);
                let temp = new[pos2];
                new.insert(pos1 + 1, temp);
                new.remove(pos2);
            }
        }
    }
    //println!("FINAL LINE {:?}", new);
    return new;
}