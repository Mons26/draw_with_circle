use crate::dwc::common::*;
use crate::dwc::complex::*;

#[derive(Clone)]
pub struct Epicycle
{
    pub c0: Complex, // a complex value to store the radius and the initial phase
    pub f: f32 // frequency
}

impl Epicycle
{
    pub fn new(c0: Complex, f: f32) -> Epicycle
    {
        return Epicycle{ c0: c0, f: f };
    }

    // get the value of multiple epicycles combined
    pub fn get_combined_position(epicycles: &Vec<Epicycle>, t: f32) -> Complex
    {
        let mut vec: Complex = Complex::new(0.0, 0.0);

        for i in 0..epicycles.len()
        {
            vec += epicycles[i].get_position(t);
        }

        return vec;
    }
}

impl Epicycle
{
    // position on the epicycle at a certain time
    pub fn get_position(&self, t: f32) -> Complex
    {
        // multiplying by e^i*f*2pi*t makes everything rotates with f cycles per sec
        return &self.c0 * euler_formula(self.f * PI_2 * t);
    }

    pub fn debug(&self)
    {
        print!("constant: ");
        self.c0.debug();
        println!("frequency: {}", self.f);
    }
}

// all of these has been explained very well in this video
// https://www.youtube.c0om/watch?v=r6sGWTCMz2k

fn compute_cn(shape: &Vec<Complex>, f: f32, dt: f32) -> Complex
{
    let mut t: f32 = 0.0;
    let mut cn: Complex = Complex::new(0.0, 0.0);
    let size: f32 = shape.len() as f32;

    for point in shape
    {
        cn += point * euler_formula(f * PI_2 * t);
        t += dt;
    }

    cn *= 1.0 / size;

    return cn;
}

pub fn compute_epicycles(shape: &Vec<Complex>, n: u8) -> Vec<Epicycle>
{
    let mut epicycles: Vec<Epicycle> = Vec::new();
    let dt = 1.0 / shape.len() as f32;

    // freqencies from -n to n (0 excluded)
    // calculate the radius and phase of each frequency
    for i in 1..n
    {
        let f: f32 = i as f32;

        let epicycle_p: Epicycle = Epicycle { c0: compute_cn(&shape, -f, dt), f: f };
        epicycles.push(epicycle_p);

        let epicycle_n: Epicycle = Epicycle { c0: compute_cn(&shape, f, dt), f: -f };
        epicycles.push(epicycle_n);
    }

    return epicycles;
}

#[cfg(test)]
mod test 
{
    use super::*;

    #[test]
    fn position() 
    {
        let e: Epicycle = Epicycle::new(Complex::new(1.0, 0.0), 1.0);
        let p0: Complex = e.get_position(0.25);
        let p1: Complex = e.get_position(0.5);
        let p2: Complex = e.get_position(0.75);
        let p3: Complex = e.get_position(1.0);

        assert!(complex_eq(p0, Complex::new(0.0, 1.0)));
        assert!(complex_eq(p1, Complex::new(-1.0, 0.0)));
        assert!(complex_eq(p2, Complex::new(0.0, -1.0)));
        assert!(complex_eq(p3, Complex::new(1.0, 0.0)));
    }

    #[test]
    fn combined_position()
    {
        let es: Vec<Epicycle> = Vec::from
        ([
            Epicycle::new(Complex::new(1.0, 0.0), 1.0),
            Epicycle::new(Complex::new(0.0, 1.0), 1.0),
        ]);

        let p0 = Epicycle::get_combined_position(&es, 0.0);
        let p1 = Epicycle::get_combined_position(&es, 0.125);
        let p2 = Epicycle::get_combined_position(&es, 0.25);
        let p3 = Epicycle::get_combined_position(&es, 0.375);

        let hyp: f32 = 2.0_f32.sqrt();

        assert!(complex_eq(p0, Complex::new(1.0, 1.0)));
        assert!(complex_eq(p1, Complex::new(0.0, hyp)));
        assert!(complex_eq(p2, Complex::new(-1.0, 1.0)));
        assert!(complex_eq(p3, Complex::new(-hyp, 0.0)));
    }
}