extern crate sdl2;

use crate::dwc::complex::*;
use crate::dwc::epicycle::*;
use crate::dwc::common::*;

use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::Canvas;

pub struct Trail
{
    max_length: usize,
    values: Vec<Complex>
}

pub struct Renderer
{
    canvas: Canvas<Window>,
    half_width: f32,
    half_height: f32,
    pub scale: f32, // specify how big the renders are (scale < 1 results in smaller renders)
    pub pixel_size: f32,
}

// trail methods
impl Trail
{
    pub fn new(max_length: usize) -> Trail
    {
        return Trail{ max_length: max_length, values: Vec::new() };
    }
}

impl Trail
{
    pub fn push(&mut self, value: Complex)
    {
        if self.values.len() == self.max_length
        {
            self.values.remove(0);
        }

        self.values.push(value);
    }

    pub fn get_value(&self, idx: usize) -> &Complex
    {
        return &self.values[idx];
    }

    pub fn get_length(&self) -> usize
    {
        return self.values.len();
    }

    pub fn get_max_length(&self) -> usize
    {
        return self.max_length;
    }
}

// renderer methods
impl Renderer
{
    pub fn new(window: Window, scale: f32, pixel_size: f32) -> Renderer
    {
        let size: (u32, u32) = window.size();
        let width: f32 = size.0 as f32 * 0.5;
        let height: f32 = size.1 as f32 * 0.5;

        let mut canvas: Canvas<Window> = window.into_canvas().build().unwrap();
        let _ = canvas.set_scale(pixel_size, pixel_size);
        canvas.set_blend_mode(sdl2::render::BlendMode::Add);
        canvas.present();

        return Renderer { canvas: canvas, 
                          half_width: width, 
                          half_height: height, 
                          scale: scale, 
                          pixel_size: pixel_size };
    }

    pub fn clear(&mut self, color: Color)
    {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn to_screen_point(&self, p: &Complex) -> Point
    {
        let x: f32 = p.real * self.scale;
        let y: f32 = p.img * self.scale;

        return Point::new((x + self.half_width / self.pixel_size) as i32,
                         (-y + self.half_height / self.pixel_size) as i32);
    }

    pub fn draw_circle(&mut self, position: &Complex, radius: f32, color: Color)
    {
        let mut points: Vec<Point> = Vec::new();
        let dtheta: f32 = 1.0 / CIRCLE_EDGE_COUNT as f32;

        for i in 0..CIRCLE_EDGE_COUNT + 1
        {
            let mut point: Complex = euler_formula(dtheta * PI_2 * i as f32) * radius;
            point += position;

            points.push(self.to_screen_point(&point));
        }

        let sliced_points: &[Point] = &points[0..points.len()];

        self.canvas.set_draw_color(color);
        let _ = self.canvas.draw_lines(sliced_points);
    }
    
    pub fn draw_trail(&mut self, trail: &Trail, color: Color)
    {
        let mut points: Vec<Point> = Vec::new();

        for i in 0..trail.get_length()
        {
            points.push(self.to_screen_point(trail.get_value(i)));
        }

        let sliced_points: &[Point] = &points[0..points.len()];

        self.canvas.set_draw_color(color);
        let _ = self.canvas.draw_lines(sliced_points);
    }

    pub fn draw_epicycles(&mut self, epicycles: &Vec<Epicycle>, t: f32, color: Color)
    {
        let mut tip: Complex = Complex::new(0.0, 0.0);
        let mut points: Vec<Point> = Vec::new();
        points.push(self.to_screen_point(&Complex::new(0.0, 0.0)));

        for i in 0..epicycles.len()
        {
            self.draw_circle(&tip, epicycles[i].c0.magnitude(), Color::RGBA(color.r, color.g, color.b, 70));

            tip += epicycles[i].get_position(t);
            points.push(self.to_screen_point(&tip));
        }

        let sliced_points: &[Point] = &points[0..points.len()];

        self.canvas.set_draw_color(color);
        let _ = self.canvas.draw_lines(sliced_points);
    }

    pub fn display(&mut self)
    {
        self.canvas.present();
    }

    pub fn set_pixel_size(&mut self, pixel_size: f32)
    {
        self.pixel_size = pixel_size;
        let _ = self.canvas.set_scale(pixel_size, pixel_size);
    }
}