use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use pathfinding::prelude::{dijkstra, astar_bag};

#[aoc_generator(day16)]
fn get_input(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|x| x.chars()
            .collect())
        .collect()
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos{x: i32, y: i32}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {UP, DOWN, LEFT, RIGHT}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Fpos(Pos, Dir);

impl Dir {
    fn get_loc(&self) -> (i32, i32){
        match self {
            Dir::UP => (0, -1),
            Dir::DOWN => (0, 1),
            Dir::RIGHT => (1, 0),
            Dir::LEFT => (-1, 0)
        }
    }

    fn opposite(&self) -> Dir {
        match self {
            Dir::UP => Dir::DOWN,
            Dir::DOWN => Dir::UP,
            Dir::RIGHT => Dir::LEFT,
            Dir::LEFT => Dir::RIGHT
        }
    }
}

impl Fpos {

    fn distance(&self, goal: &Pos) -> usize {
        let xdiff = self.0.x.abs_diff(goal.x) as usize;
        let ydiff = self.0.y.abs_diff(goal.y) as usize;

        if xdiff != 0 && ydiff != 0 {
            return xdiff + ydiff + 1000;
        }
        else {
            return xdiff + ydiff;
        }
    }

    fn successors(&self, map: &Vec<Vec<char>>, dirs: &Vec<Dir>) -> Vec<(Fpos, usize)> {
        let Fpos(pos, dir) = self;
        let mut next: Vec<(Fpos, usize)> = vec![];
        for opt in dirs {
            if opt == &dir.opposite() {continue;}
            let (x, y) = opt.get_loc();
            let x = pos.x + x;
            let y = pos.y + y;
            if map[y as usize][x as usize] != '#' {
                let npos = Pos{x, y};
                let s = Fpos(npos, opt.clone());
                if opt == dir { next.push((s, 1)) }
                else { next.push((s, 1001)) }
            }
        }
        next
    }
}

#[aoc(day16, part1)]
fn dec16_1(input: &Vec<Vec<char>>) -> usize{
    let start = Pos{x:1, y:(input.len()-2) as i32};
    let fstart = Fpos(start, Dir::RIGHT);
    let dirs = vec![Dir::RIGHT, Dir::LEFT, Dir::UP, Dir::DOWN];
    let result = dijkstra(&fstart, 
        |p| p.successors(input, &dirs), 
        |p| input[p.0.y as usize][p.0.x as usize] == 'E')
        .unwrap();
    result.1
}

#[aoc(day16, part2)]
fn dec16_2(input: &Vec<Vec<char>>) -> usize{
    let start = Pos{x:1, y:(input.len()-2) as i32};
    let fstart = Fpos(start, Dir::RIGHT);
    let dirs = vec![Dir::RIGHT, Dir::LEFT, Dir::UP, Dir::DOWN];
    let goal = Pos{x:(input[0].len()-2) as i32, y: 1};
    let result = astar_bag(&fstart, 
        |p| p.successors(input, &dirs), 
        |p| p.distance(&goal),
        |p| input[p.0.y as usize][p.0.x as usize] == 'E')
        .unwrap();
    let mut unsorted:Vec<Pos> = result.0.flatten().map(|f| f.0).collect();
    unsorted.sort();
    unsorted.iter().dedup().count()

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_large_two() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day16_large").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec16_2(&input_filtered);
        assert_eq!(64, ans);
    }

    #[test]
    fn test_small_two() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day16_small").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec16_2(&input_filtered);
        assert_eq!(45, ans);
    }

    #[test]
    fn test_large() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day16_large").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec16_1(&input_filtered);
        assert_eq!(11048, ans);
    }

    #[test]
    fn test_small() {
        let test_input: String = fs::read_to_string("input/2024/test_input/day16_small").unwrap();
        let input_filtered = get_input(&test_input);
        let ans = dec16_1(&input_filtered);
        assert_eq!(7036, ans);
    }
}