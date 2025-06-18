#[derive(Clone, Copy, Debug)]
struct Matrix<const N: usize, const M: usize, T> {
    data: [[T; M]; N],
}

impl<const N: usize, const M: usize, T> std::ops::Add<Self> for Matrix<N, M, T>
where
    T: std::ops::Add<Output = T> + Copy + Default,
{
    type Output = Matrix<N, M, T>;
    fn add(self, other: Self) -> Self::Output {
        let mut result = Self { data: [[T::default(); M]; N] };
        for i in 0..N {
            for j in 0..M {
                result.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        result
    }
}

impl<const N: usize, const K: usize, const M: usize, T> std::ops::Mul<Matrix<K, M, T>> for Matrix<N, K, T>
where
    T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy + Default,
{
    type Output = Matrix<N, M, T>;
    fn mul(self, rhs: Matrix<K, M, T>) -> Self::Output {
        let mut result = Matrix { data: [[T::default(); M]; N] };
        for i in 0..N {
            for j in 0..M {
                let mut sum = T::default();
                for k in 0..K {
                    sum = sum + self.data[i][k] * rhs.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }
}

impl<const N: usize, const M: usize, T> Matrix<N, M, T> {
    fn new(data: [[T; M]; N]) -> Self {
        Self { data }
    }
}

#[test]
fn ops_add() {
    let data_a = [[1, 2, 3], [4, 5, 6]];
    let data_b = [[10, 20, 30], [40, 50, 60]];
    let data_c = [[11, 22, 33], [44, 55, 66]];
    let a = Matrix::new(data_a);
    let b = Matrix::new(data_b);
    let c = a + b;
    assert_eq!(data_c, c.data);
}

#[test]
fn ops_mul() {
    let data_a = [[1, -2], [3, -4], [5, -6]];
    let data_b = [[1, -3, 5, -7], [-2, 4, -6, 8]];
    let data_c = [[5, -11, 17, -23], [11, -25, 39, -53], [17, -39, 61, -83]];
    let a = Matrix::new(data_a);
    let b = Matrix::new(data_b);
    let c = a * b;
    assert_eq!(data_c, c.data);
}
