use std::f64::consts::*;

pub const EPSILON: f32 = 0.005;
pub const CIRCLE_EDGE_COUNT: usize = 32;
pub const PI_2: f32 = PI as f32 * 2.0;

pub fn float_eq(a: f32, b: f32) -> bool
{
    if a - b <= EPSILON
    {
        return  true;
    }
    else 
    {
        return false;
    }
}