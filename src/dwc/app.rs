extern crate sdl2;

use crate::dwc::epicycle::*;
use crate::dwc::draw::*;
use crate::dwc::file_loader::*;

use sdl2::EventPump;
use sdl2::VideoSubsystem;
use sdl2::Sdl;
use sdl2::video::Window;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

#[allow(dead_code)]
pub struct App
{
    event: EventPump,
    context: Sdl,
    video_subsystem: VideoSubsystem,
    renderer: Renderer,

    t: f32,
    dt: f32,
    shape: Trail,
    epicycles: Vec<Epicycle>,
    begin_draw: bool,
    max_framerate: u8
}

impl App
{
    pub fn new(title: &str, window_width: u32, window_height: u32) -> App
    {
        let sdl_context: Sdl = sdl2::init().unwrap();
        let video_subsystem: VideoSubsystem = sdl_context.video().unwrap();

        let window: Window = video_subsystem.window(&title, window_width, window_height)
                            .position_centered()
                            .build()
                            .unwrap();

        let renderer: Renderer = Renderer::new(window, 0.0, 0.0);

        let event_pump = sdl_context.event_pump().unwrap();

        return App 
        {
            event: event_pump,
            context: sdl_context,
            video_subsystem: video_subsystem,
            renderer: renderer,
            max_framerate: 0,
            t: 0.0,
            dt: 0.0,
            shape: Trail::new(0),
            epicycles: Vec::new(),
            begin_draw: false
        };
    }
}

impl App {

    pub fn init(&mut self, file: String, delta_time: f32, trail_length: usize, max_framerate: u8)
    {
        self.dt = delta_time;
        self.shape = Trail::new(trail_length);
        self.max_framerate = max_framerate;

        let shape = load_from_txt(&file);

        self.epicycles = compute_epicycles(&shape, 150);
    }

    pub fn init_renderer(&mut self, draw_scale: f32, pixel_size: f32)
    {
        self.renderer.scale = draw_scale;
        self.renderer.set_pixel_size(pixel_size);
    }

    pub fn run(&mut self)
    {
        'running: loop {
            for event in self.event.poll_iter() {
                match event {
                    Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                        self.begin_draw = true;
                    },
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                    _ => {},
                }
            }
            
            self.update();

            self.render();
        }
    }
}

impl App
{
    fn render(&mut self)
    {
        self.renderer.clear(Color::RGB(5, 20, 60));
            
        self.renderer.draw_epicycles(&self.epicycles, self.t, Color::RGB(255, 255, 255));

        self.renderer.draw_trail(&self.shape, Color::RGB(255, 100, 0));

        self.renderer.display();
    }

    fn update(&mut self)
    {
        self.shape.push(Epicycle::get_combined_position(&self.epicycles, self.t));
                
        if self.begin_draw
        {
            self.t += self.dt;
        }

        ::std::thread::sleep(core::time::Duration::new(0, 1_000_000_000u32 / self.max_framerate as u32));
    }
}