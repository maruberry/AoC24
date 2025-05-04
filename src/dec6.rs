use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
#[derive(Clone)]
enum Dir {
    DirNum(usize)
}
#[derive(Clone)]
enum Adjust {
    DIR(i32, i32)
}

#[aoc_generator(day6)]
fn get_input(input: &str) -> Vec<Vec<char>> { 
    let mut output: Vec<Vec<char>> = input.lines()
                            .map(|x| x.chars().collect())
                            .map(|mut x: Vec<char>| {
                                x.insert(0, 'O');
                                x.push('O');
                                return x;
                            })
                            .collect();
    let len = output[0].len();
    let filler: Vec<char> = vec!['O'; len];
    output.push(filler.clone());
    output.insert(0, filler);
    
    return output;
}

#[aoc(day6, part1)]
fn dec6_1(input: &Vec<Vec<char>>) -> usize {
    let pos = find_beginning(input.clone());
    println!("We are located at {} {}, direction is {:?}", pos.0, pos.1, pos.2);
    let mov = vec![Adjust::DIR(-1, 0), Adjust::DIR(0, -1), Adjust::DIR(1, 0), Adjust::DIR(0, 1)];
    let fin_map = movement(input.clone(), pos, mov);
    let answer: usize = fin_map.iter().map(|l| l.iter().filter(|n| **n == 'A').count()).sum();
    return answer;
}

#[aoc(day6, part2)]
fn dec6_2(input: &Vec<Vec<char>>) -> i32 {
    let pos = find_beginning(input.clone());
    println!("We are located at {} {}, direction is {:?}", pos.0, pos.1, pos.2);
    let mov = vec![Adjust::DIR(-1, 0), Adjust::DIR(0, -1), Adjust::DIR(1, 0), Adjust::DIR(0, 1)];
    let (_fin_map, answer) = movement_two(input.clone(), pos, mov, 0);
    return answer;
}

fn find_loop(map: Vec<Vec<char>>, pos: (usize, usize, Dir), mov: Vec<Adjust>) -> (Vec<Vec<char>>, bool) {
    let Dir::DirNum(dir_num) = pos.2;
    let Adjust::DIR(x, y) = mov[dir_num];
    let x_change: usize = (pos.0 as i32 + x).try_into().unwrap();
    let y_change: usize = (pos.1 as i32 + y).try_into().unwrap();

    if map[y_change][x_change] == 'O' {
        return (map, false);
    }
    else if map[y_change][x_change] == '#' {
        if map[pos.1][pos.0] == 'T' {
            return (map, true);
        }

        let nnum = (dir_num + 1) % 4;
        let ndir = Dir::DirNum(nnum);
        return find_loop(map, (pos.0, pos.1, ndir), mov);
    }
    else {
        return find_loop(map, (x_change, y_change, Dir::DirNum(dir_num)), mov);
    }
}

fn movement_two(mut map: Vec<Vec<char>>, pos: (usize, usize, Dir), mov: Vec<Adjust>, mut block_counter: i32) -> (Vec<Vec<char>>, i32) {
    let Dir::DirNum(dir_num) = pos.2;
    let Adjust::DIR(x, y) = mov[dir_num];
    let x_change: usize = (pos.0 as i32 + x).try_into().unwrap();
    let y_change: usize = (pos.1 as i32 + y).try_into().unwrap();

    let nnum = (dir_num + 1) % 4;
    let ndir = Dir::DirNum(nnum);
    let mut smth: bool = false;
    (map, smth) = find_loop(map, (pos.0.clone() ,pos.1.clone(), ndir.clone()), mov.clone());
    if smth {
        block_counter += 1;
    }

    if map[y_change][x_change] == 'O' {
        return (map, block_counter);
    }
    else if map[y_change][x_change] == '#' {
        map[pos.1][pos.0] = 'T';
        return movement_two(map, (pos.0, pos.1, ndir), mov, block_counter);
    }
    else {
        return movement_two(map, (x_change, y_change, Dir::DirNum(dir_num)), mov, block_counter);
    }
}

fn movement(mut map: Vec<Vec<char>>, pos: (usize, usize, Dir), mov: Vec<Adjust>) -> Vec<Vec<char>> {
    let Dir::DirNum(dir_num) = pos.2;
    let Adjust::DIR(x, y) = mov[dir_num];
    let x_change: usize = (pos.0 as i32 + x).try_into().unwrap();
    let y_change: usize = (pos.1 as i32 + y).try_into().unwrap();

    if map[y_change][x_change] == 'O' {
        map[pos.1][pos.0] = 'A';
        return map;
    }
    else if map[y_change][x_change] == '#' {
        map[pos.1][pos.0] = 'A';
        let nnum = (dir_num + 1) % 4;
        let ndir = Dir::DirNum(nnum);
        return movement(map, (pos.0, pos.1, ndir), mov);
    }
    else {
        map[pos.1][pos.0] = 'A';
        return movement(map, (x_change, y_change, Dir::DirNum(dir_num)), mov);
    }
}

fn find_beginning(input: Vec<Vec<char>>) -> (usize, usize, Dir){
    for (i, line) in input.iter().enumerate() {
        if line.contains(&'<') {
            return (line.iter().position(|x| *x == '<').unwrap(), i, Dir::DirNum(0));
        }
        else if  line.contains(&'>') {
            return (line.iter().position(|x| *x == '>').unwrap(), i, Dir::DirNum(2));
        }
        else if  line.contains(&'^')  {
            return (line.iter().position(|x| *x == '^').unwrap(), i, Dir::DirNum(1));
        }
        else if  line.contains(&'v') {
            return (line.iter().position(|x| *x == 'v').unwrap(), i, Dir::DirNum(3));
        }
    }
    println!("We could not find the position?");
    return (0, 0, Dir::DirNum(0));
}