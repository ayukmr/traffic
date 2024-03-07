use crate::bounds::Bounds;
use crate::direction::Direction;
use crate::vehicle::BRAKE;

use parry2d::utils;
use parry2d::shape::Shape;
use parry2d::math::Isometry;
use parry2d::na::{Point2, Vector2};

use std::iter;

const CAR_WIDTH: f32 = 5.0;

#[derive(Debug)]
pub struct RectBounds(
    Point2<f32>,
    Point2<f32>,
    Point2<f32>,
    Point2<f32>,
);

impl RectBounds {
    pub fn vehicle(pos: &Point2<f32>, length: f32, dir: Direction) -> Self {
        let vec  = dir.vector();
        let perp = Vector2::new(-vec.y, vec.x);

        let front = pos + (vec * (length / 2.0));
        let back  = pos - (vec * (length / 2.0));

        let side = perp * (CAR_WIDTH / 2.0);

        Self(front - side, front + side, back - side, back + side)
    }

    pub fn collider(pos: &Point2<f32>, length: f32, speed: f32, dir: Direction) -> Self {
        let vec  = dir.vector();
        let perp = Vector2::new(-vec.y, vec.x);

        let projected: f32 =
            if matches!(dir, Direction::Turn(_, _)) {
                length
            } else {
                iter::successors(
                    Some(speed),
                    |prev| Some(prev + (BRAKE * 60.0)),
                )
                .take_while(|&speed| speed >= 0.0)
                .sum()
            };

        let size  = (length * 1.5) + projected;
        let front = pos + (vec * size);
        let side  = perp * (CAR_WIDTH / 2.0);

        Self(front - side, front + side, pos - side, pos + side)
    }
}

impl Bounds for RectBounds {
    fn as_parry(&self) -> (Isometry<f32>, Box<dyn Shape>) {
        let (iso, cuboid) =
            utils::obb(&[self.0, self.1, self.2, self.3]);

        (iso, Box::new(cuboid))
    }
}
