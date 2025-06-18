use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day15, part1)]
fn get_input(input: &str) -> Map {
    let (map_str, mut algorithm_str) = input.split_once("\n\n").unwrap();
    algorithm_str = algorithm_str.trim_ascii();
    algorithm_str = algorithm_str.trim();
    let algorithm: Vec<Order> = algorithm_str.chars().map(|x| Order::new(x)).collect();
    let map: Vec<Vec<char>> = map_str.lines().map(|l| l.chars().collect()).collect();
    Map{map, algorithm, x:0, y:0}
}

#[aoc_generator(day15, part2)]
fn get_input_two(input: &str) -> Map {
    let (map_str, mut algorithm_str) = input.split_once("\n\n").unwrap();
    algorithm_str = algorithm_str.trim_ascii();
    algorithm_str = algorithm_str.trim();
    let algorithm: Vec<Order> = algorithm_str.chars().map(|x| Order::new(x)).collect();
    let map: Vec<Vec<char>> = map_str.lines()
            .map(|l| l
                .chars().map(|c| {
                    if c == 'O' {return "[]"}
                    else if c == '.' {return ".."}
                    else if c == '#' {return "##"}
                    else  {return "@."}
                })
                .collect::<String>().chars().collect())
            .collect();
    Map{map, algorithm, x:0, y:0}
}

struct Map {
    map: Vec<Vec<char>>,
    algorithm: Vec<Order>,
    x: i32, //x for robot rn
    y: i32  //y for robot rn
}

impl Map {
    fn find_start(&mut self) {
        for (i, line) in self.map.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '@' {self.x = j as i32; self.y = i as i32; return;}
            }
        }
    }

    fn _draw(&self) {
        for line in &self.map {
            let l:String = line.iter().collect();
            println!("{l}");
        }
    }

    fn moving(&mut self, xchange: i32, ychange: i32) {
        let nx = (xchange + self.x) as usize;
        let ny = (ychange + self.y) as usize;
        if self.map[ny][nx] == '#' {return}
        else if self.map[ny][nx] == '.' {self.move_robot(nx, ny)}
        else if self.map[ny][nx] == 'O' {self.find_o_end(xchange, ychange);}
    }

    fn moving_two(&mut self, xchange: i32, ychange: i32) {
        let nx = (xchange + self.x) as usize;
        let ny = (ychange + self.y) as usize;
        if self.map[ny][nx] == '#' {return}
        else if self.map[ny][nx] == '.' {self.move_robot(nx, ny)}
        else if self.map[ny][nx] == '[' || self.map[ny][nx] == ']' {self.boxes_chain(xchange, ychange);}
    }

    fn boxes_chain(&mut self, xchange: i32, ychange: i32) {
        let mut x = (self.x + xchange) as usize;
        let mut y = (self.y + ychange) as usize;
        let mut loc: Vec<(usize, usize)> = vec![];
        let mut move_these: Vec<(usize, usize)> = vec![];
        move_these.push((self.x as usize, self.y as usize));
        loc.push((x, y));
        while loc.len() != 0 {
            let (nx, ny) = loc.pop().unwrap();
            if self.map[ny][nx] == '#' {return}
            else if self.map[ny][nx] == '[' {
                if !move_these.contains(&(nx + 1, ny)) {loc.push((nx + 1, ny))}
                move_these.push((nx, ny)); 
                y = (ny as i32 + ychange) as usize;
                x = (nx as i32 + xchange) as usize;
                loc.push((x, y));
            }
            else if self.map[ny][nx] == ']' {
                if !move_these.contains(&(nx - 1, ny)) {loc.push((nx - 1, ny))}
                move_these.push((nx, ny));
                y = (ny as i32 + ychange) as usize;
                x = (nx as i32 + xchange) as usize;
                loc.push((x, y));
            }
        }
        let og = self.map.clone();
        for (fx, fy) in move_these.clone().into_iter().rev() {
            let ny = (fy as i32 + ychange) as usize;
            let nx = (fx as i32 + xchange) as usize;
            let py = (fy as i32 - ychange) as usize;
            let px = (fx as i32 - xchange) as usize;
            if move_these.contains(&(px, py)) {self.map[fy][fx] = og[py][px]}
            else {self.map[fy][fx] = '.'}
            self.map[ny][nx] = og[fy][fx];
        }
        self.x = self.x + xchange;
        self.y = self.y + ychange;
    }

    fn find_o_end(&mut self, xchange: i32, ychange: i32) {
        let mut x = (self.x + xchange) as usize;
        let mut y = (self.y + ychange) as usize;
        while self.map[y][x] == 'O' {
            y = (y as i32 + ychange) as usize;
            x = (x as i32 + xchange) as usize;
        }
        if self.map[y][x] == '#' {return}
        else if self.map[y][x] == '.' {
            while self.map[y][x] != '@' {
                let ny = (y as i32 - ychange) as usize;
                let nx = (x as i32 - xchange) as usize;
                self.map[y][x] = self.map[ny][nx];
                y = ny;
                x = nx; 
            }
            self.map[self.y as usize][self.x as usize] = '.';
            self.y = self.y + ychange;
            self.x = self.x + xchange;
        }
    }

    fn move_robot(&mut self, x: usize, y: usize) {
        self.map[self.y as usize][self.x as usize] = '.';
        self.map[y][x] = '@';
        self.x = x as i32;
        self.y = y as i32;
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Order {
    ud: i32, //up-down
    lr: i32, //left-right
}

impl Order {
    fn new(symbol: char) -> Self {
        match symbol {
            '<' => Self{lr: -1, ud: 0},
            '^' => Self{lr: 0, ud: -1},
            'v' => Self{lr: 0, ud: 1},
            '>' => Self{lr: 1, ud: 0},
            _ => Self{lr: 0, ud: 0},
        }
    }
}


#[aoc(day15, part1)]
fn dec15_1(input: &Map) -> usize{
    let mut guard = Map{map: input.map.clone(), algorithm: input.algorithm.clone(), x: input.x, y: input.y};
    guard.find_start();
    for m in input.algorithm.iter() {
        guard.moving(m.lr, m.ud);
    }
    let mut answer: usize = 0;
    for (i, line) in guard.map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == 'O' {answer += 100 * i + j}
        }
    }
    answer
}

#[aoc(day15, part2)]
fn dec15_2(input: &Map) -> usize{
    let mut guard = Map{map: input.map.clone(), algorithm: input.algorithm.clone(), x: input.x, y: input.y};
    guard.find_start();
    for m in input.algorithm.iter() {
        guard.moving_two(m.lr, m.ud);
    }
    let mut answer: usize = 0;
    for (i, line) in guard.map.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == '[' {answer += 100 * i + j}
        }
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_two_large() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day15_large").unwrap();
        let input_filtered = get_input_two(&test_input);
        let ans = dec15_2(&input_filtered);
        assert_eq!(9021, ans);
    }

    #[test]
    fn test_large() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day15_large").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec15_1(&input_filtered);
        assert_eq!(10092, ans);
    }

    #[test]
    fn test_small() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day15_small").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec15_1(&input_filtered);
        assert_eq!(2028, ans);
    }
}