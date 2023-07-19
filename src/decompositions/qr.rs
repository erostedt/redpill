use crate::matrix::Mat;
use crate::vector::Vector;


pub struct QR<T>
{
    pub q: Mat<T>,
    pub r: Mat<T>,
} 


impl Mat<f64>
{
    pub fn qr(mut self) -> QR<f64>
    {
        let (m, n) = self.shape();
        let mut q = Mat::eye(m);
        let mut h = Mat::eye(m);
        let mut v = Vector::<f64>::new(m);

        for i in 0..std::cmp::min(m, n)
        {
            Self::_householder(&mut h, &mut self, &mut v, m, i);
            q._inplace_partial_matmul(&h, i);
            
            self.transpose();
            h.transpose();
            self._inplace_partial_matmul(&h, i);
            self.transpose();
            h.transpose();
        }
        QR::<f64> {q, r: self}
    }

    fn _householder(h: &mut Mat<f64>, r: &mut Mat<f64>, v: &mut Vector<f64>, m: usize, i: usize)
    {
        let mut anorm = 0.0;

        for j in i..m
        {
            let elem = r[(j, i)];
            anorm += elem * elem;
        } 

        v[i] = r[(i, i)] - anorm.sqrt();
        for j in (i+1)..m
        {
            v[j] = r[(j, i)];
        }
        
        let mut vnorm = 0.0; 
        for j in i..m
        {
            vnorm += v[j] * v[j];
        }
    
        if vnorm != 0.0
        {
            let inv_sqrt = 1.0 / vnorm.sqrt();
            for j in i..m
            {
                v[j] *= inv_sqrt;
            }
        }

        for j in 0..i
        {
            for k in 0..m
            {
                h[(j, k)] = 0.0;
                h[(k, j)] = 0.0;
            }
        }

        for j in 0..i
        {
            h[(j, j)] = 1.0;
        }

        for j in i..m
        {
            for k in i..m
            {
                h[(j, k)] = -2.0 * v[j] * v[k];
            }
        }
        
        for j in i..m
        {
            h[(j, j)] += 1.0;
        }

    }

    fn _inplace_partial_matmul(&mut self, rhs: &Mat<f64>, start_col: usize)
    {
        let mut cache = Vector::<f64>::new(self.cols-start_col);
        for i in 0..self.rows
        {
            for j in start_col..self.cols
            {
                cache[j-start_col] = self[(i, j)]; 
            }

            for j in start_col..rhs.cols
            {
                self[(i, j)] = 0.0;
                for k in start_col..self.cols
                {
                    self[(i, j)] += cache[k-start_col] * rhs[(k, j)];
                }
            }
        }
    }
}

impl QR<f64> 
{
    pub fn solve(&mut self, b: &Vector<f64>) -> Vector<f64>
    {
        // Solves a.matmul(x) = b, add checks
        let n = b.len();
        let mut x = Vector::<f64>::new(n);
        self.q.transpose();
        let y = self.q.vecmul(&b);
        self.q.transpose();
        
        for i in (0..=(n-1)).rev() 
        {
            let mut sum = 0.0;
            for j in (i+1)..n
            {
                sum += self.r[(i, j)] * x[j];
            }
            x[i] = (y[i] - sum) / self.r[(i, i)];
        }
        x
        
    }

    pub fn det(&self) -> f64
    {
        (self.q.trace().product::<f64>()).signum() * self.r.trace().product::<f64>()
    }
}