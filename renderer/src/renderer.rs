use crate::texture_loader::TextureLoader;

use routing::vehicle::Vehicle;
use routing::stoplight::Stoplight;
use routing::tile::Tile;
use routing::tile_map::{TILE_SIZE_F, TileMap};

use gl::types::GLint;
use glutin::config::{ConfigTemplateBuilder, GlConfig};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{GlSurface, SurfaceAttributesBuilder, WindowSurface};

use skia_safe::gpu::gl::{FramebufferInfo, Interface};
use skia_safe::gpu::{backend_render_targets, surfaces, DirectContext, SurfaceOrigin};
use skia_safe::{Color, ColorType, Paint, Point, Surface};

use winit::dpi::PhysicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;

use anyhow::{Result, Context};

use std::ffi::CString;
use std::num::NonZeroU32;

pub const SCALE: f32 = 16.0;

pub struct Renderer {
    window:  Window,
    surface: Surface,

    gr_context: DirectContext,
    gl_config:  Box<dyn GlConfig>,
    gl_context: PossiblyCurrentContext,
    gl_surface: glutin::surface::Surface<WindowSurface>,

    loader: TextureLoader,
}

impl Renderer {
    pub fn new() -> Result<(Self, EventLoop<()>)> {
        let event_loop = EventLoop::new()?;

        let window_builder =
            WindowBuilder::new()
                .with_title("Traffic")
                .with_inner_size(PhysicalSize::new(2560, 1600));

        let template_builder =
            ConfigTemplateBuilder::new()
                .with_alpha_size(8)
                .with_transparency(true);

        let display_builder =
            DisplayBuilder::new()
                .with_window_builder(Some(window_builder));

        let (window, gl_config) =
            display_builder
                .build(&event_loop, template_builder, |configs|
                    configs
                        .reduce(|accum, config| {
                            let transparency_check =
                                config.supports_transparency().unwrap_or(false) &
                                !accum.supports_transparency().unwrap_or(false);

                            if transparency_check || config.num_samples() < accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                )
                .unwrap();

        let window = window.context("")?;

        let window_handle = window.raw_window_handle();
        let context_attributes =
            ContextAttributesBuilder::new().build(Some(window_handle));

        let aside_gl_context = unsafe {
            gl_config
                .display()
                .create_context(&gl_config, &context_attributes)?
        };

        let (width, height) = window.inner_size().into();

        let attrs =
            SurfaceAttributesBuilder::<WindowSurface>::new().build(
                window_handle,
                NonZeroU32::new(width).context("")?,
                NonZeroU32::new(height).context("")?,
            );

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &attrs)?
        };

        let gl_context = aside_gl_context.make_current(&gl_surface)?;

        gl::load_with(|s| {
            gl_config
                .display()
                .get_proc_address(
                    CString::new(s).unwrap().as_c_str()
                )
        });

        let interface =
            Interface::new_load_with(|name| {
                gl_config
                    .display()
                    .get_proc_address(
                        CString::new(name).unwrap().as_c_str()
                    )
            }).context("")?;

        let mut gr_context =
            DirectContext::new_gl(interface, None).context("")?;

        let surface = Self::create_surface(
            &window, &mut gr_context, &gl_config,
        )?;

        let loader = TextureLoader::new()?;

        Ok((
            Self {
                window,
                surface,
                gr_context,
                gl_context,
                gl_surface,
                loader,
                gl_config: Box::new(gl_config),
            },
            event_loop,
        ))
    }

    fn create_surface(
        window:     &Window,
        gr_context: &mut DirectContext,
        gl_config:  &dyn GlConfig,
    ) -> Result<Surface> {
        let fb_info = {
            let mut fboid: GLint = 0;
            unsafe {
                gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid)
            };

            FramebufferInfo {
                fboid:  fboid.try_into()?,
                format: skia_safe::gpu::gl::Format::RGBA8.into(),

                ..Default::default()
            }
        };

