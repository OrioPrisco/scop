pub mod gl;
pub mod math;
pub mod obj;

use glfw::{Action, Context, GlfwReceiver, Key};
use std::cell::RefCell;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use gl::ebo::Ebo;
use gl::shader::{Shader, ShaderProgram};
use gl::texture::{self, Texture};
use gl::vao::{BoundVao, Vao};
use gl::vbo::Vbo;
use math::matrix::Mat4;
use math::vector::Vector3;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub struct Config {
    path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("No query String"),
        };
        Ok(Config { path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!())?;
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "scop", glfw::WindowMode::Windowed)
        .ok_or("Could not create window")?;

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol));

    let file = BufReader::new(File::open(config.path)?);
    let model = obj::parse_obj(file)?;

    let mut context = gl::Context::new();
    let mut texture_contexts = texture::get_contexts();
    let mut active_texture = texture::get_active_context();

    let mut vao = Vao::new()?;
    let vbo = RefCell::new(Vbo::new()?);
    let ebo = RefCell::new(Ebo::new()?);

    let mut bound_vao = BoundVao::new(&mut vao, context);
    bound_vao.bind_vbo(&vbo);
    bound_vao.bind_ebo(&ebo);

    vbo.borrow_mut().bind_data(&model.vertices);
    ebo.borrow_mut().bind_data(&model.indices);
    context = bound_vao.unbind();

    let vertex_shader_id = Shader::from_path("./src/vertex.glsl", gl::VERTEX_SHADER)?;
    let fragment_shader_id = Shader::from_path("./src/fragment.glsl", gl::FRAGMENT_SHADER)?;

    let shader_program = ShaderProgram::new(&vertex_shader_id, &fragment_shader_id)?;
    //TODO delete shaders

    let texture = Texture::new();
    let texture2 = Texture::new();
    let mut txt_ctx_0 = texture_contexts.remove(0);
    let mut txt_ctx_1 = texture_contexts.remove(0);
    let bound_text = texture.bind(&mut txt_ctx_0, &mut active_texture);
    let bound_text2 = texture2.bind(&mut txt_ctx_1, &mut active_texture);

    bound_text.bind_data_from_path("img/test.png", &mut active_texture)?;
    bound_text2.bind_data_from_path("img/awesomeface.png", &mut active_texture)?;

    let mut camera_pos = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 3.0,
    };
    let j = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let ijk = Vector3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    let camera_target = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let up = Vector3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    let mut last_frame = glfw.get_time() as f32;
    let mut scale = 1.0;
    let mut projection =
        Mat4::perspective(45.0, (SCR_WIDTH as f32) / (SCR_HEIGHT as f32), 0.1, 100.0);
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    while !window.should_close() {
        if let Some((width, height)) = process_events(&events) {
            projection = Mat4::perspective(45.0, (width as f32) / (height as f32), 0.1, 100.0);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); //safe
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); //can error on bad bit passed

            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        };

        let time_value = glfw.get_time() as f32;
        let delta_time = time_value - last_frame;
        last_frame = time_value;
        let camera_front = camera_target - camera_pos;

        process_input(
            &mut window,
            delta_time,
            &mut camera_pos,
            &camera_front,
            &up,
            &mut scale,
        );

        let view = Mat4::lookat(camera_pos, camera_target, up);
        let model = Mat4::rotate(&j, time_value / 6.0) * Mat4::scale(&(ijk * scale));

        shader_program.use_program();
        unsafe { shader_program.set_mat(c"view", &view) }.ok_or("Cannot set view uniform")?;
        unsafe { shader_program.set_mat(c"projection", &projection) }
            .ok_or("Cannot set projection uniform")?;
        unsafe { shader_program.set_texture(c"texture1", &bound_text) };
        unsafe { shader_program.set_texture(c"texture2", &bound_text2) };
        let bound_vao = BoundVao::new(&mut vao, context);
        unsafe { shader_program.set_mat(c"model", &model) }.ok_or("Cannot set model uniform")?;

        bound_vao.draw_elements();
        context = bound_vao.unbind();

        window.swap_buffers();
        glfw.poll_events();
    }
    Ok(())
}

fn process_events(events: &GlfwReceiver<(f64, glfw::WindowEvent)>) -> Option<(i32, i32)> {
    let mut ret = None;
    for (_, event) in glfw::flush_messages(events) {
        if let glfw::WindowEvent::FramebufferSize(width, height) = event {
            unsafe { gl::Viewport(0, 0, width, height) };
            ret = Some((width, height));
        }
    }
    ret
}

fn process_input(
    window: &mut glfw::Window,
    delta_time: f32,
    camera_pos: &mut Vector3<f32>,
    camera_front: &Vector3<f32>,
    camera_up: &Vector3<f32>,
    scale: &mut f32,
) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true)
    }

    let camera_speed = 2.5 * delta_time;
    if window.get_key(Key::W) == Action::Press {
        *camera_pos += camera_front * camera_speed;
    }
    if window.get_key(Key::S) == Action::Press {
        *camera_pos -= camera_front * camera_speed;
    }
    if window.get_key(Key::A) == Action::Press {
        *camera_pos += -(camera_front.cross(camera_up).normalized() * camera_speed);
    }
    if window.get_key(Key::D) == Action::Press {
        *camera_pos += camera_front.cross(camera_up).normalized() * camera_speed;
    }
    let camera_right = camera_front.cross(camera_up);
    let camera_up = camera_right.cross(camera_front);
    if window.get_key(Key::Space) == Action::Press {
        *camera_pos += camera_up.normalized() * camera_speed;
    }
    if window.get_key(Key::LeftShift) == Action::Press {
        *camera_pos -= camera_up.normalized() * camera_speed;
    }
    if window.get_key(Key::KpAdd) == Action::Press {
        *scale += 1.01 * delta_time;
    }
    if window.get_key(Key::KpSubtract) == Action::Press {
        *scale -= 1.01 * delta_time;
    }
}
