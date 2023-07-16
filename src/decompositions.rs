use crate::matrix::Matrix;
use crate::vector::Vector;

pub trait LU
{
    fn lup(&mut self) -> Vec<usize>;
    fn lu(self) -> Self;
}

impl LU for Matrix<f64>
{
    fn lup(&mut self) -> Vec<usize> 
    {

        assert!((self.rows == self.cols));
        let n = self.rows;
        let mut perm = (0..n).collect::<Vec<usize>>();

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
        perm
    }

    fn lu(mut self) -> Self
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
        self
    }
}

pub fn solve(a: &mut Matrix<f64>, b: &Vector<f64>) -> Vector<f64>
{
    // Solves a.matmul(x) = b, add checks
    let mut x = Vector::<f64>::new(b.size);
    let perm = a.lup();
    let n = perm.len();
    
    for i in 0..n
    {
        x[i] = b[perm[i]];

        for k in 0..i
        {
            x[i] -= a[(i, k)] * x[k];
        }
    }

    for i in (0..=(n-1)).rev() 
    {
        for k in (i+1)..n
        {
            x[i] -= a[(i, k)] * x[k];
        }
        x[i] /= a[(i, i)];
    }
    x

}
