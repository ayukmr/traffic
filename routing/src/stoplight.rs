use crate::direction::Axis;
use crate::segment_bounds::SegmentBounds;

use parry2d::na::Point2;

pub const GRACE_TIME: f32 = 2.5;

pub struct Stoplight {
    pub pos: Point2<f32>,
    freq: f32,
    pub axis: Axis,
    pub grace: Option<u32>,
}

impl Stoplight {
    pub fn new(pos: Point2<f32>, freq: f32) -> Self {
        Self {
            pos,
            freq,
            axis:  Axis::Vertical,
            grace: None,
        }
    }

    pub fn grace_period(&self) -> bool {
        self.grace.is_some()
    }

    pub fn bounds(&self) -> (SegmentBounds, SegmentBounds) {
        SegmentBounds::stoplight(&self.pos, self.axis)
    }

    pub fn all_bounds(&self) -> (
        SegmentBounds,
        SegmentBounds,
        SegmentBounds,
        SegmentBounds,
    ) {
        let horizontal = SegmentBounds::stoplight(&self.pos, Axis::Horizontal);
        let vertical   = SegmentBounds::stoplight(&self.pos, Axis::Vertical);

        (horizontal.0, horizontal.1, vertical.0, vertical.1)
    }

    pub fn update(&mut self, time: u32) {
        if let Some(period) = self.grace {
            if period >= (GRACE_TIME * 60.0) as u32 {
                self.grace = None;
                self.axis  = self.axis.flip();
            } else {
                *self.grace.as_mut().unwrap() += 1;
            }
        } else if time as f32 % (self.freq * 60.0) == 0.0 {
            self.grace = Some(0);
        }
    }
}
