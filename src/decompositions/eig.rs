use crate::matrix::Mat;
use crate::vector::Vector;


pub struct EIG<T>
{
    pub eigvals: Vector<T>,
    pub eigvecs: Mat<T>,
} 


impl Mat<f64>
{
    pub fn eig(mut self, max_iter: usize) -> EIG<f64>
    {
        assert!(self.rows == self.cols);

        let n = self.rows;
        let mut eigvals = Vector::new(n);
        let mut eigvecs = Mat::eye(n);
        
        for _ in 0..max_iter
        {
            let mu = self[(self.rows-1, self.cols-1)];
            for j in 0..n
            {
                self[(j, j)] -= mu;
            }

            let qr = self.qr();
            self = qr.r.matmul(&qr.q);
            for j in 0..n
            {
                self[(j, j)] += mu;
            }

            eigvecs = eigvecs.matmul(&qr.q);
        }

        for i in 0..n
        {
            eigvals[i] = self[(i, i)];
        }

        EIG{eigvals, eigvecs}
    }
}