#[cfg(test)]
mod tests
{
    use redpill::matrix::Mat;
    use redpill::vector::Vector;
    
    #[test]
    fn qr_test()
    {
        let matrix = Mat::from_vec((3, 3), 
            vec![1.0, 1.0, 1.0, 4.0, 3.0, -1.0, 3.0, 5.0, 3.0,]
        );
        
        
        let qr = matrix.clone().qr();
        assert!(matrix.approximately(&qr.q.matmul(&qr.r), 1e-8));
    }


    #[test]
    fn qr_solve_test()
    {
        let matrix = Mat::from_vec((2, 2), 
            vec![-3.0, 2.0, 5.0, -2.0],
        );
        
        let b = Vector::from_vec(vec![-2.0, 7.0]);
        
        let x = matrix.qr().solve(&b);
        let x_true = Vector::from_vec(vec![5.0/2.0, 11.0/4.0]);
        assert!(x.approximately(&x_true, 1e-8));
    }
    
    #[test]
    fn qr_det_test()
    {
        let matrix = Mat::from_vec((3, 3), 
            vec![2.0, -3.0, 1.0, 2.0, 0.0, -1.0, 1.0, 4.0, 5.0,]
        );

        assert!(((matrix.qr().det() - 49.0).abs() < 1e-8));
    }
    
}


