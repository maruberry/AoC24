use aoc_runner_derive::{aoc, aoc_generator};
use find_all::FindAll;
use itertools::Itertools;

#[aoc_generator(day8)]
fn get_input(input: &str) -> Vec<Vec<char>> { 
   return input.lines().map(|x| x.chars().collect()).collect();
}

#[aoc(day8, part1)]
fn dec8_1(input: &Vec<Vec<char>>) -> usize{
    println!("smth");
    let y_len: i8 = input.len() as i8;
    let x_len: i8 = input[0].len() as i8;
    let uniq_chars: Vec<&char> = input.iter()
                        .flatten()
                        .unique()
                        .filter(|x| x != &&'.')
                        .collect();
    
    let mut new_positions: Vec<(i8, i8)> = vec![];
    for freq in uniq_chars {
        let positions = get_positions(input, freq);
        let combinations: Vec<Vec<&(i8, i8)>> = get_combos(&positions);
        let mut temp = calc_new_pos_one(combinations, x_len, y_len);
        new_positions.append(&mut temp);
    }
    new_positions.sort();
    new_positions.dedup();
    println!("we have {} new positions", new_positions.len());
    //println!("{:?}", new_positions);
    return new_positions.len();
}

#[aoc(day8, part2)]
fn dec8_2(input: &Vec<Vec<char>>) -> usize{
    println!("smth");
    let y_len: i8 = input.len() as i8;
    let x_len: i8 = input[0].len() as i8;
    let uniq_chars: Vec<&char> = input.iter()
                        .flatten()
                        .unique()
                        .filter(|x| x != &&'.')
                        .collect();
    
    let mut new_positions: Vec<(i8, i8)> = vec![];
    for freq in uniq_chars {
        let mut positions = get_positions(input, freq);
        let combinations: Vec<Vec<&(i8, i8)>> = get_combos(&positions);
        let mut temp = calc_new_pos_two(combinations, x_len, y_len);
        new_positions.append(&mut temp);
        if positions.len() > 1 {
            new_positions.append(&mut positions);
        }
    }
    new_positions.sort();
    new_positions.dedup();
    println!("we have {} new positions", new_positions.len());
    //println!("{:?}", new_positions);
    return new_positions.len();
}

fn get_combos(positions: &Vec<(i8, i8)>) -> Vec<Vec<&(i8, i8)>> {
    let combinations: Vec<Vec<&(i8, i8)>> = positions
            .iter()
            .combinations(2)
            .collect();

    combinations
}

fn get_positions(input: &Vec<Vec<char>>, freq: &char) -> Vec<(i8, i8)> {
    let mut freq_pos: Vec<(i8, i8)> = vec![];
    input.iter()
        .enumerate()
        .for_each(|(i, arr)| {
            let pos = arr
                .iter()
                .find_all(|x| x == &freq);
            match pos {
                Some(x) => x.iter().for_each(|j| freq_pos.push((i as i8, *j as i8))),
                None => ()
            }                   
        });
    freq_pos
}

fn calc_new_pos_one(combinations: Vec<Vec<&(i8, i8)>> , x_len: i8, y_len: i8) -> Vec<(i8, i8)> {
    let mut new_pos: Vec<(i8, i8)> = vec![];
    for combo in combinations {
        let a = combo[0];
            let b = combo[1];
            let y_diff = a.0 - b.0;
            let x_diff = a.1 - b.1;
            let ypos_one = a.0 + y_diff;
            let xpos_one = a.1 + x_diff;
            if ypos_one < y_len && ypos_one >= 0 && xpos_one < x_len && xpos_one >= 0 {
                new_pos.push((ypos_one, xpos_one));
            }
            let ypos_two = b.0 - y_diff;
            let xpos_two = b.1 - x_diff;
            if ypos_two < y_len && ypos_two >= 0 && xpos_two < x_len && xpos_two >= 0 {
                new_pos.push((ypos_two, xpos_two));
            };
    }
    new_pos
}

fn calc_new_pos_two(combinations: Vec<Vec<&(i8, i8)>> , x_len: i8, y_len: i8) -> Vec<(i8, i8)> {
    let mut new_pos: Vec<(i8, i8)> = vec![];
    for combo in combinations {
        let a = combo[0];
            let b = combo[1];
            let y_diff = a.0 - b.0;
            let x_diff = a.1 - b.1;
            let mut ypos_one = a.0 + y_diff;
            let mut xpos_one = a.1 + x_diff;
            loop {
                if ypos_one < y_len && ypos_one >= 0 && xpos_one < x_len && xpos_one >= 0 {
                    new_pos.push((ypos_one, xpos_one));
                } 
                else {
                    break;
                }
                ypos_one = ypos_one + y_diff;
                xpos_one = xpos_one + x_diff;  
            } 
            let mut ypos_two = b.0 - y_diff;
            let mut xpos_two = b.1 - x_diff;
            loop {
                if ypos_two < y_len && ypos_two >= 0 && xpos_two < x_len && xpos_two >= 0 {
                    new_pos.push((ypos_two, xpos_two));
                }
                else {
                    break;
                }
                ypos_two = ypos_two - y_diff;
                xpos_two = xpos_two - x_diff;  
            } 
    }
    new_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let lines = get_input(TEST_INPUT);
        let ans = dec8_1(&lines);
        assert_eq!(14, ans);
    }

    #[test]
    fn test_part2() {
        let lines = get_input(TEST_INPUT);
        let ans = dec8_2(&lines);
        assert_eq!(34, ans);
    }

    const TEST_INPUT:&str =
"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

}