#[cfg(test)]
mod tests
{
    use redpill::matrix::Mat;
    use redpill::vector::Vector;
    
    #[test]
    fn lu_test()
    {
        let matrix = Mat::from_vec((3, 3), 
            vec![1.0, 1.0, 1.0, 4.0, 3.0, -1.0, 3.0, 5.0, 3.0,]
        );
        
        let lu_true = Mat::from_vec((3, 3), 
            vec![1.0, 1.0, 1.0, 4.0, -1.0, -5.0, 3.0, -2.0, -10.0]
        );

        
        let lu = matrix.clone().lu();
        assert!(matrix.approximately(&lu.l().matmul(&lu.u()), 1e-8));
        let (l, u) = lu.split();
        assert!(matrix.approximately(&l.matmul(&u), 1e-8));
        assert!(lu.compact.approximately(&lu_true, 1e-8));
    }

    #[test]
    fn lu_solve_test()
    {
        let matrix = Mat::from_vec((2, 2), 
            vec![-3.0, 2.0, 5.0, -2.0],
        );

        let b = Vector::from_vec(vec![-2.0, 7.0]);

        let x = matrix.lu().solve(&b);
        let x_true = Vector::from_vec(vec![5.0/2.0, 11.0/4.0]);
        assert!(x.approximately(&x_true, 1e-8));
    }

    #[test]
    fn lu_inv_test()
    {
        let matrix = Mat::from_vec((3, 3), 
            vec![1.0, 1.0, 1.0, 4.0, 3.0, -1.0, 3.0, 5.0, 3.0,]
        );

        let inv = matrix.clone().lu().inv();
        assert!(matrix.matmul(&inv).approximately(&Mat::eye(3), 1e-8));
    }

    #[test]
    fn lu_det_test()
    {
        let matrix = Mat::from_vec((3, 3), 
            vec![2.0, -3.0, 1.0, 2.0, 0.0, -1.0, 1.0, 4.0, 5.0,]
        );

        assert!(((matrix.lu().det() - 49.0).abs() < 1e-8));
    }

    
}

