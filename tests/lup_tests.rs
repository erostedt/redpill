#[cfg(test)]
mod tests
{
    use redpill::matrix::Mat;
    use redpill::vector::Vector;
    
    #[test]
    fn lup_test()
    {
        let matrix = Mat::from_vec((3, 3), 
            vec![0.0, 5.0, 22.0/3.0, 4.0, 2.0, 1.0, 2.0, 7.0, 9.0,]
        );
        
        let lup = matrix.lup();

        let lu_true = Mat::from_vec((3, 3), 
            vec![4.0, 2.0, 1.0, 1.0/2.0, 6.0, 17.0/2.0, 0.0, 5.0/6.0, 1.0/4.0]
        );

        let p_true = vec![1, 2, 0];

        assert!(lup.compact.approximately(&lu_true, 1e-8));
        assert_eq!(lup.perm, p_true)
    }
    
    #[test]
    fn lup_solve_test()
    {
        let matrix = Mat::from_vec((2, 2), 
            vec![-3.0, 2.0, 5.0, -2.0],
        );

        let b = Vector::from_vec(vec![-2.0, 7.0]);

        let x = matrix.lup().solve(&b);
        let x_true = Vector::from_vec(vec![5.0/2.0, 11.0/4.0]);
        assert!(x.approximately(&x_true, 1e-8));
    }

    #[test]
    fn lup_inv_test()
    {
        let matrix = Mat::from_vec((3, 3), 
            vec![1.0, 1.0, 1.0, 4.0, 3.0, -1.0, 3.0, 5.0, 3.0,]
        );

        let inv = matrix.clone().lup().inv();
        assert!(matrix.matmul(inv).approximately(&Mat::eye(3), 1e-8));
    }

    #[test]
    fn lup_det_test()
    {
        let matrix = Mat::from_vec((3, 3), 
            vec![2.0, -3.0, 1.0, 2.0, 0.0, -1.0, 1.0, 4.0, 5.0,]
        );

        assert!(((matrix.lup().det() - 49.0).abs() < 1e-8));
    }
}