        let backend_render_target =
            backend_render_targets::make_gl(
                window.inner_size().into(),
                gl_config.num_samples() as usize,
                gl_config.stencil_size() as usize,
                fb_info,
            );

        surfaces::wrap_backend_render_target(
            gr_context,
            &backend_render_target,
            SurfaceOrigin::BottomLeft,
            ColorType::RGBA8888,
            None,
            None,
        ).context("")
    }

    pub fn update_surface(&mut self) -> Result<()> {
        self.surface = Self::create_surface(
            &self.window,
            &mut self.gr_context,
            &*self.gl_config,
        )?;

        self.gl_surface.resize(
            &self.gl_context,
            NonZeroU32::new(self.window.inner_size().width).context("")?,
            NonZeroU32::new(self.window.inner_size().height).context("")?,
        );

        self.window.request_redraw();

        Ok(())
    }

    pub fn update(
        &mut self,
        vehicles:   &[Vehicle],
        tiles:      &TileMap<Tile>,
        stoplights: &[Stoplight],
    ) -> Result<()> {
        let canvas = self.surface.canvas();
        let offset = self.gl_surface.height().unwrap() as f32 / 4.0;

        let mut paint = Paint::default();
        paint.set_color(Color::BLUE);

        canvas.clear(Color::WHITE);

        for tile in &tiles.tiles {
            let pos = Point::new(tile.pos.x as f32 + 0.5, tile.pos.y as f32);

            let img_pos = Point::new(
                (pos.x * TILE_SIZE_F - (TILE_SIZE_F / 2.0)) * SCALE,
                (pos.y * TILE_SIZE_F - (TILE_SIZE_F / 2.0)) * SCALE + offset,
            );

            let rot_pos = Point::new(
                img_pos.x + ((TILE_SIZE_F / 2.0) * SCALE),
                img_pos.y + ((TILE_SIZE_F / 2.0) * SCALE),
            );

            let (img, offset) = self.loader.get_tile(&tile.dir);
            let deg = tile.dir.degrees() + offset;

            canvas.rotate(deg, Some(rot_pos));
            canvas.draw_image(img, img_pos, None);
            canvas.rotate(-deg, Some(rot_pos));
        }

        for stoplight in stoplights {
            let pos = Point::new(
                stoplight.pos.x as f32 - 0.5,
                stoplight.pos.y as f32 - 1.0,
            );

            let img_pos = Point::new(
                (pos.x * TILE_SIZE_F) * SCALE,
                (pos.y * TILE_SIZE_F) * SCALE + offset,
            );

            let rot_pos = Point::new(
                img_pos.x + (TILE_SIZE_F * SCALE),
                img_pos.y + (TILE_SIZE_F * SCALE),
            );

            let img = &self.loader.get_stoplight(&stoplight);
            let deg = stoplight.axis.degrees();

            canvas.rotate(deg, Some(rot_pos));
            canvas.draw_image(img, img_pos, None);
            canvas.rotate(-deg, Some(rot_pos));
        }

        for vehicle in vehicles {
            let pos = vehicle.pos;

            let img_pos = Point::new(
                pos.x * SCALE,
                (pos.y - (TILE_SIZE_F / 2.0)) * SCALE + offset
            );

            let rot_pos = Point::new(
                img_pos.x + ((TILE_SIZE_F / 2.0) * SCALE),
                img_pos.y + ((TILE_SIZE_F / 2.0) * SCALE),
            );

            canvas.rotate(vehicle.dir.degrees(), Some(rot_pos));
            canvas.draw_image(&self.loader.car, img_pos, None);
            canvas.rotate(-vehicle.dir.degrees(), Some(rot_pos));
        }

        self.gr_context.flush_and_submit();
        self.gl_surface.swap_buffers(&self.gl_context)?;

        self.window.request_redraw();

        Ok(())
    }
}
