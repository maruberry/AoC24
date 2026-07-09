use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::bfs_reach;
use pathfinding::prelude::count_paths;

#[aoc_generator(day10)]
fn get_input(input: &str) -> (Map, Vec<Point>) { 
    let mut starts: Vec<Point> = vec![];
    let mut map: Vec<Vec<i32>> = input.lines()
        .enumerate()
        .map(|(i, x)| {
            let mut smth = x
            .chars()
            .enumerate()
            .map(|(j, n)| {
                let f = n.to_digit(10).unwrap() as i32;
                if f == 0 {starts.push(Point::new(j + 1, i + 1))}
                return f
            })
            .collect::<Vec<i32>>();
        
            smth.push(15);
            smth.insert(0, 15i32);
            return smth;
        })
        .collect();
    let padding = vec![15; map[0].len()];
    map.push(padding.clone());
    map.insert(0, padding);

    (Map {
        map,
        loc: Point::new(0, 0)
    }, starts)
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point{
        Point { x: x, y: y }
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Map {
    map: Vec<Vec<i32>>,
    loc: Point,
}

impl Map {
    fn new_loc(&self, x_change: &i32, y_change: &i32) -> Map{
        let mut smth = self.clone();
        smth.loc.x = (self.loc.x as i32 + x_change).try_into().expect("We are fucked");
        smth.loc.y = (self.loc.y as i32 + y_change).try_into().expect("We are fucked");
        return smth;
    }

    fn start_loc(&mut self, loc: &Point){
        self.loc.x = loc.x;
        self.loc.y = loc.y;
    }

    fn calc_next(&self, x_change: &i32, y_change: &i32) -> bool{
        let x2: usize = (self.loc.x as i32 + x_change).try_into().expect("We are fucked");
        let y2: usize = (self.loc.y as i32 + y_change).try_into().expect("We are fucked");
        self.map[y2][x2] - self.map[self.loc.y][self.loc.x] == 1
    }

    fn get_value(&self) -> i32 {
        self.map[self.loc.y][self.loc.x]
    }
}


fn successors(loc: &Map) -> Vec<Map> {
    let mut next: Vec<Map> = vec![];
    vec![(0, 1), (1, 0), (-1, 0), (0, -1)].iter()
    .for_each(|(x_change, y_change)| {
        if loc.calc_next(x_change, y_change){
            next.push(loc.new_loc(x_change, y_change));
        }
    });
    return next
}

#[aoc(day10, part1)]
fn dec10_1(input: &(Map, Vec<Point>)) -> usize{
    let mut map = input.0.clone();
    let starts = &input.1;
    let mut answer = 0;

    for start in starts {
        map.start_loc(start);
        let all_nodes = bfs_reach(map.clone(), successors).collect::<Vec<_>>();
        let add = all_nodes.iter().filter(|x| x.get_value() == 9i32).count();
        answer += add;
    }
    answer
}

#[aoc(day10, part2)]
fn dec10_2(input: &(Map, Vec<Point>)) -> usize{
    let mut map = input.0.clone();
    let starts = &input.1;
    let mut answer = 0;

    for start in starts {
        map.start_loc(start);
        let count = count_paths(map.clone(), successors, |x| x.get_value() == 9);
        answer += count;
    }
    answer
}

#[cfg(test)]
mod tests {
    use super::{get_input, dec10_1, dec10_2};

    const TEST_INPUT:&str = 
"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn test_part1() {
        let input_filtered = get_input(TEST_INPUT);
        let ans = dec10_1(&input_filtered);
        assert_eq!(36, ans);
    }

    #[test]
    fn test_part2() {
        let input_filtered = get_input(TEST_INPUT);
        let ans = dec10_2(&input_filtered);
        assert_eq!(81, ans);
    }
}