// Not clear what this file should have. There are so many different 2D fields.
// * They can be a known or unknown size.
// * They could start at 0,0 or also have negative indicies.
// * They could be unbounded.
// * They could be dense or sparse.
// * They could wrap around in various ways (doughnut, cube!).

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
}

#[cfg(test)]
mod test {
    use super::BoundedField;

    #[test]
    fn make() {
        BoundedField::new(10, 20, 0);
    }
}
