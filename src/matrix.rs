use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix
{
    pub rows: usize,
    pub cols: usize,
    offset: usize,
    row_stride: usize,
    col_stride: usize,
    elements: Vec<f64>,
}

impl Matrix
{
    pub fn zero(shape: (usize, usize)) -> Self
    {
        Self {rows: shape.0, cols: shape.1, offset: 0, row_stride: shape.1, col_stride: 1, elements: vec![0.0; shape.0 * shape.1]}
    }

    pub fn shape(&self) -> (usize, usize)
    {
        (self.rows, self.cols)
    }
}


impl Index<(usize, usize)> for Matrix
{
    type Output = f64;
    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output
    {
        &self.elements[self.offset + index.0 * self.row_stride + index.1 * self.col_stride]
    }
}

impl IndexMut<(usize, usize)> for Matrix
{
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut f64
    {
        &mut self.elements[self.offset + index.0 * self.row_stride + index.1 * self.col_stride]
    }    
}

impl Add<Matrix> for Matrix
{
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Matrix::zero(self.shape());
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

impl Add<f64> for Matrix 
{
    type Output = Matrix;
    fn add(self, rhs: f64) -> Self::Output 
    {
        let mut out = Matrix::zero(self.shape());
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

impl Add<Matrix> for f64
{
    type Output = Matrix;
    fn add(self, rhs: Matrix) -> Self::Output 
    {
        rhs + self   
    }
}

impl AddAssign<Matrix> for Matrix 
{
    fn add_assign(&mut self, rhs: Matrix) 
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


impl AddAssign<f64> for Matrix 
{
    fn add_assign(&mut self, rhs: f64) 
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


impl Sub<Matrix> for Matrix 
{
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Matrix::zero(self.shape());   
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


impl Sub<f64> for Matrix 
{
    type Output = Matrix;
    fn sub(self, rhs: f64) -> Self::Output 
    {
        let mut out = Matrix::zero(self.shape());
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


impl Sub<Matrix> for f64 
{
    type Output = Matrix;
    fn sub(self, rhs: Matrix) -> Self::Output 
    {
        let mut out = Matrix::zero(rhs.shape());
        for row in 0..rhs.rows
        {
            for col in 0..rhs.cols
            {
                let pos = (row, col);
                out[pos] = self - rhs[pos];     
            }
        }
        out
    }    
}


impl SubAssign<Matrix> for Matrix
{
    fn sub_assign(&mut self, rhs: Matrix) 
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


impl SubAssign<f64> for Matrix
{
    fn sub_assign(&mut self, rhs: f64) 
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


impl Mul<Matrix> for Matrix
{
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Matrix::zero(self.shape());
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

impl Mul<f64> for Matrix 
{
    type Output = Matrix;
    fn mul(self, rhs: f64) -> Self::Output 
    {
        let mut out = Matrix::zero(self.shape());
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

impl Mul<Matrix> for f64
{
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output 
    {
        rhs * self   
    }
}

impl MulAssign<Matrix> for Matrix 
{
    fn mul_assign(&mut self, rhs: Matrix) 
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


impl MulAssign<f64> for Matrix 
{
    fn mul_assign(&mut self, rhs: f64) 
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


impl Div<Matrix> for Matrix 
{
    type Output = Matrix;
    fn div(self, rhs: Matrix) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Matrix::zero(self.shape());   
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


impl Div<f64> for Matrix 
{
    type Output = Matrix;
    fn div(self, rhs: f64) -> Self::Output 
    {
        let mut out = Matrix::zero(self.shape());
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


impl Div<Matrix> for f64 
{
    type Output = Matrix;
    fn div(self, rhs: Matrix) -> Self::Output 
    {
        let mut out = Matrix::zero(rhs.shape());
        for row in 0..rhs.rows
        {
            for col in 0..rhs.cols
            {
                let pos = (row, col);
                out[pos] = self / rhs[pos];     
            }
        }
        out
    }    
}


impl DivAssign<Matrix> for Matrix
{
    fn div_assign(&mut self, rhs: Matrix) 
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


impl DivAssign<f64> for Matrix
{
    fn div_assign(&mut self, rhs: f64) 
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


impl Matrix 
{
    pub fn matmul(&self, rhs: Matrix) -> Matrix
    {
        assert!(self.cols == rhs.rows);
        let mut out = Matrix::zero((self.rows, rhs.cols));
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