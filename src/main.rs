use core::time;
use std::thread;

use fractal::compute_iterations;
use minifb::{Window, WindowOptions, Key};
use Vec;
mod fractal;
use std::f64::consts::E;

const WIDTH: usize = 851;
const HEIGHT: usize = 851;
const MOVE_SPEED: f64 = 20.;
const ZOOM_SPEED: f64 = 5.;
const COLOUR_RANGE: i32 = 3;
const MAX_ITERATIONS: usize = 256;


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
    pub fn new(pixel: usize, zoom: f64, x_offset:f64, y_offset: f64) -> Vec2{
        Self{
            original_x: ((pixel % WIDTH) as f64 - (WIDTH/2) as f64)/ zoom + x_offset,
            original_y: ((pixel / WIDTH) as f64 - (WIDTH/2) as f64)/ zoom + y_offset,
            x: ((pixel % WIDTH) as f64 - (WIDTH/2) as f64)/ zoom + x_offset,
            y: ((pixel / WIDTH) as f64 - (WIDTH/2) as f64)/ zoom + y_offset,
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
            self.value = compute_iterations(
                &mut Vec2::new_point(0., 0.),
                self,
                MAX_ITERATIONS)
                as f64 * (255_f64).powi(COLOUR_RANGE) / MAX_ITERATIONS as f64;
            //self.value = compute_iterations(self, &mut Vec2::new_point(0.8 * time.sin().powf(2.), time.sin()), MAX_ITERATIONS) as f64 * (255_f64).powi(COLOUR_RANGE) / MAX_ITERATIONS as f64;
        }
        
    }
}

fn main() {
    let mut value_buffer: Vec<Vec2> = Vec::new();
    let mut x_offset = 0.;
    let mut y_offset = 0.;
    let mut zoom = 200.;


    fn set_buffer(value_buffer: &mut Vec<Vec2>, zoom:f64, x_offset: f64, y_offset: f64){
        for i in 0..(WIDTH*HEIGHT){
            let mut this_point = Vec2::new(i, zoom, x_offset, y_offset);
            this_point.calculate(0 as f64);
            value_buffer.push(this_point);
        }
    }

    set_buffer(&mut value_buffer, 200., 0., 0.);

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
        let mut update = false;

        if my_window.is_key_down(Key::Space){
            t += 1;
            update = true;
            println!("Cr: {} Ci: {}", 0.8 * (t as f64).sin().powf(2.), (t as f64).sin())
        }
        if my_window.is_key_down(Key::W){
            y_offset -= MOVE_SPEED / zoom;
            update = true;
        }
        if my_window.is_key_down(Key::S){
            y_offset += MOVE_SPEED / zoom;
            update = true;
        }
        if my_window.is_key_down(Key::A){
            x_offset -= MOVE_SPEED / zoom;
            update = true;
        }
        if my_window.is_key_down(Key::D){
            x_offset += MOVE_SPEED / zoom;
            update = true;
        }
        if my_window.is_key_down(Key::Q){
            zoom *= ZOOM_SPEED;
            update = true;
        }
        if my_window.is_key_down(Key::E){
            zoom /= ZOOM_SPEED;
            update = true;
        }    

        if update{
            for i in 0..(WIDTH*HEIGHT){
                let mut this_point = Vec2::new(i, zoom, x_offset, y_offset);
                this_point.calculate(t as f64 / 30.);
                value_buffer[i] = this_point;
            }

            my_window.update_with_buffer(&value_buffer.iter().map(|value| value.value as u32).collect::<Vec<u32>>(), WIDTH, HEIGHT).unwrap();
        }
        else{
            my_window.update_with_buffer(&value_buffer.iter().map(|value| value.value as u32).collect::<Vec<u32>>(), WIDTH, HEIGHT).unwrap();
            thread::sleep(time::Duration::from_micros(16600));
        }        
    }
}
