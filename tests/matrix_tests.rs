#[cfg(test)]
mod tests
{
    use redpill::matrix::Mat;
    use redpill::colvecs;
    
    #[test]
    fn mat_test_macro()
    {
        let a = Mat::<f64>::from_vec((2, 2), 
            colvecs![1.0, 3.0; 2.0, 4.0;]
        );

        let mut b = Mat::new((2, 2));
        b[(0, 0)] = 1.0;
        b[(0, 1)] = 2.0;
        b[(1, 0)] = 3.0;
        b[(1, 1)] = 4.0;
        
        assert!(a.approximately(&b, 1e-8));
    }
}