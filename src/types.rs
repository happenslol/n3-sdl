use sdl2::rect::Rect as SdlRect;

use std::ops::{Add, Sub, Mul, Div};

pub type CgPoint = super::cgmath::Point2<f64>;
pub type CgVec2 = super::cgmath::Vector2<f64>;

#[derive(PartialEq, Eq, Debug, Hash, Copy, Clone)]
pub enum KeyAction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct TilePos {
    pub x: u32,
    pub y: u32,
}

impl TilePos {
    pub fn new(x: u32, y: u32) -> TilePos {
        TilePos { x: x, y: y }
    }

    pub fn from_point(p: Point) -> TilePos {
        TilePos::new(p.x as u32, p.y as u32)
    }
}

impl Sub<TilePos> for TilePos {
    type Output = TilePos;

    fn sub(self, rhs: TilePos) -> TilePos {
        TilePos::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }

    pub fn to_sdl_rect(&self, s: Size) -> SdlRect {
        SdlRect::new(self.x as i32,
                     self.y as i32,
                     s.w as u32,
                     s.h as u32)
    }

    pub fn is_diag(&self) -> bool {
        self.x != 0.0 && self.y != 0.0
    }

    pub fn mult_diag(&mut self) {
        let mult = (0.5f64).sqrt();
        self.x *= mult;
        self.y *= mult;
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Point {
        Point::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Point {
        Point::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<Size> for Point {
    type Output = Point;

    fn div(self, rhs: Size) -> Point {
        Point::new(self.x / rhs.w, self.y / rhs.h)
    }
}

impl Size {
    pub fn new(w: f64, h: f64) -> Size {
        Size { w: w, h: h }
    }

    pub fn to_point(&self) -> CgPoint {
        CgPoint::new(self.w, self.h)
    }
}
