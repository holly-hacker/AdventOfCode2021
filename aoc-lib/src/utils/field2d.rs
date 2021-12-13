use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct Field2D<T> {
    pub data: Vec<T>,
    pub stride: usize,
}

impl<T> Field2D<T>
where
    T: Copy,
{
    pub fn new(width: usize, height: usize, value: T) -> Self {
        let stride = width;
        let data = vec![value; width * height];
        Self { data, stride }
    }

    // TODO: change with index or indexmut impl
    pub fn get(&self, x: usize, y: usize) -> T {
        self.data[x + y * self.stride]
    }

    pub fn neighbour_indices(&self, idx: usize) -> [Option<usize>; 4] {
        let x = idx % self.stride;
        [
            if idx >= self.stride {
                Some(idx - self.stride)
            } else {
                None
            },
            if idx + self.stride < self.data.len() {
                Some(idx + self.stride)
            } else {
                None
            },
            if x > 0 { Some(idx - 1) } else { None },
            if x < self.stride - 1 {
                Some(idx + 1)
            } else {
                None
            },
        ]
    }
}

impl<T> Index<(usize, usize)> for Field2D<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 + index.1 * self.stride]
    }
}

impl<T> IndexMut<(usize, usize)> for Field2D<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.data[index.0 + index.1 * self.stride]
    }
}
