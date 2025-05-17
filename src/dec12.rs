use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::bfs_reach;

#[aoc_generator(day12)]
fn get_input(input: &str) -> Vec<Vec<char>>{ 
    let mut mine: Vec<Vec<char>> = input.lines()
        .map(|s| {
            let mut n: Vec<char> = s.chars().collect();
            n.push('.');
            n.insert(0, '.');
            n
        })
        .collect();

    let fill = vec!['.'; mine[0].len()];
    mine.push(fill.clone());
    mine.insert(0, fill);
    mine

}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos{
    x: usize, 
    y: usize, 
    map: Vec<Vec<char>>,
    fence: usize,
}

impl Pos {
    fn successors(&self) -> Vec<Pos> {
        let Pos{x, y, map, fence: _} = self.clone();
        let c = map[y][x];
        let mut next: Vec<Pos> = vec![];
        let neighbors = Pos::get_neighbours(x, y, &map);
        for (xn, yn) in neighbors {
            let fence = 4 - Pos::get_neighbours(xn, yn, &map).len();
            next.push(Pos{x: xn as usize, y: yn as usize, map: map.clone(), fence});
        }

        next
    }

    fn get_neighbours(x: usize, y: usize, map: &Vec<Vec<char>>) -> Vec<(usize, usize)>{
        let mut rvec: Vec<(usize, usize)> = vec![];
        for (xplus, yplus) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = x as i32 + xplus;
            let ny = y as i32 + yplus;
            if nx == 0 || ny == 0 || nx as usize == map[0].len() || ny as usize == map.len() {continue;}
            if map[ny as usize][nx as usize] == map[y as usize][x as usize] {
                rvec.push((nx as usize, ny as usize));
            }
        }
        rvec
    }
}

#[aoc(day12, part1)]
fn dec12_1(garden: &Vec<Vec<char>>) -> usize{
    let mut all_plots: Vec<(usize, usize)> = vec![];
    let mut answer: usize = 0;
    let mut fence = 4 - Pos::get_neighbours(1, 1, garden).len();
    let mut start = Pos{x: 1, y: 1, map: garden.clone(), fence};
    for (mut i, line) in garden[1..garden.len()-1].iter().enumerate() {
        i += 1;
        for (mut j, flower) in line[1..line.len()-1].iter().enumerate() {
            j += 1;
            if !all_plots.contains(&(j, i)) {
                fence = 4 - Pos::get_neighbours(j, i, garden).len();
                start = Pos{x: j, y: i, map: garden.clone(), fence};
                let mut smth = bfs_reach(start.clone(), |p| p.successors()).collect::<Vec<_>>();
                let mut plot = smth
                    .iter()
                    .map(|x| (x.x, x.y))
                    .collect::<Vec<_>>();
                all_plots.append(&mut plot);
                let fences = smth.iter().map(|x| x.fence).sum::<usize>();
                let area = smth.len();
                answer +=  fences * area;
            }
        }
    }


    answer
}
 

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input_filtered = get_input(TEST_INPUT);
        let ans = dec12_1(&input_filtered);
        assert_eq!(1930, ans);
    }

    const SMALL_INPUT:&str = 
"OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const TEST_INPUT:&str = 
"RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
}