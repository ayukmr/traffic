use routing::direction::{Cardinal, Direction, TileDirection};
use routing::stoplight::Stoplight;
use routing::tile::Tile;
use routing::tile_map::{TileMap, TILE_SIZE_F};
use routing::vehicle::Vehicle;

use renderer::renderer::Renderer;

use routing::direction::Cardinal::*;
use routing::direction::Direction::*;

use rand::seq::SliceRandom;
use rand::Rng;

use nalgebra::Point2;
use winit::event::{Event, WindowEvent};
use anyhow::Result;

use std::collections::HashMap;

macro_rules! intersection {
    ( $( $key:expr => $val:expr ),* $( , )? ) => {{
         let mut map = HashMap::new();

         $(
             map.insert($key, $val);
         )*

         intersection(map)
    }};
}

#[allow(dead_code)]
fn tile(point: (i32, i32), dir: TileDirection) -> Tile {
    Tile::new(Point2::new(point.0, point.1), dir)
}

#[allow(dead_code)]
fn straight(dir: Cardinal) -> TileDirection {
    TileDirection::Constant(Direction::Straight(dir))
}

#[allow(dead_code)]
fn turn(in_dir: Cardinal, out_dir: Cardinal) -> TileDirection {
    TileDirection::Constant(Direction::Turn(in_dir, out_dir))
}

#[allow(dead_code)]
fn intersection(dirs: HashMap<Direction, Vec<Cardinal>>) -> TileDirection {
    TileDirection::Intersection(dirs)
}

fn main() -> Result<()> {
    let tiles = TileMap::new(vec![
        tile((6, -3), straight(Down)),
        tile((6, -2), straight(Down)),
        tile((6, -1), straight(Down)),
        tile((6,  0), straight(Down)),
        tile((6,  3), straight(Down)),
        tile((6,  4), straight(Down)),
        tile((6,  5), straight(Down)),
        tile((6,  6), straight(Down)),
        tile((6,  7), straight(Down)),

        tile((7, -3), straight(Up)),
        tile((7, -2), straight(Up)),
        tile((7, -1), straight(Up)),
        tile((7,  0), straight(Up)),
        tile((7,  3), straight(Up)),
        tile((7,  4), straight(Up)),
        tile((7,  5), straight(Up)),
        tile((7,  6), straight(Up)),
        tile((7,  7), straight(Up)),

        tile((6, 1), intersection!(
            Straight(Left) => vec![Left],
            Straight(Down) => vec![Down, Left],
        )),
        tile((7, 1), intersection!(
            Straight(Left) => vec![Left, Up],
            Straight(Up)   => vec![Up],
        )),
        tile((6, 2), intersection!(
            Straight(Right) => vec![Right, Down],
            Straight(Down)  => vec![Down],
        )),
        tile((7, 2), intersection!(
            Straight(Right) => vec![Right],
            Straight(Up)    => vec![Up, Right],
        )),

        tile((-2, 1), straight(Left)),
        tile((-1, 1), straight(Left)),
        tile((0,  1), straight(Left)),
        tile((1,  1), straight(Left)),
        tile((2,  1), straight(Left)),
        tile((3,  1), straight(Left)),
        tile((4,  1), straight(Left)),
        tile((5,  1), straight(Left)),
        tile((8,  1), straight(Left)),
        tile((9,  1), straight(Left)),
        tile((10, 1), straight(Left)),
        tile((11, 1), straight(Left)),
        tile((12, 1), straight(Left)),
        tile((13, 1), straight(Left)),
        tile((14, 1), straight(Left)),

        tile((-2, 2), straight(Right)),
        tile((-1, 2), straight(Right)),
        tile((0,  2), straight(Right)),
        tile((1,  2), straight(Right)),
        tile((2,  2), straight(Right)),
        tile((3,  2), straight(Right)),
        tile((4,  2), straight(Right)),
        tile((5,  2), straight(Right)),
        tile((8,  2), straight(Right)),
        tile((9,  2), straight(Right)),
        tile((10, 2), straight(Right)),
        tile((11, 2), straight(Right)),
        tile((12, 2), straight(Right)),
        tile((13, 2), straight(Right)),
        tile((14, 2), straight(Right)),
    ]);

    let possible = [
        Vehicle::new(
            Point2::new(6.0 * TILE_SIZE_F, -3.0 * TILE_SIZE_F),
            8.0,
            rand::thread_rng().gen_range(2.0..10.0),
            &tiles,
        ),

        Vehicle::new(
            Point2::new(7.0 * TILE_SIZE_F, 7.0 * TILE_SIZE_F),
            8.0,
            rand::thread_rng().gen_range(2.0..10.0),
            &tiles,
        ),

        Vehicle::new(
            Point2::new(-2.0 * TILE_SIZE_F, 2.0 * TILE_SIZE_F),
            8.0,
            rand::thread_rng().gen_range(2.0..10.0),
            &tiles,
        ),

        Vehicle::new(
            Point2::new(14.0 * TILE_SIZE_F, 1.0 * TILE_SIZE_F),
            8.0,
            rand::thread_rng().gen_range(2.0..10.0),
            &tiles,
        ),
    ];

    let mut vehicles: Vec<Vehicle> =
        possible
            .choose_multiple(&mut rand::thread_rng(), 4)
            .copied()
            .collect();

    let mut stoplights = vec![
        Stoplight::new(Point2::new(6.5, 1.5), 10.0),
    ];

    let (mut renderer, event_loop) = Renderer::new()?;

    let mut time = 0;

    let _ = event_loop.run(move |event, elwt| {
        if let Event::WindowEvent { ref event, .. } = event {
            match event {
                WindowEvent::CloseRequested => {
                    elwt.exit();
                }

                WindowEvent::Resized(_) => {
                    renderer.update_surface().unwrap();
                }

                WindowEvent::RedrawRequested => {
                    time += 1;

                    for idx in 0..vehicles.len() {
                        let (left, right)    = vehicles.split_at_mut(idx);
                        let (vehicle, right) = right.split_at_mut(1);

                        let vehicle = &mut vehicle[0];
                        let vehicles =
                            left
                                .iter()
                                .chain(right.as_ref())
                                .collect::<Vec<_>>();

                        vehicle.update(&vehicles, &tiles, &stoplights).unwrap();
                    }

                    for stoplight in &mut stoplights {
                        stoplight.update(time);
                    }

                    vehicles.retain(|vehicle| {
                        match vehicle.dir.out_dir() {
                            Cardinal::Up    => vehicle.tile_pos.y > -3,
                            Cardinal::Down  => vehicle.tile_pos.y < 7,
                            Cardinal::Left  => vehicle.tile_pos.x > -2,
                            Cardinal::Right => vehicle.tile_pos.x < 14,
                        }
                    });

                    if time % 60 == 0 {
                        let chosen =
                            *possible.choose(&mut rand::thread_rng()).unwrap();

                        if !vehicles.iter().any(|vehicle| vehicle.tile_pos == chosen.tile_pos) {
                            vehicles.push(chosen);
                        }
                    }

                    renderer.update(&vehicles, &tiles, &stoplights).unwrap();
                }

                _ => {}
            }
        }
    });

    Ok(())
}
