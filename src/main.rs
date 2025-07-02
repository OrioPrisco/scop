use glfw::{Action, Context, GlfwReceiver, Key};
use std::cell::RefCell;
use std::ffi::CStr;
use std::ffi::c_void;
use std::mem;
use std::ptr;

use scop::ebo::Ebo;
use scop::shader::{Shader, ShaderProgram};
use scop::vao::{BoundVao, Vao};
use scop::vbo::Vbo;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

#[rustfmt::skip]
const vertices: &[f32] = &[
     //positions      //colors        //texture coords
     0.5,  0.5, 0.0,  1.0, 0.0, 0.0,  1.0, 0.0,
     0.5, -0.5, 0.0,  0.0, 1.0, 0.0,  1.0, 1.0,
    -0.5, -0.5, 0.0,  0.0, 0.0, 1.0,  0.0, 1.0,
    -0.5,  0.5, 0.0,  1.0, 1.0, 1.0,  0.0, 0.0,
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
    let mut texture = 0;
    let mut texture2 = 0;
    unsafe {
        gl::GenTextures(1, &mut texture);
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = image::ImageReader::open("img/test.png")
            .expect("Cannot find texture")
            .decode()
            .expect("Cannot load texture");
        let img = match img {
            image::DynamicImage::ImageRgba8(img) => img,
            image => image.to_rgba8(),
        };
        let dim = img.dimensions();
        let pixels = img.into_vec();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            dim.0 as i32,
            dim.1 as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &pixels[0] as *const u8 as *const c_void,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
        while let Err(error) = scop::get_error() {
            println!("{}", error);
        }
        drop(pixels);
    }

    unsafe {
        gl::GenTextures(1, &mut texture2);
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, texture2);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let img = image::ImageReader::open("img/awesomeface.png")
            .expect("Cannot find texture")
            .decode()
            .expect("Cannot load texture");
        let img = match img {
            image::DynamicImage::ImageRgba8(img) => img,
            image => image.to_rgba8(),
        };
        let dim = img.dimensions();
        let pixels = img.into_vec();
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            dim.0 as i32,
            dim.1 as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            &pixels[0] as *const u8 as *const c_void,
        );

        gl::GenerateMipmap(gl::TEXTURE_2D);
        while let Err(error) = scop::get_error() {
            println!("{}", error);
        }
        drop(pixels);
    }

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0); //safe
            gl::Clear(gl::COLOR_BUFFER_BIT); //can error on bad bit passed

            //gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        };

        let time_value = glfw.get_time() as f32;
        let green_value = time_value.sin() / 2.0 + 0.5;
        shader_program.use_program();
        //unsafe {shader_program.set4f(c"our_color", 0.0, green_value, 0.0, 1.0)}.unwrap();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);
        }
        unsafe { shader_program.set1i(c"texture1", 0) };
        unsafe { shader_program.set1i(c"texture2", 1) };
        let bound_vao = BoundVao::new(&mut vao, context);
        bound_vao.draw_elements();
        context = bound_vao.unbind();

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
