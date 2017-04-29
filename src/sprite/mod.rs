mod sprite_cache;
pub type SpriteCache = sprite_cache::SpriteCache;

mod sprite_manager;
pub type SpriteManager = sprite_manager::SpriteManager;

use std::cell::RefCell;
use std::rc::Rc;

use sdl2::render::{Texture, Renderer};

use types::{Point, Size};
use types::to_sdl_rect;
use camera::Camera;

#[derive(Serialize, Deserialize, Debug)]
pub struct SpriteData {
    name: String
}

#[derive(Clone)]
pub struct Sprite {
    pub name: String,
    size: Size,
    src: Point,
    src_size: Size,
    tex: Rc<RefCell<Texture>>
}

impl Sprite {
    pub fn new(name: &str, size: Size, src: Point, src_size: Size, tex: Rc<RefCell<Texture>>) -> Sprite {
        Sprite {
            name: String::from(name),
            size: size,
            src: src,
            src_size: src_size,
            tex: tex
        }
    }

    pub fn draw(&self, pos: Point, r: &mut Renderer, c: &Camera) {
        let dest = pos + (c.as_vec() * -1.0);
        let _ = r.copy(&mut self.tex.borrow_mut(),
                       Some(to_sdl_rect(self.src, self.src_size)),
                       Some(to_sdl_rect(dest, self.size)));
    }
}
