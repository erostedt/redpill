use crate::matrix::Matrix;



pub fn lup_square_inplace(matrix: &mut Matrix<f64>) -> Vec<usize>
{
    assert!((matrix.rows == matrix.cols));
    let n = matrix.rows;
    let mut perm = (0..n).collect::<Vec<usize>>();

    for k in 0..n
    {
        let mut p = k;
        for i in (k + 1)..n
        {
            if matrix[(i, k)].abs() > matrix[(p, k)].abs()
            {
                p = i;
            }
        }

        if p != k
        {
            matrix.swap_rows(k, p);
            perm.swap(k, p);
        }

        for i in (k + 1)..n
        {
            matrix[(i, k)] /= matrix[(k, k)];
            for j in (k+1)..n
            {
                matrix[(i, j)] -= matrix[(i, k)] * matrix[(k, j)];
            }
        }
    }
    perm
}

pub fn lup_square(matrix: &Matrix<f64>) -> (Matrix<f64>, Vec<usize>)
{
    let mut mat = matrix.clone();
    let perm = lup_square_inplace(&mut mat);
    (mat, perm)
}