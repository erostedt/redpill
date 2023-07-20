use crate::{matrix::Mat, vector::Vector};

pub struct LUP<T>
{
    pub compact: Mat<T>,
    pub perm: Vec<usize>,
    pub num_swaps: usize
} 

impl Mat<f64>
{
    pub fn lup(mut self) -> LUP<f64> 
    {
        
        assert!((self.rows == self.cols));
        let n = self.rows;
        let mut perm = (0..n).collect::<Vec<usize>>();
        let mut num_swaps: usize = 0;

        for k in 0..n
        {
            let mut p = k;
            for i in (k + 1)..n
            {
                if self[(i, k)].abs() > self[(p, k)].abs()
                {
                    p = i;
                }
            }
            
            if p != k
            {
                self.swap_rows(k, p);
                perm.swap(k, p);
                num_swaps += 1;
            }

            for i in (k + 1)..n
            {
                self[(i, k)] /= self[(k, k)];
                for j in (k+1)..n
                {
                    self[(i, j)] -= self[(i, k)] * self[(k, j)];
                }
            }
        }
        LUP{ compact: self, perm, num_swaps}
    }
}

impl LUP<f64>
{
    pub fn solve(&mut self, b: &Vector<f64>) -> Vector<f64>
    {
        // Solves a.matmul(x) = b, add checks
        let mut x = Vector::new(b.len());
        let n = self.perm.len();
        
        for i in 0..n
        {
            x[i] = b[self.perm[i]];
            
            for k in 0..i
            {
                x[i] -= self.compact[(i, k)] * x[k];
            }
        }
        
        for i in (0..=(n-1)).rev() 
        {
            for k in (i+1)..n
            {
                x[i] -= self.compact[(i, k)] * x[k];
            }
            x[i] /= self.compact[(i, i)];
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
                if self.perm[i] == j
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
        let mut d: f64 = self.compact.iter_trace().product();
        if self.num_swaps % 2 != 0
        {
            d = -d;
        }
        d
    }
}