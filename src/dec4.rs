use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
fn generate_p1(input: &str) -> Vec<Vec<char>>{
    return input.lines().map(|x| x.chars().collect()).collect();
}

#[aoc(day4, part1)]
fn dec4_1(left_to_right: &Vec<Vec<char>>) -> usize {
    let mut up_and_down: Vec<Vec<char>> = transpose(left_to_right.clone());

    let mut all_possibilites: Vec<Vec<char>> = Vec::new();
    all_possibilites.append(&mut left_to_right.clone()); 
    all_possibilites.append(&mut get_diagonals_one(left_to_right.clone()));
    all_possibilites.append(&mut get_diagonals_one(up_and_down.clone()));
    all_possibilites.append(&mut up_and_down);   

    let string_possibilities: Vec<String> = all_possibilites.iter().map(|x| x.iter().collect()).collect();
    let mut answer: usize = string_possibilities.clone()
                                .iter()
                                .map(|x| x.match_indices("XMAS").collect::<Vec<_>>().len()).sum();

    answer += string_possibilities.clone()
                .iter()
                .map(|x| x.match_indices("SAMX").collect::<Vec<_>>().len()).sum::<usize>();
    
    return answer;
}

#[aoc(day4, part2)]
fn dec4_2(left_to_right: &Vec<Vec<char>>) -> i32 {
    let mut answer_cnt = 0;
    for i in 1..left_to_right.len()-1{
        for j in 1..left_to_right.len()-1{
            if left_to_right[i][j] == 'A'{
                if (left_to_right[i+1][j+1] == 'M' && left_to_right[i-1][j-1] == 'S' 
                        || left_to_right[i+1][j+1] == 'S' && left_to_right[i-1][j-1] == 'M')
                    && (left_to_right[i+1][j-1] == 'M' && left_to_right[i-1][j+1] == 'S' 
                        || left_to_right[i+1][j-1] == 'S' && left_to_right[i-1][j+1] == 'M'){
                    answer_cnt += 1;
                }
            }
        }
    }
    return answer_cnt;
}

fn transpose(matrix: Vec<Vec<char>> ) -> Vec<Vec<char>>  {
    let mut up_and_down: Vec<Vec<char>> = Vec::new();
    for (i, line) in matrix.iter().enumerate() {
        for (j, letter) in line.iter().rev().enumerate() {
            if i == 0 {
                let mut n_vec: Vec<char> = Vec::new();
                n_vec.push(*letter);
                up_and_down.push(n_vec);
            }
            else {
                up_and_down[j].push(*letter);
            }
        }
    }
    return up_and_down;
}

fn get_diagonals_one(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut diagonal_above: Vec<Vec<char>> = Vec::new();
    let mut diagonal_below: Vec<Vec<char>> = Vec::new();
    for i in 0..matrix.len() {
        for j in 0..matrix.len()-i {
            if i == 0 {
                let n_vec: Vec<char> = Vec::new();
                diagonal_above.push(n_vec);
            }
            diagonal_above[j].push(matrix[i][i+j])
        }
    }
    for i in 1..matrix.len() {
        for j in 0..i {
            if j == 0 {
                let n_vec: Vec<char> = Vec::new();
                diagonal_below.push(n_vec);
            }
            diagonal_below[i - j - 1].push(matrix[i][j])
        }
    }

    diagonal_above.append(&mut diagonal_below);
    return diagonal_above;
}