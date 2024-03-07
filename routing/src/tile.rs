use crate::direction::TileDirection;
use crate::tile_map::Locatable;

use parry2d::na::Point2;

pub struct Tile {
    pub pos: Point2<i32>,
    pub dir: TileDirection,
}

impl Tile {
    pub fn new(pos: Point2<i32>, dir: TileDirection) -> Self {
        Self { pos, dir }
    }
}

impl Locatable for Tile {
    fn pos(&self) -> &Point2<i32> {
        &self.pos
    }
}
