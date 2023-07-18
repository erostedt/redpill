use crate::matrix::Mat;
use crate::vector::Vector;


pub struct LU<T>
{
    pub compact: Mat<T>,
}

impl LU<f64>
{
    // Implement as triangular matrices instead.
    pub fn l(&self) -> Mat<f64>
    {
        let mut lower = Mat::new((self.compact.rows, self.compact.cols));
        for r in 0..self.compact.rows
        {
            for c in 0..r
            {
                let pos = (r, c);
                lower[pos] =  self.compact[pos];
            }  
        }
        for r in 0..self.compact.rows
        {
            lower[(r, r)] = 1.0
        }
        lower
    }

    pub fn u(&self) -> Mat<f64>
    {
        let mut upper = Mat::new((self.compact.rows, self.compact.cols));
        for c in 0..self.compact.cols
        {
            for r in 0..=c
            {
                let pos = (r, c);
                upper[pos] = self.compact[pos];
            }
        }
        upper
    }

    pub fn split(&self) -> (Mat<f64>, Mat<f64>)
    {
        let mut lower = Mat::new((self.compact.rows, self.compact.cols));
        let mut upper = Mat::new((self.compact.rows, self.compact.cols));

        for r in 0..self.compact.rows
        {
            for c in 0..=r
            {
                let lower_pos = (r, c);
                let upper_pos = (c, r);
                lower[lower_pos] = self.compact[lower_pos];
                upper[upper_pos] = self.compact[upper_pos];
            }  
        }
        for r in 0..self.compact.rows
        {
            lower[(r, r)] = 1.0
        }

        (lower, upper)
    }

}

impl Mat<f64>
{
    pub fn lu(mut self) -> LU<f64>
    {
        assert!((self.rows == self.cols));
        let n = self.rows;

        for k in 0..n
        {
            for i in (k + 1)..n
            {
                self[(i, k)] /= self[(k, k)];
                for j in (k+1)..n
                {
                    self[(i, j)] -= self[(i, k)] * self[(k, j)];
                }
            }
        }
        LU {compact: self} 
    }
}

impl LU<f64> 
{
    pub fn solve(&mut self, b: &Vector<f64>) -> Vector<f64>
    {
        // Solves a.matmul(x) = b, add checks
        let n = b.len();
        let mut x = Vector::<f64>::new(n);
        let mut y = Vector::<f64>::new(n);
        
        for i in 0..n
        {
            for k in 0..i
            {
                y[i] += self.compact[(i, k)] * y[k];
            }
            y[i] = b[i] - y[i];
        }
        
        for i in (0..=(n-1)).rev() 
        {
            for k in (i+1)..n
            {
                x[i] += self.compact[(i, k)] * x[k];
            }
            x[i] = (1.0 / self.compact[(i, i)]) * (y[i] - x[i]);
        }
        x
        
    }

    pub fn inv(&self) -> Mat<f64>
    {
        assert!(self.compact.rows == self.compact.cols);
        let n = self.compact.rows;
        let mut inverse = Mat::new((self.compact.rows, self.compact.cols));

        for j in 0..n
        {
            for i in 0..n 
            {
                if i == j
                {
                    inverse[(i, j)] = 1.0; 
                }
                else 
                {
                    inverse[(i, j)] = 0.0;    
                }
                
                for k in 0..i
                {
                    inverse[(i, j)] -= self.compact[(i, k)] * inverse[(k, j)];
                }
            }

            for i in (0..n).rev() 
            {
                for k in (i + 1)..n
                {
                    inverse[(i, j)] -= self.compact[(i, k)] * inverse[(k, j)];
                }

                inverse[(i, j)] /= self.compact[(i, i)];
            }
        }
        inverse
    }

    pub fn det(&self) -> f64
    {
        self.compact.trace().product()
    }
}