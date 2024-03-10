use crate::bounds::Bounds;

use parry2d::math::Isometry;
use parry2d::shape::{Segment, Shape};
use parry2d::na::Point2;

#[derive(Debug)]
pub struct SegmentBounds(
    pub Point2<f32>,
    pub Point2<f32>,
);

impl Bounds for SegmentBounds {
    fn as_parry(&self) -> (Isometry<f32>, Box<dyn Shape>) {
        let iso = Isometry::default();
        let seg = Segment::new(self.0, self.1);

        (iso, Box::new(seg))
    }
}
