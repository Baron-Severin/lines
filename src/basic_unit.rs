
use sdl2::render::TextureCreator;
use sdl2::rect::Rect;
use sdl2::surface::SurfaceContext;
use sdl2::video::WindowContext;
use sdl2::image::LoadTexture;

use crate::viewable::Viewable;


pub fn get<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Viewable {
    let texture = texture_creator.load_texture("../assets/images/x.png").unwrap();

    return Viewable { texture: texture, position: Rect::new(200, 200, 10, 10) }
}


