use crate::bounds::Bounds;
use crate::direction::Axis;
use crate::tile_map::TILE_SIZE_F;

use parry2d::math::Isometry;
use parry2d::shape::{Segment, Shape};
use parry2d::na::{Point2, Vector2};

#[derive(Debug)]
pub struct SegmentBounds(
    pub Point2<f32>,
    pub Point2<f32>,
);

impl SegmentBounds {
    pub fn stoplight(pos: &Point2<f32>, axis: Axis) -> (Self, Self) {
        let main =
            match axis {
                Axis::Horizontal => Vector2::new( 0.0, 1.0),
                Axis::Vertical   => Vector2::new(-1.0, 0.0),
            };

        let side =
            match axis {
                Axis::Horizontal => Vector2::new(1.0, 0.0),
                Axis::Vertical   => Vector2::new(0.0, 1.0),
            };

        (
            Self((pos - main - side) * TILE_SIZE_F, (pos - main) * TILE_SIZE_F),
            Self((pos + main + side) * TILE_SIZE_F, (pos + main) * TILE_SIZE_F),
        )
    }
}

impl Bounds for SegmentBounds {
    fn as_parry(&self) -> (Isometry<f32>, Box<dyn Shape>) {
        let iso = Isometry::default();
        let seg = Segment::new(self.0, self.1);

        (iso, Box::new(seg))
    }
}
