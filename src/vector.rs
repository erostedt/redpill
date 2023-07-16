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

//pub struct VectorIteratorMut<'a, T: 'a>
//{
//    vector: &'a mut Vector<T>,
//    current: usize,
//}


impl<T> Vector<T>
where T: Default + Clone
{
    pub fn new(size: usize) -> Self
    {
        Self {size, offset: 0, elements: vec![T::default(); size] }
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


impl<T> Vector<T>  
where T: Default + AddAssign + Mul<Output = T> + Copy
{
    pub fn dot(&self, other: &Vector<T>) -> T
    {
        assert!(self.size == other.size);
        let mut res = T::default();
        for (x1, x2) in self.iter().zip(other.iter())
        {
            res += (*x1) * (*x2); 
        }
        res
    }
}

impl<T> Vector<T>
where T: Default + AddAssign + Mul<Output = T> + Copy + Div<Output = T>
{
     pub fn proj(&self, other: &Vector<T>) -> Vector<T>
    {
        assert!(self.size == other.size);
        self.clone() * self.dot(other) / other.mag_sq()   
    }
}

impl<'a, T> Iterator for VectorIterator<'a, T>
{
    type Item = &'a T;
    fn next(& mut self) -> Option<Self::Item> 
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

//impl<'a, T> Iterator for VectorIteratorMut<'a, T>
//where T: Default
//{
//    type Item = &'a mut T;
//    fn next(&mut self) -> Option<Self::Item> 
//    {
//        if self.current >= self.vector.size
//        {
//            return None;
//        }
//        let res = Some(&mut self.vector[self.current]);
//        self.current += 1;
//        res
//    }
//}


impl<T> Vector<T>
{
    pub fn iter(&self) -> VectorIterator<T>
    {
        VectorIterator { vector: self, current: 0, }
    }

    //pub fn iter_mut(&mut self) -> VectorIteratorMut<T>
    //{
    //   VectorIteratorMut { vector: self, current: 0 , _boo: PhantomData }
    //} 
}

impl <T> Vector<T> 
where T: AddAssign + Mul<Output = T> + Default + Copy
{
    pub fn mag_sq(&self) -> T
    {
        let mut res = T::default();
        for x in self.iter()
        {
            res += (*x) * (*x);
        }
        res
    }
}

impl<T> Vector<T> 
where T: AddAssign + Default + Copy
{
    pub fn sum(&self) -> T
    {
        let mut res = T::default();
        for x in self.iter()
        {
            res += *x;
        }
        res
    }
}

impl<T> Vector<T> 
where T: Mul<Output = T> + Sub<Output = T> + Default + Copy
{
    pub fn cross(&self, other: &Vector<T>) -> Vector<T>
    {
        assert!(self.size == other.size);
        assert!(self.size == 3);
        
        let mut res = Vector::<T>::new(self.size);
        res[0] = self[1] * other[2] - self[2] * other[1];
        res[1] = self[0] * other[2] - self[2] * other[0];
        res[2] = self[0] * other[1] - self[1] * other[0];
        res
    }

    
}

impl Vector<f64> 
{
    pub fn mag(&self) -> f64
    {
        self.mag_sq().sqrt()
    }
}

impl Vector<f32>
{
    pub fn mag(&self) -> f32
    {
        self.mag_sq().sqrt()
    }
}

