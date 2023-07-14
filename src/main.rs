pub mod matrix;
pub mod decompositions;
use matrix::Matrix;

use crate::decompositions::lup_square;

fn main() 
{
    let mut matrix = Matrix::zero((3, 3));
    matrix[(0, 0)] = 0.0;
    matrix[(0, 1)] = 5.0;
    matrix[(0, 2)] = 22.0/3.0;
    matrix[(1, 0)] = 4.0;
    matrix[(1, 1)] = 2.0;
    matrix[(1, 2)] = 1.0;
    matrix[(2, 0)] = 2.0;
    matrix[(2, 1)] = 7.0;
    matrix[(2, 2)] = 9.0;


    let (lu, p) = lup_square(&matrix);

    println!("{:?}", lu);
    println!("{:?}", p);
}
