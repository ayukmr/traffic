use crate::bounds::Bounds;
use crate::direction::Axis;
use crate::tile_map::TILE_SIZE_F;
use crate::segment_bounds::SegmentBounds;

use parry2d::na::{Point2, Vector2};

#[derive(Debug)]
pub struct DirBounds {
    pub up:    SegmentBounds,
    pub down:  SegmentBounds,
    pub left:  SegmentBounds,
    pub right: SegmentBounds,
}

impl DirBounds {
    pub fn new(pos: &Point2<f32>) -> Self {
        let main_h = Vector2::new(-1.0, 0.0);
        let main_v = Vector2::new( 0.0, 1.0);

        let side_h = Vector2::new(0.0, 1.0);
        let side_v = Vector2::new(1.0, 0.0);

        Self {
            up:    SegmentBounds((pos - main_v - side_v) * TILE_SIZE_F, (pos - main_v) * TILE_SIZE_F),
            down:  SegmentBounds((pos + main_v + side_v) * TILE_SIZE_F, (pos + main_v) * TILE_SIZE_F),
            left:  SegmentBounds((pos + main_h + side_h) * TILE_SIZE_F, (pos + main_h) * TILE_SIZE_F),
            right: SegmentBounds((pos - main_h - side_h) * TILE_SIZE_F, (pos - main_h) * TILE_SIZE_F),
        }
    }

    pub fn colliding(&self, collider: &dyn Bounds) -> bool {
        collider.colliding(&self.up) ||
            collider.colliding(&self.down) ||
            collider.colliding(&self.left) ||
            collider.colliding(&self.right)
    }

    pub fn opp_axis(&self, axis: Axis) -> (&SegmentBounds, &SegmentBounds) {
        match axis {
            Axis::Horizontal => (&self.up,   &self.down),
            Axis::Vertical   => (&self.left, &self.right),
        }
    }
}
