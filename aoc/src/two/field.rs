use std::{
    fmt::{Debug, Display},
    mem,
};

use crate::two::Point;

use super::{pt, IPoint};

/// A dense 2D field of cells. Has methods to get and mutate cells as if it was
/// bounded, or an infinite toriodal surface. Allows getting neighbors for
/// different topologies too.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct DenseField<T> {
    width: isize,
    height: isize,
    data: Vec<T>,
}

impl<T: Clone> DenseField<T> {
    pub fn new(width: isize, height: isize, val: T) -> Self {
        assert!(width > 0 && height > 0);

        DenseField {
            width,
            height,
            data: vec![val; (width * height).try_into().unwrap()],
        }
    }

    pub fn width(&self) -> isize {
        self.width
    }

    pub fn height(&self) -> isize {
        self.height
    }

    /// Brute force find a given value in the field.
    pub fn find(&self, val: &T) -> Option<Point<isize>>
    where
        T: PartialEq,
    {
        for y in 0..self.height {
            for x in 0..self.width {
                if *val == *self.get(pt(x, y)) {
                    return Some(Point::new(x, y));
                }
            }
        }

        None
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    /// Rotate the field by 90 degrees clockwise. This obviously flips the
    /// height and width if they're different. Not an efficient operation.
    pub fn rotate_clockwise(&mut self) {
        let old = self.clone();
        mem::swap(&mut self.width, &mut self.height);

        for oldy in 0..old.height {
            for oldx in 0..old.width {
                *self.get_mut(pt(old.height - oldy - 1, oldx)) = old.get(pt(oldx, oldy)).clone();
            }
        }
    }
}

impl<T: Display> DenseField<T> {
    pub fn debug_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.get(pt(x, y)))
            }
            println!()
        }
    }
}

impl<T> DenseField<T> {
    pub fn get(&self, p: IPoint) -> &T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        &self.data[(p.y * self.width + p.x) as usize]
    }

    pub fn get_mut(&mut self, p: IPoint) -> &mut T {
        assert!(p.x >= 0 && p.x < self.width);
        assert!(p.y >= 0 && p.y < self.height);
        &mut self.data[(p.y * self.width + p.x) as usize]
    }

    pub fn try_get(&self, p: IPoint) -> Option<&T> {
        if (0..self.width).contains(&p.x) && (0..self.height).contains(&p.y) {
            Some(&self.data[(p.y * self.width + p.x) as usize])
        } else {
            None
        }
    }

    pub fn try_get_mut(&mut self, p: IPoint) -> Option<&mut T> {
        if (0..self.width).contains(&p.x) && (0..self.height).contains(&p.y) {
            Some(&mut self.data[(p.y * self.width + p.x) as usize])
        } else {
            None
        }
    }

    /// `get` but x and y are wrapped around like a torus.
    pub fn wrapping_get(&self, p: IPoint) -> (&T, Point<isize>) {
        let x = p.x % self.width;
        let x = if x < 0 { self.width + x } else { x };
        let y = p.y % self.height;
        let y = if y < 0 { self.height + y } else { y };
        (&self.data[(y * self.width + x) as usize], Point::new(x, y))
    }

    /// `get` but x and y are wrapped around like a torus.
    pub fn wrapping_get_mut(&mut self, p: IPoint) -> (&mut T, Point<isize>) {
        let x = p.x % self.width;
        let x = if x < 0 { self.width + x } else { x };
        let y = p.y % self.height;
        let y = if y < 0 { self.height + y } else { y };
        (
            &mut self.data[(y * self.width + x) as usize],
            Point::new(x, y),
        )
    }

    /// Return the list of the eight possible neighbours around this point.
    /// Points outside of the field are not returned. Each value contains the
    /// neighbout value and the point of that neighbour.
    pub fn neighbours8_bounded(&self, p: IPoint) -> impl Iterator<Item = (&T, Point<isize>)> {
        let Point { x, y } = p;
        let p = |x, y| (self.try_get(pt(x, y)), Point::new(x, y));
        [
            p(x - 1, y - 1),
            p(x, y - 1),
            p(x + 1, y - 1),
            p(x - 1, y),
            p(x + 1, y),
            p(x - 1, y + 1),
            p(x, y + 1),
            p(x + 1, y + 1),
        ]
        .into_iter()
        .filter_map(|(nei, p)| nei.map(|nei| (nei, p)))
    }

    /// Return neighbours as if the field is the surface of a torus.
    pub fn neighbours8_torus(&self, p: IPoint) -> impl Iterator<Item = (&T, Point<isize>)> {
        let Point { x, y } = p;
        let p = |x, y| self.wrapping_get(pt(x, y));

        [
            p(x - 1, y - 1),
            p(x, y - 1),
            p(x + 1, y - 1),
            p(x - 1, y),
            p(x + 1, y),
            p(x - 1, y + 1),
            p(x, y + 1),
            p(x + 1, y + 1),
        ]
        .into_iter()
    }

    /// Up down left right neighbours
    pub fn neighbours4_bounded(&self, p: IPoint) -> impl Iterator<Item = (&T, Point<isize>)> {
        let Point { x, y } = p;
        let p = |x, y| (self.try_get(pt(x, y)), Point::new(x, y));
        [p(x, y - 1), p(x - 1, y), p(x + 1, y), p(x, y + 1)]
            .into_iter()
            .filter_map(|(nei, p)| nei.map(|nei| (nei, p)))
    }

    /// Up down left right neighbours, like a torus.
    pub fn neighbours4_torus(&self, p: IPoint) -> impl Iterator<Item = (&T, Point<isize>)> {
        let Point { x, y } = p;
        let p = |x, y| self.wrapping_get(pt(x, y));
        [p(x, y - 1), p(x - 1, y), p(x + 1, y), p(x, y + 1)].into_iter()
    }

    // Direct diagonals from thiss point.
    pub fn diagonals_bounded(&self, p: IPoint) -> impl Iterator<Item = (&T, Point<isize>)> {
        let Point { x, y } = p;
        let p = |x, y| (self.try_get(pt(x, y)), Point::new(x, y));
        [
            p(x - 1, y - 1),
            p(x + 1, y - 1),
            p(x - 1, y + 1),
            p(x + 1, y + 1),
        ]
        .into_iter()
        .filter_map(|(nei, p)| nei.map(|nei| (nei, p)))
    }

    /// Return neighbours as if the field is the surface of a torus.
    pub fn diagonals_torus(&self, p: IPoint) -> impl Iterator<Item = (&T, Point<isize>)> {
        let Point { x, y } = p;
        let p = |x, y| self.wrapping_get(pt(x, y));

        [
            p(x - 1, y - 1),
            p(x + 1, y - 1),
            p(x - 1, y + 1),
            p(x + 1, y + 1),
        ]
        .into_iter()
    }

    pub fn points(&self) -> PointsIter {
        PointsIter {
            width: self.width,
            height: self.height,
            current: 0,
        }
    }
}

