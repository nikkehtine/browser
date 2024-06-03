use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Renderer<'a> {
    pub context: &'a sdl2::Sdl,
    pub canvas: &'a mut sdl2::render::Canvas<Window>,
}

impl Renderer<'_> {
    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(24, 24, 24));
        self.canvas.clear();
        self.canvas.present();
    }
}

pub fn new_renderer<'a>(context: &'a sdl2::Sdl, canvas: &'a mut Canvas<Window>) -> Renderer<'a> {
    Renderer {
        context: context,
        canvas: canvas,
    }
}
