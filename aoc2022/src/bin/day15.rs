use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    ops::Range,
    vec,
};

use rayon::prelude::*;

use regex::Regex;

#[derive(Debug)]
struct Sensor {
    x: isize,
    y: isize,
    beacon_x: isize,
    beacon_y: isize,
    range: isize,
}

impl Sensor {
    fn new(x: isize, y: isize, beacon_x: isize, beacon_y: isize) -> Self {
        Self {
            x,
            y,
            beacon_x,
            beacon_y,
            range: (x - beacon_x).abs() + (y - beacon_y).abs(),
        }
    }
}

fn re() -> Regex {
    Regex::new(r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#)
        .unwrap()
}

fn parse(mut r: impl BufRead) -> Vec<Sensor> {
    let re = re();
    let mut sensors = vec![];
    let mut input = String::new();
    r.read_to_string(&mut input).unwrap();

    for cap in re.captures_iter(&input) {
        let s = Sensor::new(
            cap[1].parse().unwrap(),
            cap[2].parse().unwrap(),
            cap[3].parse().unwrap(),
            cap[4].parse().unwrap(),
        );
        sensors.push(s);
    }

    sensors
}

fn range_on_y(sensor: &Sensor, target_y: isize) -> Option<Range<isize>> {
    let range = sensor.range;
    let range_on_target = range - (sensor.y - target_y).abs();
    if range_on_target >= 0 {
        // Off by ones?
        Some(sensor.x - range_on_target..sensor.x + range_on_target + 1)
    } else {
        None
    }
}

fn free_on_y(sensors: &[Sensor], target_y: isize) -> Vec<Range<isize>> {
    let mut ranges = sensors
        .iter()
        .filter_map(|sensor| range_on_y(sensor, target_y))
        .collect::<Vec<_>>();

    // sort to that range.start is lowest to highest, then by smallest len.
    ranges.sort_by(|a, b| a.start.cmp(&b.start).then_with(|| a.len().cmp(&b.len())));

    let mut merged = vec![];
    let mut start = ranges[0].start;
    let mut end = ranges[0].end;

    for range in ranges {
        // If the start of this range is inside our curent one, change end to
        // largest of them.
        if (start..end).contains(&range.start) {
            end = range.end.max(end);
        } else {
            // our start isn't inside the current range, so it's a new range,
            // store the current one and move on.
            merged.push(start..end);
            start = range.start;
            end = range.end;
        }
    }
    // last one needs adding.
    merged.push(start..end);

    merged
}

fn main() {
    let sensors = parse(BufReader::new(File::open("input.txt").unwrap()));

    let target_y: isize = 2000000;

    let beacons: HashSet<_> = sensors.iter().map(|s| (s.beacon_x, s.beacon_y)).collect();
    let beacons_on_target = beacons.iter().filter(|b| b.1 == target_y).count();

    let free = free_on_y(&sensors, target_y)
        .iter()
        .map(|r| r.len())
        .sum::<usize>();

    println!(
        "Cannot be a beacon on y={target_y}: {}",
        free - beacons_on_target
    );

    (0..4_000_000isize).into_par_iter().for_each(|y| {
        let free = free_on_y(&sensors, y);
        let intersection = |r: Range<isize>| r.start.max(0)..r.end.min(4_000_000);

        let free: Vec<_> = free
            .into_iter()
            .map(intersection)
            .filter(|r| !r.is_empty())
            .collect();

        if free.len() > 1 {
            println!("BROKEN RANGE, {:?}", free);
            let x = free[0].end;
            let freq = x * 4000000 + y;
            println!("Frequency: {}", freq);
        }
    });

    // part 2
}

#[cfg(test)]
mod test {

    use crate::{range_on_y, re, Sensor};

    #[test]
    fn parseexample() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";

        for cap in re().captures_iter(input) {
            println!("{cap:?}");
            let s = Sensor::new(
                cap[1].parse().unwrap(),
                cap[2].parse().unwrap(),
                cap[3].parse().unwrap(),
                cap[4].parse().unwrap(),
            );
            println!("{s:?}");
        }
    }

    #[test]
    fn tests() {
        assert!(range_on_y(&Sensor::new(0, 100, 0, 99), 0).is_none());
        assert_eq!(range_on_y(&Sensor::new(0, 0, 0, 2), 0).unwrap(), -2..3);
        //      21012
        // 0 ...##S##....
        // 1 ....###.....
        // 2 .....B......

        assert_eq!(range_on_y(&Sensor::new(0, 1, 0, 2), 0).unwrap(), 0..1);
        //      21012
        // 0 .....#......
        // 1 ....#S#.....
        // 2 .....B......

        assert!(range_on_y(&Sensor::new(0, 2, 0, 2), 0).is_none());
        //      21012
        // 0 ............
        // 1 ............
        // 2 .....B......

        assert_eq!(range_on_y(&Sensor::new(5, 2, 0, 2), 0).unwrap(), 2..9);
        //      21012345678
        // 0 .......#######
        // 1 ......#########
        // 2 .....B####S#####
    }
}
