extern crate sdl2;
extern crate cairo;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;

use cairo::ImageSurface;
use cairo::Format;
use cairo::Context;
use cairo::Error;

use std::f64::consts::PI;
use std::time::Duration;
use std::mem;

pub fn main() -> Result<(), Error> {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    const WIDTH: u32 = 640;
    const HEIGHT: u32 = 480;
    const DEPTH: u32 = 4;
    const PITCH: u32 = WIDTH * DEPTH;

    let window = video_subsystem.window("rust-sdl2-cairo-example", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.clear();

    let mut pixels: [u8; (WIDTH * HEIGHT * DEPTH) as usize] = [0u8; (WIDTH * HEIGHT * DEPTH) as usize];
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::BGRA32, WIDTH, HEIGHT).unwrap();

    let cairo_surface: ImageSurface;
    unsafe {
        cairo_surface = ImageSurface::create_for_data_unsafe(pixels[..].as_mut_ptr(), Format::ARgb32, 640, 480, (640 * 4 * mem::size_of::<u8>()) as i32)
            .expect("Couldn't create Cairo surface (using pixels from SDL surface)");
        }
    let cairo_ctx = Context::new(&cairo_surface).unwrap();

    // White background
    cairo_ctx.set_source_rgba(1.0, 1.0, 1.0, 1.0);
    cairo_ctx.paint()?;

    // Arc
    let (xc, yc) = (WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0);
    let radius = 200.0;
    let angle1 = 45.0  * (PI/180.0);
    let angle2 = 180.0 * (PI/180.0);

    cairo_ctx.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cairo_ctx.set_line_width(10.0);
    cairo_ctx.arc(xc, yc, radius, angle1, angle2);
    cairo_ctx.stroke()?;

    // Dot
    cairo_ctx.set_source_rgba(1.0, 0.2, 0.2, 0.6);
    cairo_ctx.set_line_width(6.0);

    cairo_ctx.arc(xc, yc, 10.0, 0.0, 2.0 * PI);
    cairo_ctx.fill()?;

    // Lines
    cairo_ctx.arc(xc, yc, radius, angle1, angle1);
    cairo_ctx.line_to(xc, yc);
    cairo_ctx.arc(xc, yc, radius, angle2, angle2);
    cairo_ctx.line_to(xc, yc);
    cairo_ctx.stroke()?;

    cairo_surface.flush();

    texture.update(None, &pixels[..], PITCH as usize).unwrap();
    canvas.copy(&texture, None, None).unwrap();
    canvas.present();

    let mut event_pump = sdl_ctx.event_pump().unwrap();
    'running: loop {
        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
