use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};
use std::iter::Iterator;
use crate::vector::Vector;

#[derive(Clone, Debug)]
pub struct Mat<T>
{
    pub rows: usize,
    pub cols: usize,
    row_stride: usize,
    col_stride: usize,
    elements: Vec<T>,
}

pub struct TraceIter<'a, T>
{
    pub matrix: &'a Mat<T>,
    current: usize,
    min_dim: usize,
}

pub struct SubMatIter<'a, T>
{
    pub matrix: &'a Mat<T>,
    rmax: usize,
    cmin: usize,
    cmax: usize,
    current_row: usize,
    current_col: usize,
}

impl<'a, T> Iterator for TraceIter<'a, T>
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.current >= self.min_dim
        {
            return None;
        }

        let elem = Some(&self.matrix[(self.current, self.current)]);
        self.current += 1;
        elem
    }    
}


impl<'a, T> Iterator for SubMatIter<'a, T>
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.rmax >= self.current_row
        {
            return None;
        }

        let elem = Some(&self.matrix[(self.current_row, self.current_col)]);
        
        self.current_col += 1;
        if self.current_col >= self.cmax
        {
            self.current_row += 1;
            self.current_col = self.cmin;
        }
        elem
    }    
}


#[macro_export]
macro_rules! colvecs {
    ($([$tail:expr,];)* -> [$($head:expr,)*]) => (
        vec![$($head,)* $($tail,)*]
    );
    ($([$middle:expr, $($tail:expr,)*];)* -> [$($head:expr,)*]) => (
        colvecs!($([$($tail,)*];)* -> [$($head,)* $($middle,)*])
    );
    ($($($item:expr),*;)*) => (
        colvecs!($([$($item,)*];)* -> [])
    );
    ($($($item:expr,)*;)*) => (
        colvecs!($([$($item,)*];)* -> [])
    );
}

impl<T> Mat<T>
where T: Default + Clone + Sized
{
    pub fn new(shape: (usize, usize)) -> Self
    {
        Self {rows: shape.0, cols: shape.1, row_stride: shape.1, col_stride: 1, elements: vec![T::default(); shape.0 * shape.1]}
    }

    pub fn from_vec(shape: (usize, usize), vec: Vec::<T>) -> Self
    {
        assert!(shape.0*shape.1 == vec.len());
        Self { rows: shape.0, cols: shape.1, row_stride: shape.1, col_stride: 1, elements: vec }
    }


    pub fn shape(&self) -> (usize, usize)
    {
        (self.rows, self.cols)
    }

    pub fn iter_trace(&self) -> TraceIter<T>
    {
        let n = std::cmp::min(self.rows, self.cols);
        TraceIter { matrix: self, current: 0, min_dim: n }
    }

    pub fn iter_submat(&self, rmin: usize, rmax: usize, cmin: usize, cmax: usize) -> Result<SubMatIter<T>, &str>
    {
        if (rmin > rmax) || (cmin > cmax) || (rmax > self.rows) || (cmax > self.cols)
        {
            return Err("Invalid submatrix.");
        }
        Ok(SubMatIter { matrix: self, rmax, cmin, cmax, current_row: rmin, current_col: cmin })
    }

    pub fn submat(&self, rmin: usize, rmax: usize, cmin: usize, cmax: usize) -> Result<Mat<T>, &str>
    {
        if (rmin > rmax) || (cmin > cmax) || (rmax > self.rows) || (cmax > self.cols)
        {
            return Err("Invalid submatrix.");
        }

        let rows = rmax - rmin;
        let cols = cmax - cmin;
        let mut sub = Mat::<T>::new((rows, cols));
        
        for row in 0..rows
        {
            for col in 0..cols
            {
                sub[(row, col)] = self[(row + rmin, (col + cmin))].clone();
            }
        }
        Ok(sub)
    }

}

impl<T> PartialEq<Mat<T>> for Mat<T>
where T: PartialEq
{
    fn eq(&self, other: &Mat<T>) -> bool 
    {
        if self.rows != other.rows || self.cols != other.cols
        {
            return false;
        }

        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                if self[pos] != other[pos]
                {
                    return false;
                }
            }
        }
        true
    }

    fn ne(&self, other: &Mat<T>) -> bool 
    {
        !(self == other)
    }
}

impl Mat<f64>
{
    pub fn approximately(&self, other: &Mat<f64>, tol: f64) -> bool
    {
        assert!((self.rows == other.rows) && (self.cols == other.cols));
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                if (self[pos] - other[pos]).abs() > tol 
                {
                    return false;
                }
            }
        }
        true
    }
}


