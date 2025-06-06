use glfw::{Context, GlfwReceiver};

const SCR_WIDTH : u32 = 800;
const SCR_HEIGHT : u32 = 600;


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

    while !window.should_close() {
        process_events(&mut window, &events);
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(_window : &mut glfw::Window, _events : &GlfwReceiver<(f64, glfw::WindowEvent)>) {

}