impl<T: From<u8>> DenseField<T> {
    /// Create a field from line input. All lines must be the same length. You
    /// can implement From<u8> for T in order to have more complex starting types.
    pub fn from_lines(lines: Vec<String>) -> Self {
        Self::from_lines_with(lines, T::from)
    }

    pub fn from_lines_with(lines: Vec<String>, f: fn(u8) -> T) -> Self {
        let width = lines[0].len() as isize;
        let height = lines.len() as isize;

        for line in &lines {
            assert_eq!(line.len(), width as usize);
        }

        let data = lines.join("");
        assert!(data.is_ascii());
        let data = data.into_bytes().into_iter().map(f).collect();

        DenseField {
            width,
            height,
            data,
        }
    }
}

#[derive(Debug)]
pub struct PointsIter {
    width: isize,
    height: isize,
    current: isize,
}

impl Iterator for PointsIter {
    type Item = IPoint;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.width * self.height {
            None
        } else {
            let p = pt(self.current % self.width, self.current / self.width);
            self.current += 1;
            Some(p)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::two::{pt, DenseField};
    use std::{collections::HashSet, mem};

    #[test]
    fn make() {
        DenseField::new(10, 20, 0);
    }

    #[test]
    fn wrapping() {
        let field = DenseField::new(10, 10, 0);
        assert_eq!(field.wrapping_get(pt(11, 12)).1, pt(1, 2));
        assert_eq!(field.wrapping_get(pt(101, 12)).1, pt(1, 2));
        assert_eq!(field.wrapping_get(pt(-2, -3)).1, pt(8, 7)); // 0 -> 0, -1 -> 9...
        assert_eq!(field.wrapping_get(pt(-12, -103)).1, pt(8, 7));

        let neighbours: HashSet<_> = field.neighbours8_torus(pt(9, 5)).map(|t| t.1).collect();
        assert!(neighbours.contains(&pt(8, 5)));
        assert!(neighbours.contains(&pt(0, 5)));
        assert!(neighbours.contains(&pt(0, 6)));
    }

    #[test]
    fn from_u8_field() {
        struct Cell {
            pub _inner: u8,
        }
        impl From<u8> for Cell {
            fn from(value: u8) -> Self {
                Cell { _inner: value }
            }
        }
        let field = DenseField::from_lines(vec!["aaa".to_string()]);
        let _c: &Cell = field.get(pt(0, 0));
    }

    #[test]
    fn rotate() {
        pub fn naive_clockwise_rotate(field: &mut DenseField<isize>) {
            let old = field.clone();
            mem::swap(&mut field.width, &mut field.height);

            for oldy in 0..old.height {
                for oldx in 0..old.width {
                    *field.get_mut(pt(old.height - oldy - 1, oldx)) = *old.get(pt(oldx, oldy));
                }
            }
        }

        let mut field = DenseField::<isize>::new(12, 32, 0);
        let mut i = 0;
        for y in 0..32 {
            for x in 0..12 {
                i += 1;
                *field.get_mut(pt(x, y)) = i;
            }
        }

        let mut expected = field.clone();
        let mut actual = field;
        naive_clockwise_rotate(&mut expected);
        actual.rotate_clockwise();

        assert_eq!(expected, actual);
    }

    #[test]
    fn point_iter() {
        let mut ps = vec![];
        for y in 0..10 {
            for x in 0..5 {
                ps.push(pt(x, y));
            }
        }

        assert!(ps.into_iter().eq(DenseField::new(5, 10, 0).points()));
    }
}
