#[macro_use]
extern crate gfx;
extern crate gfx_app;

extern crate cgmath; // vector math
extern crate winit; // window events
extern crate image; // loading textures from images

extern crate tobj;

use gfx_app::{ColorFormat, DepthFormat};

use gfx::{Bundle, Slice, texture, handle};
use gfx::format::Rgba8;

use cgmath::prelude::*;
use cgmath::{Deg, Matrix3, Matrix4, Point3, Vector3};

use tobj::{Model, Mesh};

use std::path::Path;

// Declare the vertex format suitable for drawing,
// as well as the constants used by the shaders
// and the pipeline state object format.
// Notice the use of FixedPoint.
gfx_defines!{
    vertex Vertex {
        pos: [f32; 4] = "a_Pos",
        tex_coord: [f32; 2] = "a_TexCoord",
        normal: [f32; 3] = "a_Normal",
    }

    pipeline pipe {
        // vertex data
        vbuf: gfx::VertexBuffer<Vertex> = (),

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
            normal: [0.0, 0.0, 0.0],
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
    fn new<F: gfx::Factory<R>>(
        factory: &mut F,
        backend: gfx_app::shade::Backend,
        window_targets: gfx_app::WindowTargets<R>) -> Self {
        use gfx::traits::FactoryExt;

        let vs = gfx_app::shade::Source {
            glsl_120: include_bytes!("shader/cube_120.glslv"),
            glsl_150: include_bytes!("shader/cube_150_core.glslv"),
            glsl_es_100: include_bytes!("shader/cube_100_es.glslv"),
            glsl_es_300: include_bytes!("shader/cube_300_es.glslv"),
            msl_11: include_bytes!("shader/cube_vertex.metal"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_120: include_bytes!("shader/cube_120.glslf"),
            glsl_150: include_bytes!("shader/cube_150_core.glslf"),
            glsl_es_100: include_bytes!("shader/cube_100_es.glslf"),
            glsl_es_300: include_bytes!("shader/cube_300_es.glslf"),
            msl_11: include_bytes!("shader/cube_frag.metal"),
            .. gfx_app::shade::Source::empty()
        };

//        let cube = Cube {};
//        let (vbuf, slice) = cube.vbuf(factory);
        let (vbuf, slice) = teapot(factory);
        let (texture_view, sampler) = load_texture(factory);

        let shader_set = factory.create_shader_set(vs.select(backend).unwrap(),
                                                   ps.select(backend).unwrap()).unwrap();
        let pso = factory.create_pipeline_state(&shader_set,
                                                gfx::Primitive::TriangleList,
                                                gfx::state::Rasterizer::new_fill().with_cull_back(),
                                                pipe::new()).unwrap();
//        let pso = factory.create_pipeline_simple(
//            vs.select(backend).unwrap(),
//            ps.select(backend).unwrap(),
//            pipe::new()
//        ).unwrap();

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

fn teapot<R, F>(factory: &mut F) -> (handle::Buffer<R, Vertex>, Slice<R>)
    where R: gfx::Resources,
          F: gfx::Factory<R> {
    let path = Path::new("examples/cube/res/bunny.obj");
    println!("opening {:?}", path);

    let teapot = tobj::load_obj(&path);
    let models: Vec<Model> = teapot.unwrap().0;

    let model = &models[0];
    let mesh: &Mesh = &model.mesh;
    println!("model.name = \'{}\'", model.name);
    println!("model.mesh.material_id = {:?}", mesh.material_id);
    println!("mesh.indices: {}", mesh.indices.len());
    println!("mesh.positions: {}", mesh.positions.len());
    println!("mesh.texcoords: {}", mesh.texcoords.len());
    println!("mesh.normals: {}", mesh.normals.len());

    let mut vertices: Vec<Vertex> = Vec::new();
    let indices: &Vec<u32> = &mesh.indices;

    let mut poss = mesh.positions.chunks(3);
    let mut texs = mesh.texcoords.chunks(2);
    let mut nrms = mesh.normals.chunks(3);
    while let (Some(pos), Some(tex), Some(nrm)) = (poss.next(), texs.next(), nrms.next()) {
        vertices.push(Vertex {
            pos: [pos[0], pos[1], pos[2], 1.0],
            tex_coord: [tex[0], tex[1]],
            normal: [nrm[0], nrm[1], nrm[2]],
        })
    }

    use gfx::traits::FactoryExt;
    factory.create_vertex_buffer_with_slice(vertices.as_slice(), indices.as_slice())
}

struct Cube {
}

impl Cube {
    fn vbuf<R, F>(&self, factory: &mut F) -> (handle::Buffer<R, Vertex>, Slice<R>)
        where R: gfx::Resources,
              F: gfx::Factory<R> {
        use gfx::traits::FactoryExt;

        let vertices: &[Vertex] = &[
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

        let indices: &[u32] = &[
//            0,  1,  2,  2,  3,  0, // front
//            4,  5,  6,  6,  7,  4, // back
//            8,  9, 10, 10, 11,  8, // right
//            12, 13, 14, 14, 15, 12, // left
//            16, 17, 18, 18, 19, 16, // top
//            20, 21, 22, 22, 23, 20, // bottom

            24, 25, 26, 26, 27, 24, // ground
        ];

        factory.create_vertex_buffer_with_slice(vertices, indices)
    }
}

fn load_texture<R, F>(factory: &mut F) -> (handle::ShaderResourceView<R, [f32; 4]>,
                                           handle::Sampler<R>)
    where R: gfx::Resources,
          F: gfx::Factory<R> {
    use image::GenericImage;
    let image_bytes = &include_bytes!("image/bg.png")[..];
    let texture_image = image::load(std::io::Cursor::new(image_bytes), image::ImageFormat::PNG).unwrap();
    println!("Loaded texture color {:?}", texture_image.color());
    println!("Loaded texture dims {:?}", texture_image.dimensions());

    let texture_rgba = texture_image.to_rgba();
    let (_, texture_view) = factory.create_texture_immutable_u8::<Rgba8>(
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

    (texture_view, sampler)
}
