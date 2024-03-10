use crate::bounds::Bounds;
use crate::rect_bounds::RectBounds;
use crate::direction::{TileDirection, Direction};
use crate::stop_sign::StopSign;
use crate::stoplight::Stoplight;
use crate::tile::Tile;
use crate::tile_map::{TileMap, TILE_SIZE_F};

use parry2d::na::Point2;
use anyhow::{Result, Context};

pub const ACCEL: f32 = 0.25;
pub const BRAKE: f32 = -0.5;
pub const MAX_SPEED: f32 = 20.0;

#[derive(Clone, Copy)]
pub struct Vehicle {
    pub pos:      Point2<f32>,
    pub tile_pos: Point2<i32>,

    length: f32,
    speed:  f32,

    pub dir: Direction,
}

impl Vehicle {
    pub fn new(pos: Point2<f32>, length: f32, speed: f32, tiles: &TileMap<Tile>) -> Self {
        let tile = tiles.at_pos(&pos).expect("vehicle not on tile");

        if let TileDirection::Constant(dir) = tile.dir {
            Self {
                pos,
                length,
                speed,
                dir,
                tile_pos: tile.pos,
            }
        } else {
            panic!("vehicle started on intersection");
        }
    }

    pub fn bounds(&self) -> RectBounds {
        RectBounds::vehicle(&self.pos, self.length, self.dir)
    }

    pub fn update(
        &mut self,
        vehicles:   &[&Vehicle],
        tiles:      &TileMap<Tile>,
        stop_signs: &[StopSign],
        stoplights: &[Stoplight],
    ) -> Result<()> {
        let cur_tile = tiles.at_pos(&self.pos).context("")?;

        let tile_dir =
            if cur_tile.pos != self.tile_pos {
                cur_tile.dir.as_dir(self.dir)
            } else {
                self.dir
            };

        if cur_tile.pos != self.tile_pos {
            let tile_pos = Point2::new(cur_tile.pos.x as f32, cur_tile.pos.y as f32);

            self.pos = (
                tile_pos + (-tile_dir.in_dir().vector() / 2.0)
            ) * TILE_SIZE_F;

            self.tile_pos = cur_tile.pos;
            self.dir = tile_dir;
        }

        self.speed +=
            if self.should_slow(vehicles, stop_signs, stoplights) {
                BRAKE
            } else {
                ACCEL
            };
        self.speed = self.speed.clamp(0.0, MAX_SPEED);

        self.pos += (self.speed / 60.0) * tile_dir.vector();

        Ok(())
    }

    pub fn should_slow(
        &self,
        vehicles:   &[&Vehicle],
        stop_signs: &[StopSign],
        stoplights: &[Stoplight],
    ) -> bool {
        let collider =
            RectBounds::collider(
                &self.pos,
                self.length,
                self.speed,
                self.dir,
            );

        let collisions =
            vehicles.iter().any(|vehicle| {
                let bounds = vehicle.bounds();
                collider.colliding(&bounds)
            });

        let stop_signs =
            stop_signs.iter().any(|stop_sign| {
                stop_sign.colliding(self.tile_pos, &collider)
            });

        let stoplights =
            stoplights.iter().any(|stoplight| {
                let on_axis =
                    self.dir.out_dir().on_axis(&stoplight.axis);

                let bounds = stoplight.bounds();

                if stoplight.grace_period() {
                    bounds.colliding(&collider)
                } else {
                    let axis_bounds = bounds.opp_axis(stoplight.axis);

                    !on_axis && (
                        collider.colliding(axis_bounds.0) ||
                        collider.colliding(axis_bounds.1)
                    )
                }
            });

        collisions || stop_signs || stoplights
    }
}
