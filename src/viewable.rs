
use sdl2::render::Texture;
use sdl2::rect::Rect;


pub struct Viewable<'a> {
    pub texture: Texture<'a>,
    pub position: Rect
}
