use gl::types::*;
use std::cell::RefCell;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::c_void;
use std::fmt::{self, Display};
use std::mem;
use std::ptr;

pub struct Context;

//Empty struct that needs to be borrwed to bind a vao/vbo
//Allows the borrow checker to detect lifetime issues with vao binding
impl Context {
    pub fn new() -> Context {
        Context
    }
}

#[derive(Debug)]
pub enum GLError {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    StackOverflow,
    StackUnderflow,
    OutOfMemory,
    InvalidFramebufferOperation,
    ContextLost,
    Unknown(GLenum),
}

type GLResult<T> = Result<T, GLError>;

impl Display for GLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GLError::InvalidEnum => write!(f, "InvalidEnum"),
            GLError::InvalidValue => write!(f, "InvalidValue"),
            GLError::InvalidOperation => write!(f, "InvalidOperation"),
            GLError::StackOverflow => write!(f, "StackOverflow"),
            GLError::StackUnderflow => write!(f, "StackUnderflow"),
            GLError::OutOfMemory => write!(f, "OutOfMemory"),
            GLError::InvalidFramebufferOperation => write!(f, "InvalidFramebufferOperation"),
            GLError::ContextLost => write!(f, "ContextLost"),
            GLError::Unknown(err) => write!(f, "UnknownError({err})"),
        }
    }
}

impl Error for GLError {}

pub fn get_error() -> GLResult<()> {
    let err: GLenum = unsafe { gl::GetError() };
    match err {
        0 => Ok(()),
        gl::INVALID_ENUM => Err(GLError::InvalidEnum),
        gl::INVALID_VALUE => Err(GLError::InvalidValue),
        gl::INVALID_OPERATION => Err(GLError::InvalidOperation),
        gl::STACK_OVERFLOW => Err(GLError::StackOverflow),
        gl::STACK_UNDERFLOW => Err(GLError::StackUnderflow),
        gl::OUT_OF_MEMORY => Err(GLError::OutOfMemory),
        gl::INVALID_FRAMEBUFFER_OPERATION => Err(GLError::InvalidFramebufferOperation),
        gl::CONTEXT_LOST => Err(GLError::ContextLost),
        err => Err(GLError::Unknown(err)),
    }
}

type VboRef<'vbo, 'vert> = &'vbo RefCell<vbo::Vbo<'vert>>;
type EboRef<'ebo, 'ind> = &'ebo RefCell<ebo::Ebo<'ind>>;

pub mod vao {

    use super::*;

    //TODO:impl Drop
    pub struct Vao<'vbo, 'vert, 'ebo, 'ind> {
        handle: GLuint,
        vbo: Option<VboRef<'vbo, 'vert>>,
        ebo: Option<EboRef<'ebo, 'ind>>,
    }

