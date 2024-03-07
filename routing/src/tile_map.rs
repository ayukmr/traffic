use parry2d::na::Point2;

pub const TILE_SIZE:   i32 = 15;
pub const TILE_SIZE_F: f32 = TILE_SIZE as f32;

pub trait Locatable {
    fn pos(&self) -> &Point2<i32>;
}

pub struct TileMap<T: Locatable> {
    pub tiles: Vec<T>,
}

impl<T: Locatable> TileMap<T> {
    pub fn new(tiles: Vec<T>) -> Self {
        Self { tiles }
    }

     pub fn at_pos(&self, pos: &Point2<f32>) -> Option<&T> {
        self.tiles.iter().find(|tile| {
            let x = tile.pos().x as f32 * TILE_SIZE_F;
            let y = tile.pos().y as f32 * TILE_SIZE_F;

            pos.x <= x + (TILE_SIZE_F / 2.0) &&
                pos.x >= x - (TILE_SIZE_F / 2.0) &&
                pos.y <= y + (TILE_SIZE_F / 2.0) &&
                pos.y >= y - (TILE_SIZE_F / 2.0)
        })
    }
}
