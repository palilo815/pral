use std::vec;

struct Grid<T> {
    n: usize,
    m: usize,
    data: Box<[T]>,
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.data[x * self.m + y]
    }
}

impl<T> std::ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.data[x * self.m + y]
    }
}

impl<T: Clone + Copy> Grid<T> {
    fn new(n: usize, m: usize, default: T) -> Self {
        let data = vec![default; n * m].into_boxed_slice();
        Self { n, m, data }
    }
    fn iter(&self) -> GridIter<'_, T> {
        GridIter { grid: self, x: 0, y: 0 }
    }
}

struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    x: usize,
    y: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((usize, usize), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.grid.n {
            return None;
        }
        let item = ((self.x, self.y), &self.grid[(self.x, self.y)]);
        if self.y + 1 == self.grid.m {
            self.x += 1;
            self.y = 0;
        } else {
            self.y += 1;
        }
        Some(item)
    }
}

#[test]
fn index() {
    let array_2d = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    let mut grid = Grid::new(3, 4, 0);
    for (i, v) in array_2d.iter().enumerate() {
        for (j, x) in v.iter().enumerate() {
            grid[(i, j)] = *x;
        }
    }
    assert_eq!(
        array_2d.iter().flatten().collect::<Vec<_>>(),
        grid.data.iter().collect::<Vec<_>>()
    );
}

#[test]
fn grid_iterator() {
    let array_2d = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]];
    let mut grid = Grid::new(3, 4, 0);
    for (i, v) in array_2d.iter().enumerate() {
        for (j, x) in v.iter().enumerate() {
            grid[(i, j)] = *x;
        }
    }
    let mut it = grid.iter();
    for i in 0..3 {
        for j in 0..4 {
            let value = i * 4 + j + 1;
            let item = it.next();
            assert_eq!(item, Some(((i, j), &value)));
        }
    }
    assert!(it.next().is_none());
}
