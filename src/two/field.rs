// Not clear what this file should have. There are so many different 2D fields.
// * They can be a known or unknown size.
// * They could start at 0,0 or also have negative indicies.
// * They could be unbounded.
// * They could be dense or sparse.
// * They could wrap around in various ways (doughnut, cube!).

use super::Point;

pub struct BoundedField<T> {
    pub width: isize,
    pub height: isize,
    pub data: Vec<T>,
}

impl<T: Clone> BoundedField<T> {
    pub fn new(width: isize, height: isize, val: T) -> Self {
        assert!(width > 0 && height > 0);

        BoundedField {
            width,
            height,
            data: vec![val; (width * height).try_into().unwrap()],
        }
    }
    pub fn from_lines(lines: Vec<String>) -> BoundedField<u8> {
        let width = lines[0].len() as isize;
        let height = lines.len() as isize;
        let data = lines.join("");
        assert!(data.is_ascii());
        let data = data.into_bytes();

        BoundedField {
            width,
            height,
            data,
        }
    }

    pub fn get(&self, x: isize, y: isize) -> T {
        assert!(x >= 0 && x < self.width);
        assert!(y >= 0 && y < self.height);
        self.data[(y * self.width + x) as usize].clone()
    }

    pub fn try_get(&self, x: isize, y: isize) -> Option<T> {
        if (x >= 0 && x < self.width) && (y >= 0 && y < self.height) {
            Some(self.data[(y * self.width + x) as usize].clone())
        } else {
            None
        }
    }

    pub fn eight_neighbours(&self, x: isize, y: isize) -> Vec<(T, Point<isize>)> {
        vec![
            (self.try_get(x - 1, y - 1), Point::new(x - 1, y - 1)),
            (self.try_get(x, y - 1), Point::new(x, y - 1)),
            (self.try_get(x + 1, y - 1), Point::new(x + 1, y - 1)),
            (self.try_get(x - 1, y + 1), Point::new(x - 1, y + 1)),
            (self.try_get(x, y + 1), Point::new(x, y + 1)),
            (self.try_get(x + 1, y + 1), Point::new(x + 1, y + 1)),
            (self.try_get(x - 1, y), Point::new(x - 1, y)),
            (self.try_get(x + 1, y), Point::new(x + 1, y)),
        ]
        .into_iter()
        .filter_map(|(nei, p)| nei.map(|nei| (nei, p)))
        .collect()
    }
}

#[cfg(test)]
mod test {
    use super::BoundedField;

    #[test]
    fn make() {
        BoundedField::new(10, 20, 0);
    }
}
