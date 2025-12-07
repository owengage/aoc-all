use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Map {
    data: Vec<isize>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(data: Vec<isize>, width: usize) -> Self {
        let height = data.len() / width;
        assert!(width * height == data.len());
        Self {
            data,
            width,
            height,
        }
    }

    fn get(&self, p: (isize, isize)) -> isize {
        self.data[self.i(p)]
    }

    fn set(&mut self, p: (isize, isize), v: isize) {
        let i = self.i(p);
        self.data[i] = v;
    }

    fn wasd(&self, p: (isize, isize)) -> WasdIter {
        WasdIter::new(p, self.width as isize, self.height as isize)
    }

    fn i(&self, p: (isize, isize)) -> usize {
        p.1 as usize * self.width + p.0 as usize
    }
}

struct WasdIter {
    wasd: [(isize, isize); 4],
    height: isize,
    width: isize,
    i: usize,
}

impl WasdIter {
    fn new(p: (isize, isize), width: isize, height: isize) -> Self {
        Self {
            wasd: [
                (p.0 - 1, p.1),
                (p.0, p.1 - 1),
                (p.0 + 1, p.1),
                (p.0, p.1 + 1),
            ],
            height,
            width,
            i: 0,
        }
    }
}

impl Iterator for WasdIter {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        for i in self.i..4 {
            let p = self.wasd[i];
            if (0..self.width as isize).contains(&p.0) && (0..self.height as isize).contains(&p.1) {
                self.i = i + 1;
                return Some(p);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Goal {
    start: (isize, isize),
    target: (isize, isize),
}

fn parse(r: impl BufRead) -> (Map, Goal) {
    let mut r = r.lines().flatten().peekable();
    let width = r.peek().unwrap().len();

    let data: Vec<_> = r
        .flat_map(|line| line.into_bytes())
        .map(|b| b as isize)
        .collect();

    let start = data
        .iter()
        .enumerate()
        .find(|(_, &c)| c == 'S' as isize)
        .map(|(i, _)| ((i % width) as isize, (i / width) as isize))
        .unwrap();

    let target = data
        .iter()
        .enumerate()
        .find(|(_, &c)| c == 'E' as isize)
        .map(|(i, _)| ((i % width) as isize, (i / width) as isize))
        .unwrap();

    let mut map = Map::new(data, width);
    map.set(start, 'a' as isize);
    map.set(target, 'z' as isize);

    (map, Goal { start, target })
}

fn trails(map: &Map, start: (isize, isize)) -> Map {
    let mut q = VecDeque::<(isize, isize)>::new();
    let mut best = Map::new(vec![isize::MAX; map.data.len()], map.width);

    best.set(start, 0);
    q.push_back(start);

    while let Some(current) = q.pop_back() {
        let current_best = best.get(current);
        let search = map.wasd(current);
        for next in search {
            let next_old_best = best.get(next);
            // can we actually do this step?
            if (map.get(current) - map.get(next)) <= 1 && (current_best + 1 < next_old_best) {
                // if this path is better than the existing one, make sure we
                // search this path further.
                best.set(next, current_best + 1);
                q.push_back(next);
            }
        }
    }

    best
}

fn main() {
    let (map, goal) = parse(BufReader::new(File::open("input.txt").unwrap()));
    // let reachability = reachability(&map, goal.start);

    // println!("{:?}", reachability.get(goal.target));

    let tra = trails(&map, goal.target);

    let mut min = isize::MAX;

    for i in 0..map.data.len() {
        if map.data[i] == 'a' as isize {
            let trail = tra.data[i];
            if trail < min {
                min = trail;
            }
        }
    }

    println!("to start: {}", tra.get(goal.start));
    println!("min: {}", min);
}
