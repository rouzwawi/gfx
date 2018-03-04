// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate winit;
extern crate image;

pub use gfx_app::{ColorFormat, DepthFormat};

use cgmath::prelude::*;
use cgmath::{Deg, Matrix3, Matrix4, Point3, Vector3};
use gfx::{Bundle, texture};

use std::f32::consts::PI;

// Declare the vertex format suitable for drawing,
// as well as the constants used by the shaders
// and the pipeline state object format.
// Notice the use of FixedPoint.
gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
    }

//    constant Locals {
//        transform: [[f32; 4]; 4] = "u_Transform",
//    }

    pipeline pipe {
        // vertex data
        vbuf: gfx::VertexBuffer<Vertex> = (),

        // vertex shader transform matrix -> uniform Locals { mat4 u_Transform; };
//        locals: gfx::ConstantBuffer<Locals> = "Locals",

        // fragment shader texture sampler -> uniform sampler2D t_Color
        color: gfx::TextureSampler<[f32; 4]> = "t_Color",

        // uniforms
        transform: gfx::Global<[[f32; 4]; 4]> = "u_transform",
        resolution: gfx::Global<[f32; 2]> = "u_resolution",
        mouse: gfx::Global<[f32; 2]> = "u_mouse",
        time : gfx::Global<f32> = "u_time",

        // fragment shader final color -> Target0 = vec4(1.0)
        out_color: gfx::RenderTarget<ColorFormat> = "Target0",

        out_depth: gfx::DepthTarget<DepthFormat> =
            gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}


impl Vertex {
    fn new(p: [i8; 3], t: [i8; 2]) -> Vertex {
        Vertex {
            pos: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0],
            tex_coord: [t[0] as f32, t[1] as f32],
        }
    }
}

//----------------------------------------
struct App<R: gfx::Resources> {
    bundle: Bundle<R, pipe::Data<R>>,
    stand: Point3<f32>,
    mouse: [f32; 2],
    aspect_ratio: f32,
    fov: f32,
}

impl<R: gfx::Resources> App<R> {
    fn default_view(&self) -> Matrix4<f32> {
        Matrix4::look_at(
            self.stand,
            self.stand + self.look_vector(),
            Vector3::unit_y(),
        )
    }

