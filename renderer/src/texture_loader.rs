use crate::renderer::SCALE;
use routing::direction::{Cardinal, Direction, TileDirection};

use image::{EncodableLayout, ImageBuffer, Rgba};
use image::imageops::{self, FilterType};
use routing::stoplight::{Stoplight, GRACE_TIME};
use skia_safe::{images, AlphaType, ColorType, Image, ImageInfo};

use anyhow::{Result, Context};

pub struct TextureLoader {
    pub car: Image,

    straight:     Image,
    turn:         Image,
    intersection: Image,

    pub stop_signs: Image,

    stoplight:       Image,
    stoplight_grace: Image,
    stoplight_stop:  Image,
}

impl TextureLoader {
    pub fn new() -> Result<Self> {
        Ok(Self {
            car: load_image(include_bytes!("assets/car.png"))?,

            straight:     load_image(include_bytes!("assets/straight.png"))?,
            turn:         load_image(include_bytes!("assets/turn.png"))?,
            intersection: load_image(include_bytes!("assets/intersection.png"))?,

            stop_signs: load_image(include_bytes!("assets/stop-signs.png"))?,

            stoplight:       load_image(include_bytes!("assets/stoplight.png"))?,
            stoplight_grace: load_image(include_bytes!("assets/stoplight-grace.png"))?,
            stoplight_stop:  load_image(include_bytes!("assets/stoplight-stop.png"))?,
        })
    }

    pub fn get_tile(&self, tile_dir: &TileDirection) -> (&Image, f32) {
        match tile_dir {
            TileDirection::Constant(dir) => {
                match dir {
                    Direction::Straight(_) => (&self.straight, 0.0),

                    Direction::Turn(Cardinal::Left,  Cardinal::Up)    |
                    Direction::Turn(Cardinal::Right, Cardinal::Down)  |
                    Direction::Turn(Cardinal::Up,    Cardinal::Right) |
                    Direction::Turn(Cardinal::Down,  Cardinal::Left)
                        => (&self.turn, -45.0 + 180.0),

                    Direction::Turn(_, _) => (&self.turn, -45.0),
                }
            }

            TileDirection::Intersection(_) => (&self.intersection, 0.0),
        }
    }

    pub fn get_stoplight(&self, stoplight: &Stoplight) -> &Image {
        if let Some(period) = stoplight.grace {
            if period >= ((GRACE_TIME - 1.0) * 60.0) as u32 {
                &self.stoplight_stop
            } else {
                &self.stoplight_grace
            }
        } else {
            &self.stoplight
        }
    }
}

fn load_image(data: &[u8]) -> Result<Image> {
    let img =
        image::load_from_memory(data)
            .unwrap()
            .to_rgba8();

    let resized = imageops::resize(
        &img,
        img.width()  * SCALE as u32,
        img.height() * SCALE as u32,
        FilterType::Nearest,
    );

    skia_image(&resized)
}

fn skia_image(img: &ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<Image> {
    let info = ImageInfo::new(
        (img.width() as i32, img.height() as i32),
        ColorType::RGBA8888,
        AlphaType::Premul,
        None
    );

    let image = images::raster_from_data(
        &info,
        skia_safe::Data::new_copy(img.as_bytes()),
        (img.width() * 4) as usize,
    ).context("")?;

    Ok(image)
}
