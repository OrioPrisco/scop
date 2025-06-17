use glfw::{Context, GlfwReceiver, Action, Key};
use std::ptr;
use std::mem;
use std::ffi::c_void;
use std::ffi::CStr;

use scop::vao::{Vao, BoundVao};
use scop::vbo::{Vbo, BoundVbo};

const SCR_WIDTH : u32 = 800;
const SCR_HEIGHT : u32 = 600;

const vertices : [f32;9] = [
    -0.5, -0.5, 0.0,
     0.5, -0.5, 0.0,
     0.0, 0.5, 0.0,
];

const vertex_shader : &CStr = c"
#version 330 core
layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}";

const fragment_shader : &CStr = c"
#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}";

fn main() {
    use glfw::fail_on_errors;
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3,3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(SCR_WIDTH, SCR_HEIGHT, "scop", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW Window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    gl::load_with(|symbol| window.get_proc_address(symbol));

    let mut context = scop::GLContext::new();

    let mut vao = Vao::new().unwrap();
    let mut vbo = Vbo::new().unwrap();

    let bound_vao = BoundVao::new(&vao, &mut context.vao);
    let bound_vbo = BoundVbo::new(&vbo, &mut context.vbo);

    unsafe {
        gl::BufferData(gl::ARRAY_BUFFER, (mem::size_of::<f32>() * vertices.len()) as isize, vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<f32>() as i32, ptr::null() as *const c_void);
        gl::EnableVertexAttribArray(0);
    };

    let vertex_shader_id = unsafe{ gl::CreateShader(gl::VERTEX_SHADER) };
    let fragment_shader_id = unsafe{ gl::CreateShader(gl::FRAGMENT_SHADER) };
    unsafe {
        let source_ptr : *const i8 = vertex_shader.as_ptr() as *const i8;
        gl::ShaderSource(vertex_shader_id, 1, &raw const source_ptr, ptr::null());//Todo : put this in a function
        gl::CompileShader(vertex_shader_id);
        let mut status : i32 = 0;
        let mut info_log : [u8; 512] = [0; 512];
        let mut info_log_size : i32 = 0;
        gl::GetShaderiv(vertex_shader_id, gl::COMPILE_STATUS, &mut status);
        gl::GetShaderInfoLog(vertex_shader_id, 512, &mut info_log_size, info_log.as_mut_ptr() as *mut i8);
        println!("{status} : {}", std::str::from_utf8_unchecked(&info_log[..info_log_size as usize]));
        //TODO: check Compile status
        let source_ptr : *const i8 = fragment_shader.as_ptr() as *const i8;
        gl::ShaderSource(fragment_shader_id, 1, &raw const source_ptr, ptr::null());//Todo : put this in a function
        gl::CompileShader(fragment_shader_id);
        //TODO: check Compile status
        gl::GetShaderiv(fragment_shader_id, gl::COMPILE_STATUS, &mut status);
        gl::GetShaderInfoLog(fragment_shader_id, 512, &mut info_log_size, info_log.as_mut_ptr() as *mut i8);
        println!("{status} : {}", std::str::from_utf8_unchecked(&info_log[..info_log_size as usize]));

        //unbind vao and vbo
        drop(bound_vbo);
        drop(bound_vao);

    };
    let shader_program : u32 = unsafe {gl::CreateProgram()};
    unsafe {
        gl::AttachShader(shader_program, vertex_shader_id);
        gl::AttachShader(shader_program, fragment_shader_id);
        gl::LinkProgram(shader_program);
        //TODO check linking success

        gl::UseProgram(shader_program);
    };
    //TODO delete shaders

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);//safe
            gl::Clear(gl::COLOR_BUFFER_BIT);//can error on bad bit passed

            gl::UseProgram(shader_program);
            let _bound_vao = BoundVao::new(&vao, &mut context.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        };

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window : &mut glfw::Window, events : &GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height)};
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {},
        }
    }
}
