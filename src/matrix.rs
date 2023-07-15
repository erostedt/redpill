use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T>
{
    pub rows: usize,
    pub cols: usize,
    offset: usize,
    row_stride: usize,
    col_stride: usize,
    elements: Vec<T>,
}

pub struct MatrixIterator<'a, T>
{
    matrix: &'a Matrix<T>,
    row: usize,
    col: usize,
}


impl<T> Matrix<T>
where T: Default + Clone
{
    pub fn new(shape: (usize, usize)) -> Self
    {
        Self {rows: shape.0, cols: shape.1, offset: 0, row_stride: shape.1, col_stride: 1, elements: vec![T::default(); shape.0 * shape.1]}
    }

    pub fn shape(&self) -> (usize, usize)
    {
        (self.rows, self.cols)
    }
}


impl<T> Index<(usize, usize)> for Matrix<T>
{
    type Output = T;
    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output
    {
        &self.elements[self.offset + index.0 * self.row_stride + index.1 * self.col_stride]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T>
{
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T
    {
        &mut self.elements[self.offset + index.0 * self.row_stride + index.1 * self.col_stride]
    }    
}

impl<T> Add<Matrix<T>> for Matrix<T>
where T: Add<Output = T> + Copy + Default
{
    type Output = Matrix<T>;
    fn add(self, rhs: Matrix<T>) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Matrix::new(self.shape());
        for row in 0..self.rows 
        {
            for col in 0..self.cols 
            {
                let pos = (row, col);
                out[pos] = self[pos] + rhs[pos];       
            }
        }
        out
    }
}

impl<T> Add<T> for Matrix<T>
where T: Add<Output = T> + Copy + Default
{
    type Output = Matrix<T>;
    fn add(self, rhs: T) -> Self::Output 
    {
        let mut out = Matrix::new(self.shape());
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                out[pos] = self[pos] + rhs;
            }
        }
        out
    }
}

impl<T> AddAssign<Matrix<T>> for Matrix<T>
where T: AddAssign + Copy
{
    fn add_assign(&mut self, rhs: Matrix<T>) 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                self[pos] += rhs[pos];
            }
        }  
    }
}


impl<T> AddAssign<T> for Matrix<T>
where T: AddAssign + Copy
{
    fn add_assign(&mut self, rhs: T) 
    {
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                self[pos] += rhs;
            }
        }
    }
}


impl<T> Sub<Matrix<T>> for Matrix<T> 
where T: Sub<Output = T> + Copy + Default
{
    type Output = Matrix<T>;
    fn sub(self, rhs: Matrix<T>) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Matrix::new(self.shape());   
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                out[pos] = self[pos] - rhs[pos];
            }
        }
        out    
    }
}


impl<T> Sub<T> for Matrix<T> 
where T: Sub<Output = T> + Copy + Default
{
    type Output = Matrix<T>;
    fn sub(self, rhs: T) -> Self::Output 
    {
        let mut out = Matrix::new(self.shape());
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                out[pos] = self[pos] - rhs;
            }
        }
        out
    }
}


impl<T> SubAssign<Matrix<T>> for Matrix<T>
where T: SubAssign + Copy
{
    fn sub_assign(&mut self, rhs: Matrix<T>) 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                self[pos] -= rhs[pos]
            } 
        }
    }
}


impl<T> SubAssign<T> for Matrix<T>
where T: SubAssign + Copy
{
    fn sub_assign(&mut self, rhs: T) 
    {
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                self[pos] -= rhs;
            }
        }    
    }
}


impl<T> Mul<Matrix<T>> for Matrix<T>
where T: Mul<Output = T> + Copy + Default
{
    type Output = Matrix<T>;
    fn mul(self, rhs: Matrix<T>) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Matrix::new(self.shape());
        for row in 0..self.rows 
        {
            for col in 0..self.cols 
            {
                let pos = (row, col);
                out[pos] = self[pos] * rhs[pos];       
            }
        }
        out
    }
}

impl<T> Mul<T> for Matrix<T>
where T: Mul<Output = T> + Copy + Default
{
    type Output = Matrix<T>;
    fn mul(self, rhs: T) -> Self::Output 
    {
        let mut out = Matrix::new(self.shape());
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                out[pos] = self[pos] * rhs;
            }
        }
        out
    }
}

impl<T> MulAssign<Matrix<T>> for Matrix<T>
where T: MulAssign + Copy
{
    fn mul_assign(&mut self, rhs: Matrix<T>) 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                self[pos] *= rhs[pos];
            }
        }  
    }
}


impl<T> MulAssign<T> for Matrix<T>
where T: MulAssign + Copy
{
    fn mul_assign(&mut self, rhs: T) 
    {
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                self[pos] *= rhs;
            }
        }
    }
}


impl<T> Div<Matrix<T>> for Matrix<T>
where T: Div<Output = T> + Copy + Default
{
    type Output = Matrix<T>;
    fn div(self, rhs: Matrix<T>) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Matrix::new(self.shape());   
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                out[pos] = self[pos] / rhs[pos];
            }
        }
        out    
    }
}


impl<T> Div<T> for Matrix<T> 
where T: Div<Output = T> + Copy + Default
{
    type Output = Matrix<T>;
    fn div(self, rhs: T) -> Self::Output 
    {
        let mut out = Matrix::new(self.shape());
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                out[pos] = self[pos] / rhs;
            }
        }
        out
    }
}


impl<T> DivAssign<Matrix<T>> for Matrix<T>
where T: DivAssign + Copy
{
    fn div_assign(&mut self, rhs: Matrix<T>) 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                self[pos] /= rhs[pos]
            } 
        }
    }
}


impl<T> DivAssign<T> for Matrix<T>
where T: DivAssign + Copy
{
    fn div_assign(&mut self, rhs: T) 
    {
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                self[pos] /= rhs;
            }
        }    
    }
}

impl<'a, T> Iterator for MatrixIterator<'a, T>
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.row >= self.matrix.rows
        {
            return None;
        }
        
        let res = Some(&self.matrix[(self.row, self.col)]);
        self.col += 1;
        
        if self.col >= self.matrix.cols
        {
            self.row += 1;
            self.col = 0;
        }
        res
    }
}


impl<T> Matrix<T>
where T: AddAssign + Mul<Output = T> + Copy + Default
{
    pub fn matmul(&self, rhs: Matrix<T>) -> Matrix<T>
    {
        assert!(self.cols == rhs.rows);
        let mut out = Matrix::new((self.rows, rhs.cols));
        for i in 0..self.rows
        {
            for j in 0..rhs.cols
            {
                for k in 0..self.cols
                {
                    out[(i, j)] += self[(i, k)] * rhs[(k, j)];
                }
            }
        }
        out
    }
}

impl<T> Matrix<T>
where T: Copy
{
    pub fn swap_rows(&mut self, row1: usize, row2: usize)
    {
        assert!((row1 < self.rows) && (row2 < self.rows));        
        for col in 0..self.cols
        {
            let p1 = (row1, col);
            let p2 = (row2, col);
            let temp = self[p1];
            self[p1] = self[p2];
            self[p2] = temp;
        }
    }

}
impl <T> Matrix<T>
{
    pub fn iter(&self) -> MatrixIterator<T>
    {
        MatrixIterator { matrix: self, row: 0, col: 0 }
    }
}