    //The currently bound VAO
    pub struct BoundVao<'vao, 'vbo, 'vert, 'ebo, 'ind> {
        vao: &'vao mut Vao<'vbo, 'vert, 'ebo, 'ind>,
        ctx: Context,
    }

    impl<'vbo, 'vert, 'ebo, 'ind> Vao<'vbo, 'vert, 'ebo, 'ind> {
        pub fn new() -> GLResult<Vao<'vbo, 'vert, 'ebo, 'ind>> {
            let mut vao = Vao {
                handle: 0,
                vbo: None,
                ebo: None,
            };
            unsafe {
                gl::GenVertexArrays(1, &mut vao.handle);
            };
            get_error()?;
            Ok(vao)
        }
        pub unsafe fn raw(&self) -> u32 {
            self.handle
        }
    }

    impl<'vao, 'vbo, 'vert, 'ebo, 'ind> BoundVao<'vao, 'vbo, 'vert, 'ebo, 'ind> {
        pub fn new(
            vao: &'vao mut Vao<'vbo, 'vert, 'ebo, 'ind>,
            ctx: Context,
        ) -> BoundVao<'vao, 'vbo, 'vert, 'ebo, 'ind> {
            unsafe { gl::BindVertexArray(vao.handle) };
            get_error().unwrap();
            BoundVao { vao, ctx }
        }
        pub unsafe fn raw(&self) -> GLuint {
            unsafe { self.vao.raw() }
        }
        pub fn bind_vbo(&mut self, vbo: VboRef<'vbo, 'vert>) {
            self.vao.vbo.replace(vbo);
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo.borrow().raw());
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    6 * mem::size_of::<f32>() as i32,
                    ptr::null() as *const c_void,
                );
                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    6 * mem::size_of::<f32>() as i32,
                    (mem::size_of::<f32>() * 3) as *const c_void,
                );
                gl::EnableVertexArrayAttrib(self.raw(), 0);
                gl::EnableVertexArrayAttrib(self.raw(), 1);
            };
        }
        pub fn unbind_vbo(&mut self) {
            self.vao.vbo.take();
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
        }
        pub fn get_vbo(&self) -> Option<VboRef<'vbo, 'vert>> {
            self.vao.vbo
        }
        pub fn bind_ebo(&mut self, ebo: EboRef<'ebo, 'ind>) {
            self.vao.ebo.replace(ebo);
            unsafe {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo.borrow().raw());
            };
        }
        pub fn unbind_ebo(&mut self) {
            self.vao.ebo.take();
            unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
        }
        pub fn get_ebo(&self) -> Option<EboRef<'ebo, 'ind>> {
            self.vao.ebo
        }
        pub fn unbind(self) -> Context {
            unsafe { gl::BindVertexArray(0) };
            self.ctx
        }
        pub fn draw_triangles(&self) {
            assert!(self.vao.vbo.is_some());
            assert!(self.vao.vbo.unwrap().borrow().vertices().is_some());
            let verts = self.vao.vbo.unwrap().borrow().len().unwrap() / 3;
            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, verts as i32) }
        }
        pub fn draw_elements(&self) {
            let vbo = self.vao.vbo.unwrap().borrow();
            let ebo = self.vao.ebo.unwrap().borrow();
            let vertices = vbo.vertices().unwrap();
            let indices = ebo.indices().unwrap();
            assert!(ebo.max_index() as usize <= vertices.len() / (3 * 2));
            unsafe {
                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                )
            };
            get_error().unwrap()
        }
    }
}

pub mod vbo {

    use super::*;

    //TODO:impl Drop
    pub struct Vbo<'vert> {
        pub(crate) handle: GLuint,
        vertices: Option<&'vert [f32]>,
    }

    impl<'vert> Vbo<'vert> {
        pub fn new() -> GLResult<Vbo<'vert>> {
            let mut vbo = Vbo {
                handle: 0,
                vertices: None,
            };
            unsafe { gl::CreateBuffers(1, &mut vbo.handle) };
            get_error()?;
            Ok(vbo)
        }
        pub fn len(&self) -> Option<usize> {
            self.vertices.map(|s| s.len())
        }
        pub fn vertices(&self) -> Option<&'vert [f32]> {
            self.vertices
        }
        pub fn bind_data<'a>(&'a mut self, vertices: &'vert [f32]) {
            self.vertices.replace(vertices);
            unsafe {
                gl::NamedBufferData(
                    self.raw(),
                    (vertices.len() * mem::size_of::<f32>()) as isize,
                    vertices.as_ptr() as *const c_void,
                    gl::STATIC_DRAW,
                )
            };
            get_error().unwrap();
        }
        pub unsafe fn raw(&self) -> u32 {
            self.handle
        }
    }
}

pub mod ebo {

    use super::*;

    //TODO:impl Drop
    pub struct Ebo<'ind> {
        pub(crate) handle: GLuint,
        indices: Option<&'ind [u32]>,
        max_index: u32,
    }

    impl<'ind> Ebo<'ind> {
        pub fn new() -> GLResult<Ebo<'ind>> {
            let mut ebo = Ebo {
                handle: 0,
                indices: None,
                max_index: 0,
            };
            unsafe {
                gl::CreateBuffers(1, &mut ebo.handle);
            };
            get_error()?;
            Ok(ebo)
        }
        pub fn indices(&self) -> Option<&'ind [u32]> {
            self.indices
        }
        pub fn max_index(&self) -> u32 {
            self.max_index
        }
        pub fn bind_data<'a>(&'a mut self, indices: &'ind [u32]) {
            self.indices.replace(indices);
            self.max_index = *indices.iter().max().unwrap_or(&0);
            unsafe {
                gl::NamedBufferData(
                    self.raw(),
                    (indices.len() * mem::size_of::<u32>()) as isize,
                    indices.as_ptr() as *const c_void,
                    gl::STATIC_DRAW,
                )
            };
            get_error().unwrap();
        }
        pub unsafe fn raw(&self) -> u32 {
            self.handle
        }
    }
}

