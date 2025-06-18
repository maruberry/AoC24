use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day14)]
fn get_input(input: &str) -> Map{
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let (mut pos, mut vel) = line.split_once(' ').unwrap();
        pos = pos.trim_start_matches("p=");
        let (posx, posy) = pos.split_once(',').map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())).unwrap();
        vel = vel.trim_start_matches("v=");
        let (velx, vely) = vel
            .split_once(',').map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap())).unwrap();
        robots.push(Robot{posx, posy, velx, vely})
    }
    let quads = Quads::new();
    Map{robots, quads}
}

#[derive(Clone, PartialEq, Debug)]
struct Robot {
    posx: i32,
    posy: i32,
    velx: i32,
    vely: i32
}

impl Robot {
    fn update_pos(self) -> Robot{
        let mut new = self.clone();
        new.posx = ((self.posx + self.velx) % WIDE + WIDE) % WIDE;
        new.posy = ((self.posy + self.vely) % TALL + TALL) % TALL; 
        new
    } 
}

#[derive(Clone, PartialEq)]
struct Map {
    robots: Vec<Robot>,
    quads: Quads
}

impl Map {
    fn draw(&self) -> Vec<Vec<char>>{
        let mut map = vec![vec!['.'; WIDE as usize]; TALL as usize];
        for robot in self.robots.clone() {
            map[robot.posy as usize][robot.posx as usize] = 'x';
        }
        return map;
    }

    fn update_pos(&mut self) {
        self.robots = self.robots.clone().into_iter().map(|r| r.update_pos()).collect();
    }

    fn calc_score(&mut self) -> i32{
        let map = self.draw();
        let mut score: i32 = 0;
        for y in 1..TALL-1 {
            for x in 1..WIDE-1 {
                let cnt = Map::count_neighbours(&map, x, y);
                if cnt == 0 {
                    score += -2;
                }
                else if cnt == 1 {
                    score += 0;
                }
                else if cnt > 1 {
                    score += 2;
                }
            }
        } 

        return score;   
    }

    fn count_neighbours(map: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {
        if map[y as usize][x as usize] == '.' {
            return -5;
        }
        let change_one:Vec<i32>  = vec![0, 1, 0, -1, 1, -1, -1, 1];
        let change_two:Vec<i32> = vec![1, 0,-1, 0, -1, 1, -1, 1];
        let mut num = 0;
        for (a, b) in change_one.iter().zip(change_two) {

            if map[y as usize][x as usize] == map[(y + a) as usize][(x + b) as usize]{
                num += 1;
            }
        }
        return num
    }
}

#[derive(Clone, PartialEq)]
struct Quads {
    ne: usize,
    se: usize,
    sw: usize,
    nw: usize,
}

impl Quads {
    fn new() -> Self {
        Self{ne: 0, se: 0, sw: 0, nw: 0}
    }

    fn add(&mut self, x: i32, y: i32) {
        let vert = WIDE/2;
        let hor = TALL/2;
        if x > vert && y < hor {
            self.ne += 1;
            //println!("{} {} is in ne {}", x, y, self.ne);
        }
        else if x > vert && y > hor {
            self.se += 1;
            //println!("{} {} is in se {}", x, y, self.se);
        }
        else if x < vert && y < hor {
            self.nw += 1;
            //println!("{} {} is in nw {}", x, y, self.nw);
        }
        else if x < vert && y > hor {
            self.sw += 1;
            //println!("{} {} is in sw {}", x, y, self.sw);
        }
    }
}

const WIDE: i32 = 101;
const TALL: i32 = 103;
const STEPS: i32 = 100;

#[aoc(day14, part1)]
fn dec14_1(input: &Map) -> usize{
    let mut quads = input.quads.clone();
    for robot in input.robots.clone() {
        //println!("Robot x: {}, y: {}, velx {}, vely {}", robot.posx, robot.posy, robot.velx, robot.vely);
        let tempx = robot.posx + robot.velx * STEPS;
        let tempy = robot.posy + robot.vely * STEPS;
        let x = (tempx % WIDE + WIDE) % WIDE; 
        let y = (tempy % TALL + TALL) % TALL; 
        quads.add(x, y);
    }

    quads.ne * quads.nw * quads.se * quads.sw
}

#[aoc(day14, part2)]
fn dec14_2(input: &Map) -> i32{
    let mut map = input.clone();
    //let mut quads = input.quads.clone();
    //let mut robots = input.robots.clone();
    let mut count = 0;
    loop {
        count += 1;
        map.update_pos();
        let sc = map.calc_score();
        if sc > 0 {
            map.draw(); 
            println!("ENDING {count} with score {}", sc);
            return count;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_filtered = get_input(SMALL_INPUT);
        let ans = dec14_1(&input_filtered);
        assert_eq!(12, ans);
    }

    #[test]
    fn test_part2() {
        let input_filtered = get_input(SMALL_INPUT);
        dec14_2(&input_filtered);
    }

    const SMALL_INPUT:&str = 
"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
"; 
}

