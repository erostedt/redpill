use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};


#[derive(Clone, Debug, PartialEq)]
pub struct Vector<T>
{
    pub size: usize,
    offset: usize,
    elements: Vec<T>,
}

pub struct VectorIterator<'a, T>
{
    vector: &'a Vector<T>,
    current: usize,
}


impl<T> Vector<T>
where T: Default + Clone
{
    pub fn new(size: usize) -> Self
    {
        Self {size, offset: 0, elements: vec![T::default(); size]}
    }
}


impl<T> Index<usize> for Vector<T>
{
    type Output = T;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output
    {
        &self.elements[self.offset + index]
    }
}

impl<T> IndexMut<usize> for Vector<T>
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut T
    {
        &mut self.elements[self.offset + index]
    }    
}

impl<T> Add<Vector<T>> for Vector<T>
where T: Add<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output 
    {
        assert!((self.size == rhs.size));
        let mut out = Vector::new(self.size);
        for i in 0..self.size
        {
            out[i] = self[i] + rhs[i];       
        }
        out
    }
}

impl<T> Add<T> for Vector<T>
where T: Add<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn add(self, rhs: T) -> Self::Output 
    {
        let mut out = Vector::new(self.size);
        for i in 0..self.size
        {
            out[i] = self[i] + rhs;
        }
        out
    }
}

impl<T> AddAssign<Vector<T>> for Vector<T>
where T: AddAssign + Copy
{
    fn add_assign(&mut self, rhs: Vector<T>) 
    {
        assert!(self.size == rhs.size);
        for i in 0..self.size
        {
            self[i] += rhs[i];
        }  
    }
}


impl<T> AddAssign<T> for Vector<T>
where T: AddAssign + Copy
{
    fn add_assign(&mut self, rhs: T) 
    {
        for i in 0..self.size
        {
            self[i] += rhs;
        }
    }
}


impl<T> Sub<Vector<T>> for Vector<T> 
where T: Sub<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn sub(self, rhs: Vector<T>) -> Self::Output 
    {
        assert!(self.size == rhs.size);
        let mut out = Vector::new(self.size);   
        for i in 0..self.size
        {
            out[i] = self[i] - rhs[i];
        }
        out    
    }
}


impl<T> Sub<T> for Vector<T> 
where T: Sub<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn sub(self, rhs: T) -> Self::Output 
    {
        let mut out = Vector::new(self.size);
        for i in 0..self.size
        {
            out[i] = self[i] - rhs;
        }
        out
    }
}


impl<T> SubAssign<Vector<T>> for Vector<T>
where T: SubAssign + Copy
{
    fn sub_assign(&mut self, rhs: Vector<T>) 
    {
        assert!(self.size == rhs.size);
        for i in 0..self.size
        {
            self[i] -= rhs[i];
        }
    }
}


impl<T> SubAssign<T> for Vector<T>
where T: SubAssign + Copy
{
    fn sub_assign(&mut self, rhs: T) 
    {
        for i in 0..self.size
        {
            self[i] -= rhs;
        }    
    }
}


impl<T> Mul<Vector<T>> for Vector<T>
where T: Mul<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn mul(self, rhs: Vector<T>) -> Self::Output 
    {
        assert!(self.size == rhs.size);
        let mut out = Vector::new(self.size);
        for i in 0..self.size
        {
            out[i] = self[i] * rhs[i];       
        }
        out
    }
}

impl<T> Mul<T> for Vector<T>
where T: Mul<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn mul(self, rhs: T) -> Self::Output 
    {
        let mut out = Vector::new(self.size);
        for i in 0..self.size
        {
            out[i] = self[i] * rhs;
        }
        out
    }
}

impl<T> MulAssign<Vector<T>> for Vector<T>
where T: MulAssign + Copy
{
    fn mul_assign(&mut self, rhs: Vector<T>) 
    {
        assert!(self.size == rhs.size);
        for i in 0..self.size
        {
            self[i] *= rhs[i];
        }  
    }
}


impl<T> MulAssign<T> for Vector<T>
where T: MulAssign + Copy
{
    fn mul_assign(&mut self, rhs: T) 
    {
        for i in 0..self.size
        {
            self[i] *= rhs;
        }
    }
}


impl<T> Div<Vector<T>> for Vector<T>
where T: Div<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn div(self, rhs: Vector<T>) -> Self::Output 
    {
        assert!(self.size == rhs.size);
        let mut out = Vector::new(self.size);   
        for i in 0..self.size
        {
            out[i] = self[i] / rhs[i];
        }
        out    
    }
}


impl<T> Div<T> for Vector<T> 
where T: Div<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn div(self, rhs: T) -> Self::Output 
    {
        let mut out = Vector::new(self.size);
        for i in 0..self.size
        {
            out[i] = self[i] / rhs;
        }
        out
    }
}


impl<T> DivAssign<Vector<T>> for Vector<T>
where T: DivAssign + Copy
{
    fn div_assign(&mut self, rhs: Vector<T>) 
    {
        assert!((self.size == rhs.size));
        for i in 0..self.size
        {
            self[i] /= rhs[i]
        }
    }
}


impl<T> DivAssign<T> for Vector<T>
where T: DivAssign + Copy
{
    fn div_assign(&mut self, rhs: T) 
    {
        for i in 0..self.size
        {
            self[i] /= rhs;
        }    
    }
}

impl<'a, T> Iterator for VectorIterator<'a, T>
{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> 
    {
        if self.current >= self.vector.size 
        {
            return None;
        }
        
        let res = Some(&self.vector[self.current]);
        self.current += 1;
        res
    }
}


impl<T> Vector<T>
{
    pub fn iter(&self) -> VectorIterator<T>
    {
        VectorIterator { vector: self, current: 0, }
    }
}
