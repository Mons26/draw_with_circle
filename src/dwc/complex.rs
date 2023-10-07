use crate::dwc::common::*;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct Complex
{
    pub real: f32, // real part
    pub img: f32, // imaginary part
}

impl Complex
{
    pub fn new(real: f32, img: f32) -> Complex
    {
        return Complex { real: real, img: img };
    }
}

impl Complex
{
    pub fn debug(&self)
    {
        if self.img >= 0.0
        {
            println!("{} + {}i", self.real, self.img);
        }
        else 
        {
            println!("{} - {}i", self.real, self.img.abs());
        }
    }

    pub fn magnitude(&self) -> f32
    {
        return (self.real * self.real + self.img * self.img).sqrt();
    }
}

// add
impl std::ops::Add<&Complex> for &Complex
{
    type Output = Complex;

    fn add(self, rhs: &Complex) -> Complex
    {
        return Complex{ real: self.real + rhs.real, img: self.img + rhs.img };
    }
}

impl std::ops::Add<Complex> for &Complex
{
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex
    {
        return Complex{ real: self.real + rhs.real, img: self.img + rhs.img };
    }
}

impl std::ops::Add<&Complex> for Complex
{
    type Output = Complex;

    fn add(self, rhs: &Complex) -> Complex
    {
        return Complex{ real: self.real + rhs.real, img: self.img + rhs.img };
    }
}

impl std::ops::Add<Complex> for Complex
{
    type Output = Complex;

    fn add(self, rhs: Complex) -> Complex
    {
        return Complex{ real: self.real + rhs.real, img: self.img + rhs.img };
    }
}

// sub
impl std::ops::Sub<&Complex> for &Complex
{
    type Output = Complex;

    fn sub(self, rhs: &Complex) -> Complex
    {
        return Complex{ real: self.real - rhs.real, img: self.img - rhs.img };
    }
}

impl std::ops::Sub<Complex> for &Complex
{
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex
    {
        return Complex{ real: self.real - rhs.real, img: self.img - rhs.img };
    }
}

impl std::ops::Sub<&Complex> for Complex
{
    type Output = Complex;

    fn sub(self, rhs: &Complex) -> Complex
    {
        return Complex{ real: self.real - rhs.real, img: self.img - rhs.img };
    }
}

impl std::ops::Sub<Complex> for Complex
{
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Complex
    {
        return Complex{ real: self.real - rhs.real, img: self.img - rhs.img };
    }
}

// add assign
impl std::ops::AddAssign<&Complex> for Complex
{
    fn add_assign(&mut self, rhs: &Complex)
    {
        self.real += rhs.real;
        self.img += rhs.img;
    }
}

impl std::ops::AddAssign<Complex> for Complex
{
    fn add_assign(&mut self, rhs: Complex)
    {
        self.real += rhs.real;
        self.img += rhs.img;
    }
}

// sub assign
impl std::ops::SubAssign<&Complex> for Complex
{
    fn sub_assign(&mut self, rhs: &Complex)
    {
        self.real -= rhs.real;
        self.img -= rhs.img;
    }
}

impl std::ops::SubAssign<Complex> for Complex
{
    fn sub_assign(&mut self, rhs: Complex)
    {
        self.real -= rhs.real;
        self.img -= rhs.img;
    }
}

// mul with float
impl std::ops::Mul<f32> for &Complex
{
    type Output = Complex;

    fn mul(self, rhs: f32) -> Complex
    {
        return Complex{ real: self.real * rhs, img: self.img * rhs };
    }
}

impl std::ops::Mul<f32> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: f32) -> Complex
    {
        return Complex{ real: self.real * rhs, img: self.img * rhs };
    }
}

impl std::ops::Mul<&Complex> for f32
{
    type Output = Complex;

    fn mul(self, rhs: &Complex) -> Complex
    {
        return Complex{ real: self * rhs.real, img: self * rhs.img };
    }
}

impl std::ops::Mul<Complex> for f32
{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex
    {
        return Complex{ real: self * rhs.real, img: self * rhs.img };
    }
}

// mul
impl std::ops::Mul<&Complex> for &Complex
{
    type Output = Complex;

    fn mul(self, rhs: &Complex) -> Complex
    {
        return Complex
        { 
            real: self.real * rhs.real - self.img * rhs.img, 
            img: self.real * rhs.img + self.img * rhs.real
        };
    }
}

impl std::ops::Mul<&Complex> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: &Complex) -> Complex
    {
        return Complex
        { 
            real: self.real * rhs.real - self.img * rhs.img, 
            img: self.real * rhs.img + self.img * rhs.real
        };
    }
}

impl std::ops::Mul<Complex> for &Complex
{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex
    {
        return Complex
        { 
            real: self.real * rhs.real - self.img * rhs.img, 
            img: self.real * rhs.img + self.img * rhs.real
        };
    }
}

impl std::ops::Mul<Complex> for Complex
{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Complex
    {
        return Complex
        { 
            real: self.real * rhs.real - self.img * rhs.img, 
            img: self.real * rhs.img + self.img * rhs.real
        };
    }
}

// mull assign with float
impl std::ops::MulAssign<f32> for Complex
{
    fn mul_assign(&mut self, rhs: f32)
    {
        self.real *= rhs; 
        self.img *= rhs;
    }
}

// euler formula the
pub fn euler_formula(exponent: f32) -> Complex
{
    return Complex { real: exponent.cos(), img: exponent.sin() };
}

pub fn complex_eq(a: Complex, b: Complex) -> bool
{
    return float_eq(a.real, b.real) && float_eq(a.img, b.img);
}

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn addition() {
        let a: Complex = Complex::new(5.0, 4.0);
        let b: Complex = Complex::new(3.0, 1.0);

        let c: Complex = a + b;

        assert!(complex_eq(c, Complex::new(8.0, 5.0)));
    }

    #[test]
    fn subtraction() {
        let a: Complex = Complex::new(5.0, 4.0);
        let b: Complex = Complex::new(3.0, 1.0);

        let c: Complex = a - b;

        assert!(complex_eq(c, Complex::new(2.0, 3.0)));
    }

    #[test]
    fn multiplication() {
        let a: Complex = Complex::new(4.0, 2.0);
        let b: Complex = Complex::new(0.0, 1.0);

        let c: Complex = a * b;

        assert!(complex_eq(c, Complex::new(-2.0, 4.0)));
    }
}