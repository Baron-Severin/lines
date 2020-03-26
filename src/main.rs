extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::{Rect, Point};
use sdl2::image::{self, LoadTexture, InitFlag};

mod basic_unit;
mod viewable;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    // canvas.set_draw_color(Color::RGB(0, 255, 255));
    // canvas.clear();
    // canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut i = 0;
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        // canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...


        canvas.set_draw_color(Color::RGB(245, 245, 220));

        let _image_context = image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

        let texture_creator = canvas.texture_creator();

        let basic_unit = basic_unit::get(&texture_creator);

        canvas.copy(&basic_unit.texture, None, basic_unit.position).unwrap();

        canvas.present();
    }
}
