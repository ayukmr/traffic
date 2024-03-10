use crate::dir_bounds::DirBounds;
use crate::direction::Axis;

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

    pub fn bounds(&self) -> DirBounds {
        DirBounds::new(&self.pos)
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
