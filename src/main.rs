use core::time;
use std::thread;

use fractal::compute_iterations;
use minifb::{Window, WindowOptions, Key};
use Vec;
mod fractal;

const WIDTH: usize = 851;
const HEIGHT: usize = 851;

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

#[derive(Debug)]
pub struct Vec2 {
    original_x: f64,
    original_y: f64,
    x: f64,
    y: f64,
    value: f64,
}


impl Vec2{
    pub fn new(pixel: usize) -> Vec2{
        Self{
            original_x: ((pixel % WIDTH) as f64 - (WIDTH/2) as f64)/ 200.,
            original_y: ((pixel / WIDTH) as f64 - (WIDTH/2) as f64)/ 200.,
            x: ((pixel % WIDTH) as f64 - (WIDTH/2) as f64)/ 200.,
            y: ((pixel / WIDTH) as f64 - (WIDTH/2) as f64)/ 200.,
            value: 0.,
        }
    }

    pub fn reset(&mut self){
        self.x = self.original_x;
        self.y = self.original_y;
    }

    pub fn new_point(x: f64, y: f64) -> Vec2{
        Self { original_x: x, original_y: y, x: x, y: y, value: 0. }
    }

    pub fn calculate(&mut self, time: f64){
        if self.x == 0. || self.y == 0. {
            self.value = 16777215.;
        }
        else if fractal::mod2(self) > 0.99 && fractal::mod2(self) < 1.01 {
            self.value = 124.;
        }

        else{
            self.value = compute_iterations(&mut Vec2::new_point(0., 0.), self, 124) as f64 * 255. * 255. / 124.;
            //self.value = compute_iterations(self, &mut Vec2::new_point(0.8 * time.sin().powf(2.), time.sin()), 50) as f64 * 255. * 255. / 50.;
        }
        
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut value_buffer: Vec<Vec2> = Vec::new();
    for (i, _) in buffer.iter().enumerate(){
        let mut this_point = Vec2::new(i);
        this_point.calculate(0 as f64);
        value_buffer.push(this_point);
    }

    let mut my_window = Window::new(
        "Pixels",
        WIDTH,
        HEIGHT,
        WindowOptions::default()
    ).unwrap_or_else(|e| {
        panic!("{}", e);}
    );

    my_window.limit_update_rate(Some(std::time::Duration::from_micros(166000)));

    let mut t = 1;
    while my_window.is_open() && !my_window.is_key_down(Key::Escape){
        if t % 1 == 0{
            value_buffer.iter_mut().for_each(|point: &mut Vec2| {
                point.reset();
                point.calculate(t as f64 / 100.);
            });

            my_window.update_with_buffer(&value_buffer.iter().map(|value| value.value as u32).collect::<Vec<u32>>(), WIDTH, HEIGHT).unwrap();
        }

        if my_window.is_key_down(Key::Space){
            t += 1;
            println!("Cr: {} Ci: {}", 0.8 * (t as f64).sin().powf(2.), (t as f64).sin())
        }
        else{
            thread::sleep(time::Duration::from_micros(16600))
        }
        
        
    }




    //fractal::compute_iterations(&mut Vec2::new_point(0., 0.), &mut Vec2::new_point(-0.5, 0.5), 50);
}
