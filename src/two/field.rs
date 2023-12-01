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
}

#[cfg(test)]
mod test {
    use super::BoundedField;

    #[test]
    fn make() {
        BoundedField::new(10, 20, 0);
    }
}