    fn look_vector(&self) -> Vector3<f32> {
        let rx = cgmath::Rad(-self.mouse[0]*2.0);

        let rotate: Matrix4<f32> =
            Matrix4::from_translation(Vector3{x:0.0, y:2.0*self.mouse[1], z:0.0})
                * Matrix4::from_angle_y(rx)
        ;

        rotate.transform_point(Point3::new(0.0, 0.0, -1.0)).to_vec().normalize()
    }
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend, window_targets: gfx_app::WindowTargets<R>) -> Self {
        use gfx::traits::FactoryExt;

        let vs = gfx_app::shade::Source {
            glsl_120: include_bytes!("shader/cube_120.glslv"),
            glsl_150: include_bytes!("shader/cube_150_core.glslv"),
            glsl_es_100: include_bytes!("shader/cube_100_es.glslv"),
            glsl_es_300: include_bytes!("shader/cube_300_es.glslv"),
            hlsl_40:  include_bytes!("data/vertex.fx"),
            msl_11: include_bytes!("shader/cube_vertex.metal"),
            vulkan:   include_bytes!("data/vert.spv"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_120: include_bytes!("shader/cube_120.glslf"),
            glsl_150: include_bytes!("shader/cube_150_core.glslf"),
            glsl_es_100: include_bytes!("shader/cube_100_es.glslf"),
            glsl_es_300: include_bytes!("shader/cube_300_es.glslf"),
            hlsl_40:  include_bytes!("data/pixel.fx"),
            msl_11: include_bytes!("shader/cube_frag.metal"),
            vulkan:   include_bytes!("data/frag.spv"),
            .. gfx_app::shade::Source::empty()
        };

        let vertex_data = [
            // front (0, 0, 1)
            Vertex::new([-1, -1,  1], [0, 1]),
            Vertex::new([ 1, -1,  1], [1, 1]),
            Vertex::new([ 1,  1,  1], [1, 0]),
            Vertex::new([-1,  1,  1], [0, 0]),
            // back (0, 0, -1)
            Vertex::new([ 1, -1, -1], [0, 1]),
            Vertex::new([-1, -1, -1], [1, 1]),
            Vertex::new([-1,  1, -1], [1, 0]),
            Vertex::new([ 1,  1, -1], [0, 0]),
            // right (1, 0, 0)
            Vertex::new([ 1, -1, -1], [1, 1]),
            Vertex::new([ 1,  1, -1], [1, 0]),
            Vertex::new([ 1,  1,  1], [0, 0]),
            Vertex::new([ 1, -1,  1], [0, 1]),
            // left (-1, 0, 0)
            Vertex::new([-1, -1,  1], [1, 1]),
            Vertex::new([-1,  1,  1], [1, 0]),
            Vertex::new([-1,  1, -1], [0, 0]),
            Vertex::new([-1, -1, -1], [0, 1]),
            // top (0, 1, 0)
            Vertex::new([-1,  1,  1], [0, 1]),
            Vertex::new([ 1,  1,  1], [1, 1]),
            Vertex::new([ 1,  1, -1], [1, 0]),
            Vertex::new([-1,  1, -1], [0, 0]),
            // bottom (0, -1, 0)
            Vertex::new([-1, -1, -1], [0, 1]),
            Vertex::new([ 1, -1, -1], [1, 1]),
            Vertex::new([ 1, -1,  1], [1, 0]),
            Vertex::new([-1, -1,  1], [0, 0]),

            // ground plane
            Vertex::new([-5, -1,  5], [0, 1]),
            Vertex::new([ 5, -1,  5], [1, 1]),
            Vertex::new([ 5, -1, -5], [1, 0]),
            Vertex::new([-5, -1, -5], [0, 0]),
        ];

        let index_data: &[u16] = &[
             0,  1,  2,  2,  3,  0, // front
             4,  5,  6,  6,  7,  4, // back
             8,  9, 10, 10, 11,  8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // top
            20, 21, 22, 22, 23, 20, // bottom

            24, 25, 26, 26, 27, 24, // ground
        ];

        let (vbuf, slice) = factory.create_vertex_buffer_with_slice(&vertex_data, index_data);

        use image::GenericImage;
        let image_bytes = &include_bytes!("image/bg.png")[..];
        let texture_image = image::load(std::io::Cursor::new(image_bytes), image::ImageFormat::PNG).unwrap();
        println!("Loaded texture color {:?}", texture_image.color());
        println!("Loaded texture dims {:?}", texture_image.dimensions());

        let texture_rgba = texture_image.to_rgba();
        let (_, texture_view) = factory.create_texture_immutable_u8::<gfx::format::Rgba8>(
            texture::Kind::D2(
                texture_rgba.width() as u16,
                texture_rgba.height() as u16,
                texture::AaMode::Single),
            texture::Mipmap::Provided,
            &[&texture_rgba]
        ).unwrap();

        let sinfo = texture::SamplerInfo::new(
            texture::FilterMethod::Bilinear,
            texture::WrapMode::Clamp);
        let sampler = factory.create_sampler(sinfo);

        let pso = factory.create_pipeline_simple(
            vs.select(backend).unwrap(),
            ps.select(backend).unwrap(),
            pipe::new()
        ).unwrap();

        let dims = window_targets.color.get_dimensions();
        let proj = cgmath::perspective(Deg(45.0f32), window_targets.aspect_ratio, 1.0, 10.0);

        let data = pipe::Data {
            vbuf: vbuf,

            // uniforms
            transform: (proj).into(),
            resolution: [dims.0 as f32, dims.1 as f32],
            mouse: [0.0, 0.0],
            time: 0.0,
//            locals: factory.create_constant_buffer(1),

            // textures
            color: (texture_view, sampler),

            // buffers
            out_color: window_targets.color,
            out_depth: window_targets.depth,
        };

        App {
            bundle: Bundle::new(slice, pso, data),
            stand: Point3::new(0.0, 0.0, 5.0),
            mouse: [0.0, 0.0],
            aspect_ratio: window_targets.aspect_ratio,
            fov: 45.0,
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        let proj =
            cgmath::perspective(Deg(self.fov), self.aspect_ratio, 1.0, 100.0);

        self.bundle.data.transform = (proj * self.default_view()).into();
        self.bundle.data.time += 0.05;

//        let locals = Locals {
//            transform: self.bundle.data.transform,
//        };
//        encoder.update_constant_buffer(&self.bundle.data.locals, &locals);

        encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
        encoder.clear_depth(&self.bundle.data.out_depth, 1.0);
        self.bundle.encode(encoder);
    }

    fn on_resize(&mut self, window_targets: gfx_app::WindowTargets<R>) {
        let dims = window_targets.color.get_dimensions();
        self.bundle.data.out_color = window_targets.color;
        self.bundle.data.out_depth = window_targets.depth;

        // In this example the transform is static except for window resizes.
        self.aspect_ratio = window_targets.aspect_ratio;
        self.bundle.data.resolution = [dims.0 as f32, dims.1 as f32];
    }

    fn on(&mut self, _event: winit::WindowEvent) {
        use winit::WindowEvent::*;
        use winit::VirtualKeyCode;

        let dims = self.bundle.data.out_color.get_dimensions();
        if let CursorMoved { position, .. } = _event {
            let mouse_pos = [position.0 as f32, position.1 as f32];
            self.mouse = [
                2.0*mouse_pos[0]/dims.0 as f32 - 1.0,
                2.0*mouse_pos[1]/dims.1 as f32 - 1.0,
            ];
            self.bundle.data.mouse = [mouse_pos[0], dims.1 as f32-mouse_pos[1]];
        }

        if let KeyboardInput {
            input: winit::KeyboardInput { virtual_keycode: Some(key_code), .. }, ..
        } = _event {
            let look = self.look_vector();
            match key_code {
                VirtualKeyCode::Q => self.fov -= 2.0,
                VirtualKeyCode::Z => self.fov += 2.0,

                VirtualKeyCode::W => self.stand += look*0.2,
                VirtualKeyCode::S => self.stand += -look*0.2,
                VirtualKeyCode::A => self.stand += Vector3::unit_y().cross(look).normalize()*0.2,
                VirtualKeyCode::D => self.stand += -Vector3::unit_y().cross(look).normalize()*0.2,

                _ => ()
            }
        }
    }
}

pub fn main() {
    use gfx_app::Application;
    App::launch_simple("Cube example");
}
