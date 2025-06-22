use glfw::{Action, Context, GlfwReceiver, Key};
use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::c_void;
use std::mem;
use std::ptr;

use scop::shader::{Shader, ShaderProgram};
use scop::vao::{BoundVao, Vao};
use scop::vbo::Vbo;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

const vertices: [f32; 9] = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

const vertex_shader: &CStr = c"
#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const fragment_shader: &CStr = c"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}";

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

    let mut vao = Vao::new().unwrap();
    let vbo = RefCell::new(Vbo::new().unwrap());

    let mut bound_vao = BoundVao::new(&mut vao, context);
    bound_vao.bind_vbo(&vbo);

    bound_vao.get_vbo().unwrap().bind_data(&vertices);
    context = bound_vao.unbind();

    let vertex_shader_id = Shader::new(vertex_shader, gl::VERTEX_SHADER).unwrap();
    let fragment_shader_id = Shader::new(fragment_shader, gl::FRAGMENT_SHADER).unwrap();

    let shader_program = ShaderProgram::new(&vertex_shader_id, &fragment_shader_id).unwrap();
    //TODO delete shaders

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); //safe
            gl::Clear(gl::COLOR_BUFFER_BIT); //can error on bad bit passed

            shader_program.use_program();
            let bound_vao = BoundVao::new(&mut vao, context);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            context = bound_vao.unbind();
        };

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) };
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