impl<T> Index<(usize, usize)> for Mat<T>
{
    type Output = T;
    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output
    {
        &self.elements[index.0 * self.row_stride + index.1 * self.col_stride]
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T>
{
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T
    {
        &mut self.elements[index.0 * self.row_stride + index.1 * self.col_stride]
    }    
}

impl<T> Add<Mat<T>> for Mat<T>
where T: Add<Output = T> + Copy + Default
{
    type Output = Mat<T>;
    fn add(self, rhs: Mat<T>) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Mat::new(self.shape());
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

impl<T> Add<T> for Mat<T>
where T: Add<Output = T> + Copy + Default
{
    type Output = Mat<T>;
    fn add(self, rhs: T) -> Self::Output 
    {
        let mut out = Mat::new(self.shape());
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

impl<T> AddAssign<Mat<T>> for Mat<T>
where T: AddAssign + Copy
{
    fn add_assign(&mut self, rhs: Mat<T>) 
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


impl<T> AddAssign<T> for Mat<T>
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


impl<T> Sub<Mat<T>> for Mat<T> 
where T: Sub<Output = T> + Copy + Default
{
    type Output = Mat<T>;
    fn sub(self, rhs: Mat<T>) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Mat::new(self.shape());   
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


impl<T> Sub<T> for Mat<T> 
where T: Sub<Output = T> + Copy + Default
{
    type Output = Mat<T>;
    fn sub(self, rhs: T) -> Self::Output 
    {
        let mut out = Mat::new(self.shape());
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


impl<T> SubAssign<Mat<T>> for Mat<T>
where T: SubAssign + Copy
{
    fn sub_assign(&mut self, rhs: Mat<T>) 
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


impl<T> SubAssign<T> for Mat<T>
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


impl<T> Mul<Mat<T>> for Mat<T>
where T: Mul<Output = T> + Copy + Default
{
    type Output = Mat<T>;
    fn mul(self, rhs: Mat<T>) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Mat::new(self.shape());
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

impl<T> Mul<Vector<T>> for Mat<T>
where T: AddAssign + Mul<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output 
    {
        assert!(self.cols == rhs.len());
        let mut out = Vector::new(self.rows);
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                out[row] += self[pos] * rhs[col];
            }
        }
        out
    }    
}

impl<T> Mul<T> for Mat<T>
where T: Mul<Output = T> + Copy + Default
{
    type Output = Mat<T>;
    fn mul(self, rhs: T) -> Self::Output 
    {
        let mut out = Mat::new(self.shape());
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

impl<T> MulAssign<Mat<T>> for Mat<T>
where T: MulAssign + Copy
{
    fn mul_assign(&mut self, rhs: Mat<T>) 
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


impl<T> MulAssign<T> for Mat<T>
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


impl<T> Div<Mat<T>> for Mat<T>
where T: Div<Output = T> + Copy + Default
{
    type Output = Mat<T>;
    fn div(self, rhs: Mat<T>) -> Self::Output 
    {
        assert!((self.rows == rhs.rows) && (self.cols == rhs.cols));
        let mut out = Mat::new(self.shape());   
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


impl<T> Div<T> for Mat<T> 
where T: Div<Output = T> + Copy + Default
{
    type Output = Mat<T>;
    fn div(self, rhs: T) -> Self::Output 
    {
        let mut out = Mat::new(self.shape());
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


impl<T> DivAssign<Mat<T>> for Mat<T>
where T: DivAssign + Copy
{
    fn div_assign(&mut self, rhs: Mat<T>) 
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


impl<T> DivAssign<T> for Mat<T>
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

impl<T> Mat<T>
where T: AddAssign + Mul<Output = T> + Copy + Default
{
    pub fn matmul(&self, rhs: &Mat<T>) -> Mat<T>
    {
        assert!(self.cols == rhs.rows);
        let mut out = Mat::<T>::new((self.rows, rhs.cols));
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

    pub fn vecmul(&self, rhs: &Vector<T>) -> Vector<T>
    {
        assert!(self.cols == rhs.len());
        let mut out = Vector::new(self.rows);
        for row in 0..self.rows
        {
            for col in 0..self.cols
            {
                let pos = (row, col);
                out[row] += self[pos] * rhs[col];
            }
        }
        out
    }    
}

impl<T> Mat<T>
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
impl <T> Mat<T>
{
    pub fn iter(&self) -> std::slice::Iter<'_, T>
    {
        self.elements.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T>
    {
        self.elements.iter_mut()
    }

    pub fn transpose_self(mut self) -> Self
    {   
        let mut temp = self.rows;
        self.rows = self.cols;
        self.cols = temp;
        
        temp = self.row_stride;
        self.row_stride = self.col_stride;
        self.col_stride = temp;
        self
    } 
    
    pub fn transpose(&mut self)
    {   
        let mut temp = self.rows;
        self.rows = self.cols;
        self.cols = temp;
        
        temp = self.row_stride;
        self.row_stride = self.col_stride;
        self.col_stride = temp;
    }
}

impl <T> Mat<T> 
where T: Clone
{
    pub fn transposed(&self) -> Mat<T>
    {
        let mat = self.clone();
        mat.transpose_self()
    }
}

impl Mat<f64>
{
    pub fn eye(size: usize) -> Mat<f64>
    {
        let mut mat = Mat::<f64>::new((size, size));
        for r in 0..size
        {
            mat[(r, r)] = 1.0;
        }
        mat
    }
}