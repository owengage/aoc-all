use aoc::{
    fetch_input, lines,
    two::{pt, DenseField, IPoint},
};
use itertools::Itertools;

#[derive(Debug)]
struct Robot {
    start: IPoint,
    vel: IPoint,
}

fn main() {
    let input = lines(fetch_input(2024, 14));
    let robots = parse_robots(&input);
    let space = DenseField::new(101, 103, false);

    let part1 = part1(&robots, &space);
    println!("part1 = {part1}");

    for i in 0..100000 {
        let locations = robots
            .iter()
            .map(|r| predict_location(&space, r, i))
            .collect_vec();

        let mut picture = DenseField::new(101, 103, '.');
        for p in locations {
            *picture.get_mut(p) = '#';
        }

        let found_line = picture
            .data()
            .windows(10)
            .any(|win| win.iter().all(|d| *d == '#'));

        if found_line {
            println!("\nBelow is second {i}");
            picture.debug_print();
            println!("If you see a christmas tree... part2 = {i}");
            break;
        }
    }
}

fn part1(robots: &[Robot], space: &DenseField<bool>) -> usize {
    let locations = robots
        .iter()
        .map(|r| predict_location(space, r, 100))
        .collect_vec();

    let qtl = count_quadrant(
        &locations,
        pt(0, 0),
        pt(space.width() / 2, space.height() / 2),
    );
    let qtr = count_quadrant(
        &locations,
        pt(space.width() / 2 + 1, 0),
        pt(space.width(), space.height() / 2),
    );
    let qbl = count_quadrant(
        &locations,
        pt(0, space.height() / 2 + 1),
        pt(space.width() / 2, space.height()),
    );
    let qbr = count_quadrant(
        &locations,
        pt(space.width() / 2 + 1, space.height() / 2 + 1),
        pt(space.width(), space.height()),
    );

    qtl * qtr * qbl * qbr
}

fn count_quadrant(locations: &[IPoint], start: IPoint, end: IPoint) -> usize {
    locations
        .iter()
        .filter(|loc| (start.x..end.x).contains(&loc.x) && (start.y..end.y).contains(&loc.y))
        .count()
}

fn predict_location(space: &DenseField<bool>, r: &Robot, steps: isize) -> IPoint {
    let end_unwrapped = r.start + r.vel * steps;

    let (_, p) = space.wrapping_get(end_unwrapped);
    p
}

fn parse_robots(input: &[String]) -> Vec<Robot> {
    let re = regex::Regex::new(r#"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)"#).unwrap();
    input
        .iter()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            let start = pt(caps[1].parse().unwrap(), caps[2].parse().unwrap());
            let vel = pt(caps[3].parse().unwrap(), caps[4].parse().unwrap());
            Robot { start, vel }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use aoc::{lines_from_str, two::DenseField};

    use crate::{parse_robots, part1};

    #[test]
    fn test_parse() {
        let input = lines_from_str(
            r#"p=0,4 v=3,-3
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
p=9,5 v=-3,-3"#,
        );

        let robots = parse_robots(&input);
        let space = DenseField::new(11, 7, false);

        let part1 = part1(&robots, &space);
        assert_eq!(part1, 12);
    }
}
