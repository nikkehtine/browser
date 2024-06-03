use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::video::Window;

fn main() {
    let sdl_ctx = sdl2::init().unwrap();
    let video_subsystem = sdl_ctx.video().unwrap();

    let window = video_subsystem
        .window("Browser", 1280, 720)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(24, 24, 24));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_ctx.event_pump().unwrap();

    'running: loop {
        for e in event_pump.poll_iter() {
            match e {
                Event::Quit { .. } => break 'running,
                Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(..),
                    ..
                } => {
                    canvas.clear();
                    canvas.present();
                }
                _ => {}
            }
        }
        canvas.present();
    }
}
