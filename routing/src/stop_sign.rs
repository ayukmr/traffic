use crate::bounds::Bounds;
use crate::vehicle::Vehicle;
use crate::direction::Cardinal;
use crate::dir_bounds::DirBounds;

use parry2d::na::{Point2, Vector2};

pub struct StopSign {
    pub pos: Point2<f32>,
    pub dir: Cardinal,
    moved_inside: bool,
}

impl StopSign {
    pub fn new(pos: Point2<f32>) -> Self {
        Self {
            pos,
            dir: Cardinal::Up,
            moved_inside: false,
        }
    }

    pub fn colliding(&self, pos: Point2<i32>, collider: &dyn Bounds) -> bool {
        let bounds = DirBounds::new(&self.pos);

        if self.pos_inside(pos) {
            false
        } else if self.moved_inside {
            bounds.colliding(collider)
        } else {
            let side_bounds = match self.dir {
                Cardinal::Up    => (bounds.up,   bounds.left, bounds.right),
                Cardinal::Down  => (bounds.down, bounds.left, bounds.right),
                Cardinal::Left  => (bounds.up,   bounds.down, bounds.left),
                Cardinal::Right => (bounds.up,   bounds.down, bounds.right),
            };

            collider.colliding(&side_bounds.0) ||
            collider.colliding(&side_bounds.1) ||
            collider.colliding(&side_bounds.2)
        }
    }

    pub fn update(&mut self, vehicles: &[Vehicle]) {
        let any_inside = self.any_inside(vehicles);

        if any_inside {
            self.moved_inside = true;
        }

        let rotated = self.dir.rotate();

        let should_rotate =
            !any_inside &&
            (self.moved_inside || !self.vehicle_outside(vehicles, self.dir)) &&
            self.vehicle_outside(vehicles, rotated);

        if should_rotate {
            self.dir = rotated;
            self.moved_inside = false;
        }
    }

    pub fn any_inside(&self, vehicles: &[Vehicle]) -> bool {
        let f_pos = self.pos + Vector2::new(-0.5, -0.5);
        let pos   = Point2::new(f_pos.x as i32, f_pos.y as i32);

        [
            Vector2::new(0, 0),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(1, 1),
        ].iter().any(|offset| {
            vehicles.iter().any(|vehicle| {
                vehicle.tile_pos == pos + offset
            })
        })
    }

    pub fn pos_inside(&self, pos: Point2<i32>) -> bool {
        let f_pos = self.pos + Vector2::new(-0.5, -0.5);
        let stoplight_pos = Point2::new(f_pos.x as i32, f_pos.y as i32);

        [
            Vector2::new(0, 0),
            Vector2::new(1, 0),
            Vector2::new(0, 1),
            Vector2::new(1, 1),
        ].iter().any(|offset| {
            pos == stoplight_pos + offset
        })
    }

    pub fn vehicle_outside(&self, vehicles: &[Vehicle], dir: Cardinal) -> bool {
        let offsets =
            match dir {
                Cardinal::Up    => [Vector2::new( 0.5,  1.5), Vector2::new( 0.5,  2.5)],
                Cardinal::Down  => [Vector2::new(-0.5, -1.5), Vector2::new(-0.5, -2.5)],
                Cardinal::Left  => [Vector2::new( 1.5, -0.5), Vector2::new( 2.5, -0.5)],
                Cardinal::Right => [Vector2::new(-1.5,  0.5), Vector2::new(-2.5,  0.5)],
            };

        offsets.iter().any(|offset| {
            let f_pos = self.pos + offset;
            let pos   = Point2::new(f_pos.x as i32, f_pos.y as i32);

            vehicles.iter().any(|vehicle| vehicle.tile_pos == pos)
        })
    }
}
