use std::ops::{Add, Mul, Neg, Sub};

pub trait Num:
    Copy + Add<Self, Output = Self> + Sub<Self, Output = Self> + Mul<Self, Output = Self>
{
}

impl Num for isize {}
impl Num for f64 {}

pub const LEFT: Point<isize> = pt(-1, 0);
pub const RIGHT: Point<isize> = pt(1, 0);
pub const UP: Point<isize> = pt(0, -1);
pub const DOWN: Point<isize> = pt(0, 1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dirn {
    Right,
    Left,
    Up,
    Down,
}

impl Dirn {
    pub fn as_point(self) -> Point<isize> {
        match self {
            Dirn::Right => RIGHT,
            Dirn::Left => LEFT,
            Dirn::Up => UP,
            Dirn::Down => DOWN,
        }
    }

    pub fn all() -> [Point<isize>; 4] {
        [LEFT, RIGHT, UP, DOWN]
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub const fn pt<T: Num>(x: T, y: T) -> Point<T> {
    Point::new(x, y)
}

impl<T: Num> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn norm_squared(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl Point<isize> {
    pub fn norm(&self) -> f64 {
        (self.norm_squared() as f64).sqrt()
    }

    // Absolute distance between two points, only along the x and y directions.
    // The way a taxi would drive in Manhatten's grid.
    pub fn taxicab_dist(&self, p: Self) -> usize {
        self.x.abs_diff(p.x) + self.y.abs_diff(p.y)
    }
}

impl Point<f64> {
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }
}

impl<T: Num> Add<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Num> Sub<Point<T>> for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Num> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Point<isize>> for isize {
    type Output = Point<isize>;

    fn mul(self, rhs: Point<isize>) -> Self::Output {
        rhs * self
    }
}

impl Neg for Point<isize> {
    type Output = Point<isize>;

    fn neg(self) -> Self::Output {
        -1 * self
    }
}

impl Mul<Point<f64>> for f64 {
    type Output = Point<f64>;

    fn mul(self, rhs: Point<f64>) -> Self::Output {
        rhs * self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ops() {
        let p1 = pt(1, 2);
        let p2 = pt(4, 5);
        assert_eq!(p1 + p2, pt(5, 7));
        assert_eq!(p1 - p2, pt(-3, -3));
        assert_eq!(p1 * 2, pt(2, 4));
        assert_eq!(2 * p1, pt(2, 4));

        let pt3 = pt(1.0, 2.0);
        assert_eq!(pt3 * 0.5, pt(0.5, 1.0));
        assert_eq!(0.5 * pt3, pt(0.5, 1.0));

        assert_eq!(pt(3, 4).norm(), 5.0);
        assert_eq!(pt(4.0, 3.0).norm(), 5.0);
    }
}
