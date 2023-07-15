#[cfg(test)]
mod tests
{
    use redpill::matrix::Matrix;
    use redpill::decompositions::lup_square;
    
    #[test]
    fn lup_test()
    {
        let matrix = Matrix::from_vec((3, 3), 
            vec![0.0, 5.0, 22.0/3.0, 4.0, 2.0, 1.0, 2.0, 7.0, 9.0,]
        );
        
        let (lu, p) = lup_square(&matrix);

        let lu_true = Matrix::from_vec((3, 3), 
            vec![4.0, 2.0, 1.0, 1.0/2.0, 6.0, 17.0/2.0, 0.0, 5.0/6.0, 1.0/4.0]
        );

        let p_true = vec![1, 2, 0];

        assert!(lu.approximately(&lu_true, 1e-8));
        assert_eq!(p, p_true)
    }
}