pub mod shader {
    use super::*;
    use std::io::Error as IOError;

    #[derive(Debug)]
    pub enum Error {
        Shader(String),
        Program(String),
        IO(IOError),
    }

    impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Error::Shader(error) => write!(f, "{}", error),
                Error::Program(error) => write!(f, "{}", error),
                Error::IO(error) => write!(f, "{}", error),
            }
        }
    }
    impl super::Error for Error {}

    impl Error {
        pub(self) fn shader_error(shader: GLuint) -> Error {
            let mut error_size = 0;
            unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut error_size) };
            get_error().unwrap();
            let mut buffer: Vec<u8> = vec![0; error_size as usize];
            unsafe {
                gl::GetShaderInfoLog(
                    shader,
                    error_size,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                )
            };
            get_error().unwrap();
            Error::Shader(String::from_utf8(buffer).unwrap())
        }
        pub(self) fn program_error(program: GLuint) -> Error {
            let mut error_size = 0;
            unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut error_size) };
            get_error().unwrap();
            let mut buffer: Vec<u8> = vec![0; error_size as usize];
            unsafe {
                gl::GetProgramInfoLog(
                    program,
                    error_size,
                    ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut i8,
                )
            };
            get_error().unwrap();
            Error::Program(String::from_utf8(buffer).unwrap())
        }
    }
    impl From<IOError> for Error {
        fn from(error: std::io::Error) -> Self {
            Self::IO(error)
        }
    }

    pub struct Shader(GLuint);

    //TODO:impl Drop
    impl Shader {
        //TODO shader type enum
        pub fn new(program: &CStr, shader_type: GLuint) -> Result<Shader, Error> {
            let shader = unsafe { gl::CreateShader(shader_type) };
            get_error().unwrap();
            let mut status = 0;
            let source_ptr: *const i8 = program.as_ptr() as *const i8;
            unsafe { gl::ShaderSource(shader, 1, &raw const source_ptr, ptr::null()) };
            get_error().unwrap();
            unsafe { gl::CompileShader(shader) };
            get_error().unwrap();
            unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status) };
            get_error().unwrap();
            if status != 1 {
                return Err(Error::shader_error(shader));
            }
            Ok(Shader(shader))
        }
        pub fn from_path(path: &str, shader_type: GLuint) -> Result<Shader, Error> {
            use std::ffi::CString;
            use std::fs::File;
            use std::io::Read;
            let mut file = File::open(path)?;
            let mut content: Vec<u8> = Vec::new();
            file.read_to_end(&mut content)?;
            let content = unsafe { CString::from_vec_unchecked(content) }; // if you put null bytes in your files: skill issue
            Self::new(&content, shader_type)
        }
        pub unsafe fn raw(&self) -> GLuint {
            self.0
        }
    }

    //TODO:impl Drop
    pub struct ShaderProgram(GLuint);
    impl ShaderProgram {
        //TODO: Maybe take an array of programs ?
        //Probably should enforce a vertex and fragment shader
        pub fn new(vertex: &Shader, fragment: &Shader) -> Result<ShaderProgram, Error> {
            let program = unsafe { gl::CreateProgram() };
            assert!(program != 0, " Couldn't create opengl program. Why idk");

            unsafe {
                gl::AttachShader(program, vertex.raw());
                get_error().unwrap();
                gl::AttachShader(program, fragment.raw());
                get_error().unwrap();
                gl::LinkProgram(program);
                get_error().unwrap();
            }
            let mut success = 0;
            unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut success) };
            get_error().unwrap();
            if success != 1 {
                return Err(Error::program_error(program));
            }
            Ok(ShaderProgram(program))
        }
        pub fn use_program(&self) {
            unsafe { gl::UseProgram(self.0) };
        }
        //TODO Put this on a BoundPrgram created with a ProgramContext
        pub unsafe fn set4f(&self, name : &CStr, x: f32, y: f32, z: f32, w: f32) -> Option<()> {
            let location = unsafe { gl::GetUniformLocation(self.raw(), name.as_ptr()) };
            get_error().unwrap();
            if location == -1 {
                return None;
            }
            unsafe { gl::Uniform4f(location, x, y, z, w) };
            get_error().unwrap();
            Some(())
        }
        pub unsafe fn raw(&self) -> GLuint {
            self.0
        }
    }
}
