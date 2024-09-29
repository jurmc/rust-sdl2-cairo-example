extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::image::LoadTexture;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("dots.bmp").unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.copy(&texture,None,None).unwrap();

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        //canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        //canvas.clear();
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

        //canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

//fn main() {
//    let sdl_context = sdl2::init().unwrap();
//    let video = sdl_context.video().unwrap();
//    let window = video.window("SDL Tutorial 02", 640,480)
//        .position_centered().opengl().build().unwrap();
//    let mut canvas = window.into_canvas().build().unwrap();
//
//    let texture_creator = canvas.texture_creator();
//    let texture = texture_creator.load_texture("assets/foo.bmp").unwrap();
//    canvas.copy(&texture,None,None).unwrap();
//    canvas.present();
//    sleep(Duration::from_secs(2));
//}
