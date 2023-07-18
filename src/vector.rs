use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Sub, SubAssign};


#[derive(Clone, Debug)]
pub struct Vector<T>
{
    elements: Vec<T>,
}

impl<T> Vector<T> 
where T: Default + Clone
{
    pub fn new(size: usize) -> Self
    {
        Self { elements: vec![T::default(); size] }
    }

    pub fn from_vec(vec: Vec<T>) -> Self
    {
        Self { elements: vec }
    }
}

impl <T> Vector<T> 
{
    pub fn len(&self) -> usize
    {
        self.elements.len()
    }
    
    pub fn iter(&self) -> std::slice::Iter<T>
    {
        self.elements.iter()
    }
    
    pub fn iter_mut(&mut self) -> std::slice::IterMut<T>
    {
        self.elements.iter_mut()
    }
}


impl<T> Index<usize> for Vector<T> 
{
    type Output = T;
    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output 
    {
        &self.elements[index]
    }    
}

impl<T> IndexMut<usize> for Vector<T> 
{
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut T 
    {
        &mut self.elements[index]
    }    
}

impl<T> Add<Vector<T>> for Vector<T>
where T: Add<Output = T> + Copy + Default
{
    type Output = Vector<T>;
    fn add(self, rhs: Vector<T>) -> Self::Output 
    {
        assert!((self.len() == rhs.len()));
        let mut out = Vector::<T>::new(self.len());
        for i in 0..self.len()
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
        let mut out = Vector::new(self.len());
        for i in 0..self.len()
        {
            out[i] = self[i] + rhs;
        }
        out
    }
}

impl<T> AddAssign<Vector<T>> for Vector<T>
where T: AddAssign + Copy + Clone + Default
{
    fn add_assign(&mut self, rhs: Vector<T>) 
    {
        assert!(self.len() == rhs.len());
        for i in 0..self.len()
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
        for i in 0..self.len()
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
        assert!(self.len() == rhs.len());
        let mut out = Vector::new(self.len());   
        for i in 0..self.len()
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
        let mut out = Vector::new(self.len());
        for i in 0..self.len()
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
        assert!(self.len() == rhs.len());
        for i in 0..self.len()
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
        for i in 0..self.len()
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
        assert!(self.len() == rhs.len());
        let mut out = Vector::new(self.len());
        for i in 0..self.len()
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
        let mut out = Vector::new(self.len());
        for i in 0..self.len()
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
        assert!(self.len() == rhs.len());
        for i in 0..self.len()
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
        for i in 0..self.len()
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
        assert!(self.len() == rhs.len());
        let mut out = Vector::new(self.len());   
        for i in 0..self.len()
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
        let mut out = Vector::new(self.len());
        for i in 0..self.len()
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
        assert!((self.len() == rhs.len()));
        for i in 0..self.len()
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
        for i in 0..self.len()
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
        assert!(self.len() == other.len());
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
        assert!(self.len() == other.len());
        self.clone() * self.dot(other) / other.mag_sq()   
    }
}
impl<T> Vector<T>
where T: Default + AddAssign + Copy + Mul<Output = T>
{   
    fn mag_sq(&self) -> T
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
where T: Default + Copy + AddAssign
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
where T: Default + Clone + Copy + Mul<Output = T> + Sub<Output = T>
{
    pub fn cross(&self, other: &Vector<T>) -> Vector<T>
    {
        assert!(self.len() == other.len());
        assert!(self.len() == 3);
        
        let mut res = Vector::new(self.len());
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

    pub fn approximately(&self, other: &Self, tol: f64) -> bool
    {
        if self.len() != other.len()
        {
            return false;
        }

        for i in 0..self.len()
        {
            if (self[i] - other[i]).abs() > tol
            {
                return false;
            } 
        }
        true
    }
}