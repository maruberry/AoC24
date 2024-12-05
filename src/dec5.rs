use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{Itertools, Position};
use std::cmp::Ordering;

#[aoc_generator(day5)]
fn get_input(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>){
    let mut rules: Vec<(i32, i32)> = Vec::new();
    let mut updates: Vec<Vec<i32>> = Vec::new();

    let (rules_str, updates_str) = input.split("\n\n").next_tuple().unwrap();

    rules = rules_str.lines().map(|x| x.split("|").map(|y| y.parse().unwrap()).next_tuple().unwrap()).collect();
    updates = updates_str.lines().map(|x| x.split(",").map(|y| y.parse().unwrap()).collect()).collect();
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
    for update in input.1.clone() {
        let result = is_sorted_correct(input.0.clone(), update.clone());
        if !result {
            let mut sort_update = sort_correct(input.0.clone(), update.clone());
            sort_update = sort_correct(input.0.clone(), sort_update.clone());
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

fn sort_correct(rules: Vec<(i32, i32)>, mut update: Vec<i32>) -> Vec<i32> {
    update.sort_by(|a, b| {
        let new: Vec<(i32, i32)> = rules.clone().into_iter()
        .filter(|x| (x.0 == *a && x.1 == *b )|| (x.1 == *a && x.0 == *b )).collect();

        if !new.is_empty() {
            if new[0].0 == *a {
                return Ordering::Less;
            }
            else {
                return Ordering::Greater;
            }
        }
        else {
            println!("Why is there no rule for this?");
            return Ordering::Equal;
        }
    });
    return update
}