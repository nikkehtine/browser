use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Renderer<'renderer_life> {
    pub context: &'renderer_life sdl2::Sdl,
    pub canvas: &'renderer_life mut sdl2::render::Canvas<Window>,
}

impl Renderer<'_> {
    pub fn render(&mut self) {
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(23, 23, 24));
        self.canvas.clear();
        self.canvas.present();
    }
}

pub fn new_renderer<'render_context>(
    context: &'render_context sdl2::Sdl,
    canvas: &'render_context mut Canvas<Window>,
) -> Renderer<'render_context> {
    Renderer {
        context: context,
        canvas: canvas,
    }
}
