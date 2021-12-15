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

    pub fn width(&self) -> usize {
        self.stride
    }

    pub fn height(&self) -> usize {
        self.data.len() / self.stride
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

impl Field2D<u8> {
    pub fn from_str(s: &str) -> Self {
        Field2D {
            data: s
                .lines()
                .map(|l| l.chars().map(|c| (c as u8) - b'0'))
                .flatten()
                .collect(),
            stride: s.lines().next().unwrap().len(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut s = String::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                s.push((b'0' + self[(x, y)]) as char);
            }
            s.push('\n');
        }
        s
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
