use glfw::{Action, Context, GlfwReceiver, Key};
use std::cell::RefCell;
use std::f32::consts::TAU;
use std::ffi::CStr;
use std::ffi::c_void;
use std::mem;
use std::ptr;

use scop::ebo::Ebo;
use scop::math::matrix::Mat4;
use scop::math::vector::{Vector3, Vector4};
use scop::shader::{Shader, ShaderProgram};
use scop::texture::{self, Texture};
use scop::vao::{BoundVao, Vao};
use scop::vbo::Vbo;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

#[rustfmt::skip]
const vertices: &[f32] = &[
     //positions       //colors          //texture coords
    -0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  0.0,
     0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
     0.5,  0.5, -0.5,  1.0,  1.0,  1.0,  1.0,  1.0,
     0.5,  0.5, -0.5,  1.0,  1.0,  1.0,  1.0,  1.0,
    -0.5,  0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
    -0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  0.0,
    -0.5, -0.5,  0.5,  1.0,  1.0,  1.0,  0.0,  0.0,
     0.5, -0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
     0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  1.0,
     0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  1.0,
    -0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,  1.0,  1.0,  1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
    -0.5,  0.5, -0.5,  1.0,  1.0,  1.0,  1.0,  1.0,
    -0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
    -0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
    -0.5, -0.5,  0.5,  1.0,  1.0,  1.0,  0.0,  0.0,
    -0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
     0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
     0.5,  0.5, -0.5,  1.0,  1.0,  1.0,  1.0,  1.0,
     0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
     0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
     0.5, -0.5,  0.5,  1.0,  1.0,  1.0,  0.0,  0.0,
     0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
    -0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
     0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  1.0,  1.0,
     0.5, -0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
     0.5, -0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
    -0.5, -0.5,  0.5,  1.0,  1.0,  1.0,  0.0,  0.0,
    -0.5, -0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
    -0.5,  0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0,
     0.5,  0.5, -0.5,  1.0,  1.0,  1.0,  1.0,  1.0,
     0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
     0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  1.0,  0.0,
    -0.5,  0.5,  0.5,  1.0,  1.0,  1.0,  0.0,  0.0,
    -0.5,  0.5, -0.5,  1.0,  1.0,  1.0,  0.0,  1.0

];

#[rustfmt::skip]
const cube_positions : &[Vector3<f32>] = &[
    Vector3{x:  0.0, y:  0.0, z:  0.0},
    Vector3{x:  2.0, y:  5.0, z:-15.0},
    Vector3{x: -1.5, y: -2.2, z: -2.5},
    Vector3{x: -3.8, y: -2.0, z:-12.3},
    Vector3{x:  2.4, y: -0.4, z: -3.5},
    Vector3{x: -1.7, y:  3.0, z: -7.5},
    Vector3{x:  1.3, y: -2.0, z: -2.5},
    Vector3{x:  1.5, y:  2.0, z: -2.5},
    Vector3{x:  1.5, y:  0.2, z: -1.5},
    Vector3{x: -1.3, y:  1.0, z: -1.5},
];

#[rustfmt::skip]
const indices: [u32; 6] = [
    0, 1, 3,
    1, 2, 3,
];

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "scop", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol));

    let mut context = scop::Context::new();
    let mut texture_contexts = texture::get_contexts();
    let mut active_texture = texture::get_active_context();

    let mut vao = Vao::new().unwrap();
    let vbo = RefCell::new(Vbo::new().unwrap());
    let ebo = RefCell::new(Ebo::new().unwrap());

    let mut bound_vao = BoundVao::new(&mut vao, context);
    bound_vao.bind_vbo(&vbo);
    bound_vao.bind_ebo(&ebo);

    vbo.borrow_mut().bind_data(&vertices);
    ebo.borrow_mut().bind_data(&indices);
    context = bound_vao.unbind();

    let vertex_shader_id = Shader::from_path("./src/vertex.glsl", gl::VERTEX_SHADER).unwrap();
    let fragment_shader_id = Shader::from_path("./src/fragment.glsl", gl::FRAGMENT_SHADER).unwrap();

    let shader_program = ShaderProgram::new(&vertex_shader_id, &fragment_shader_id).unwrap();
    //TODO delete shaders

    let texture = Texture::new();
    let texture2 = Texture::new();
    let mut txt_ctx_0 = texture_contexts.remove(0);
    let mut txt_ctx_1 = texture_contexts.remove(0);
    let bound_text = texture.bind(&mut txt_ctx_0, &mut active_texture);
    let bound_text2 = texture2.bind(&mut txt_ctx_1, &mut active_texture);

    bound_text
        .bind_data_from_path("img/test.png", &mut active_texture)
        .expect("Cannot load texture");
    bound_text2
        .bind_data_from_path("img/awesomeface.png", &mut active_texture)
        .expect("Cannot load texture");

    let view = Mat4::translate(&Vector3 {
        x: 0.0,
        y: 0.0,
        z: -3.0,
    });
    let mut projection =
        Mat4::perspective(45.0, (SCR_WIDTH as f32) / (SCR_HEIGHT as f32), 0.1, 100.0);
    unsafe { gl::Enable(gl::DEPTH_TEST) };
    while !window.should_close() {
        if let Some((width, height)) = process_events(&mut window, &events) {
            projection = Mat4::perspective(45.0, (width as f32) / (height as f32), 0.1, 100.0);
        }

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); //safe
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); //can error on bad bit passed

            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        };

        let time_value = glfw.get_time() as f32;

        let green_value = time_value.sin() / 2.0 + 0.5;
        shader_program.use_program();
        unsafe { shader_program.set_mat(c"view", &view) }.unwrap();
        unsafe { shader_program.set_mat(c"projection", &projection) }.unwrap();
        //unsafe {shader_program.set4f(c"our_color", 0.0, green_value, 0.0, 1.0)}.unwrap();
        unsafe { shader_program.set_texture(c"texture1", &bound_text) };
        unsafe { shader_program.set_texture(c"texture2", &bound_text2) };
        let bound_vao = BoundVao::new(&mut vao, context);
        for (index, position) in cube_positions.iter().enumerate() {
            let model = Mat4::translate(&position)
                 * Mat4::rotate(
                    &Vector3 {
                        x: 1.0,
                        y: 0.3,
                        z: 0.5,
                    }
                    .normalized(),
                    (20.0 * index as f32).to_radians(),
                );
            unsafe { shader_program.set_mat(c"model", &model) }.unwrap();

            bound_vao.draw_triangles();
        }
        context = bound_vao.unbind();

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) -> Option<(i32,i32)>{
    let mut ret = None;
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) };
                ret = Some((width, height));
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
    ret
}
