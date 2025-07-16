use gl::types::*;
use std::cell::RefCell;
use std::error::Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::c_void;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::Error as IOError;
use std::io::Read;
use std::mem;
use std::ptr;

pub mod math;

pub struct Context;

//Empty struct that needs to be borrwed to bind a vao/vbo
//Allows the borrow checker to detect lifetime issues with vao binding
impl Context {
    pub fn new() -> Context {
        Context
    }
}

const ELEMS_PER_VERTEX: usize = 3 + 3 + 2; //pos color texture

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

type VboRef<'vbo> = &'vbo RefCell<vbo::Vbo>;
type EboRef<'ebo> = &'ebo RefCell<ebo::Ebo>;

pub mod vao {

    use super::*;

    //TODO:impl Drop
    pub struct Vao<'vbo, 'ebo> {
        handle: GLuint,
        vbo: Option<VboRef<'vbo>>,
        ebo: Option<EboRef<'ebo>>,
    }

    //The currently bound VAO
    pub struct BoundVao<'vao, 'vbo, 'ebo> {
        vao: &'vao mut Vao<'vbo, 'ebo>,
        ctx: Context,
    }

    impl<'vbo, 'ebo> Vao<'vbo, 'ebo> {
        pub fn new() -> GLResult<Vao<'vbo, 'ebo>> {
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

    impl<'vao, 'vbo, 'ebo> BoundVao<'vao, 'vbo, 'ebo> {
        pub fn new(vao: &'vao mut Vao<'vbo, 'ebo>, ctx: Context) -> BoundVao<'vao, 'vbo, 'ebo> {
            unsafe { gl::BindVertexArray(vao.handle) };
            get_error().unwrap();
            BoundVao { vao, ctx }
        }
        pub unsafe fn raw(&self) -> GLuint {
            unsafe { self.vao.raw() }
        }
        pub fn bind_vbo(&mut self, vbo: VboRef<'vbo>) {
            self.vao.vbo.replace(vbo);
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo.borrow().raw());
                gl::VertexAttribPointer(
                    0,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (ELEMS_PER_VERTEX * mem::size_of::<f32>()) as i32,
                    ptr::null() as *const c_void,
                );
                gl::VertexAttribPointer(
                    1,
                    3,
                    gl::FLOAT,
                    gl::FALSE,
                    (ELEMS_PER_VERTEX * mem::size_of::<f32>()) as i32,
                    (mem::size_of::<f32>() * 3) as *const c_void,
                );
                gl::VertexAttribPointer(
                    2,
                    2,
                    gl::FLOAT,
                    gl::FALSE,
                    (ELEMS_PER_VERTEX * mem::size_of::<f32>()) as i32,
                    (mem::size_of::<f32>() * 6) as *const c_void,
                );
                gl::EnableVertexArrayAttrib(self.raw(), 0);
                gl::EnableVertexArrayAttrib(self.raw(), 1);
                gl::EnableVertexArrayAttrib(self.raw(), 2);
            };
        }
        pub fn unbind_vbo(&mut self) {
            self.vao.vbo.take();
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0) };
        }
        pub fn get_vbo(&self) -> Option<VboRef<'vbo>> {
            self.vao.vbo
        }
        pub fn bind_ebo(&mut self, ebo: EboRef<'ebo>) {
            self.vao.ebo.replace(ebo);
            unsafe {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo.borrow().raw());
            };
        }
        pub fn unbind_ebo(&mut self) {
            self.vao.ebo.take();
            unsafe { gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0) };
        }
        pub fn get_ebo(&self) -> Option<EboRef<'ebo>> {
            self.vao.ebo
        }
        pub fn unbind(self) -> Context {
            unsafe { gl::BindVertexArray(0) };
            self.ctx
        }
        pub fn draw_triangles(&self) {
            assert!(self.vao.vbo.is_some());
            assert!(self.vao.vbo.unwrap().borrow().len().is_some());
            let verts = self.vao.vbo.unwrap().borrow().len().unwrap() / ELEMS_PER_VERTEX;
            unsafe { gl::DrawArrays(gl::TRIANGLES, 0, verts as i32) }
        }
        pub fn draw_elements(&self) {
            let vbo = self.vao.vbo.unwrap().borrow();
            let ebo = self.vao.ebo.unwrap().borrow();
            let vertices = vbo.len().unwrap();
            let indices = ebo.length();
            assert!(ebo.max_index() as usize <= vertices / (ELEMS_PER_VERTEX));
            unsafe {
                gl::DrawElements(gl::TRIANGLES, indices as i32, gl::UNSIGNED_INT, ptr::null())
            };
            get_error().unwrap()
        }
    }
}

pub mod vbo {

    use super::*;

    //TODO:impl Drop
    pub struct Vbo {
        pub(crate) handle: GLuint,
        vertices_len: Option<usize>,
    }

    impl Vbo {
        pub fn new() -> GLResult<Vbo> {
            let mut vbo = Vbo {
                handle: 0,
                vertices_len: None,
            };
            unsafe { gl::CreateBuffers(1, &mut vbo.handle) };
            get_error()?;
            Ok(vbo)
        }
        pub fn len(&self) -> Option<usize> {
            self.vertices_len
        }
        pub fn bind_data(&mut self, vertices: &[f32]) {
            self.vertices_len.replace(vertices.len());
            unsafe {
                gl::NamedBufferData(
                    self.raw(),
                    mem::size_of_val(vertices) as isize,
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
    pub struct Ebo {
        pub(crate) handle: GLuint,
        max_index: u32,
        length: usize,
    }

    impl Ebo {
        pub fn new() -> GLResult<Ebo> {
            let mut ebo = Ebo {
                handle: 0,
                max_index: 0,
                length: 0,
            };
            unsafe {
                gl::CreateBuffers(1, &mut ebo.handle);
            };
            get_error()?;
            Ok(ebo)
        }
        pub fn max_index(&self) -> u32 {
            self.max_index
        }
        pub fn length(&self) -> usize {
            self.length
        }
        pub fn bind_data(&mut self, indices: &[u32]) {
            self.max_index = *indices.iter().max().unwrap_or(&0);
            self.length = indices.len();
            unsafe {
                gl::NamedBufferData(
                    self.raw(),
                    std::mem::size_of_val(indices) as isize,
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
            let source_ptr: *const i8 = program.as_ptr();
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
    use math::matrix::Mat4;
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
        pub unsafe fn set4f(&self, name: &CStr, x: f32, y: f32, z: f32, w: f32) -> Option<()> {
            let location = unsafe { gl::GetUniformLocation(self.raw(), name.as_ptr()) };
            get_error().unwrap();
            if location == -1 {
                return None;
            }
            unsafe { gl::Uniform4f(location, x, y, z, w) };
            get_error().unwrap();
            Some(())
        }
        pub unsafe fn set1i(&self, name: &CStr, int: GLint) -> Option<()> {
            let location = unsafe { gl::GetUniformLocation(self.raw(), name.as_ptr()) };
            get_error().unwrap();
            if location == -1 {
                return None;
            }
            unsafe { gl::Uniform1i(location, int) };
            get_error().unwrap();
            Some(())
        }
        pub unsafe fn set_mat(&self, name: &CStr, mat: &Mat4<f32>) -> Option<()> {
            let location = unsafe { gl::GetUniformLocation(self.raw(), name.as_ptr()) };
            get_error().unwrap();
            if location == -1 {
                return None;
            }
            unsafe {
                gl::UniformMatrix4fv(
                    location,
                    1,
                    gl::TRUE,
                    (&mat.components[0][0]) as *const f32,
                )
            };
            get_error().unwrap();
            Some(())
        }
        //TODO: ensure texture outlives Program
        pub unsafe fn set_texture(
            &self,
            name: &CStr,
            texture: &texture::BoundTexture,
        ) -> Option<()> {
            unsafe { self.set1i(name, texture.context.number as i32) }
        }
        pub unsafe fn raw(&self) -> GLuint {
            self.0
        }
    }
}

pub mod texture {
    use super::*;

    #[derive(Debug)]
    pub struct Context {
        pub(crate) number: GLuint,
    }
    pub struct ActiveContext {
        current: GLuint,
    }
    //TODO : impl Drop
    //Maybe use a bool to know wether it's initialized ?
    pub struct Texture {
        handle: GLuint,
    }
    pub struct BoundTexture<'ctx, 'tex> {
        pub(crate) context: &'ctx mut Context,
        _texture: &'tex Texture,
    }

    pub fn get_contexts() -> Vec<Context> {
        //OpenGL guarantees at least 16 different active textures
        //Could return the actual number of Context by querying openGL
        (0..15).map(|i| Context { number: i }).collect()
    }
    pub fn get_active_context() -> ActiveContext {
        unsafe { gl::ActiveTexture(gl::TEXTURE0) };
        ActiveContext { current: 0 }
    }
    impl ActiveContext {
        pub fn switch_to(&mut self, number: GLuint) {
            if number == self.current {
                return;
            }
            self.current = number;
            unsafe { gl::ActiveTexture(gl::TEXTURE0 + number) };
            get_error().unwrap();
        }
    }

    impl Texture {
        pub fn new() -> Self {
            let mut handle: GLuint = 0;
            unsafe { gl::CreateTextures(gl::TEXTURE_2D, 1, &mut handle) };
            get_error().unwrap();
            Texture { handle }
        }
        pub fn bind<'tex, 'ctx>(
            &'tex self,
            context: &'ctx mut Context,
            active_context: &mut ActiveContext,
        ) -> BoundTexture<'ctx, 'tex> {
            active_context.switch_to(context.number);
            unsafe { gl::BindTexture(gl::TEXTURE_2D, self.handle) };
            BoundTexture {
                _texture: self,
                context,
            }
        }
    }
    impl<'ctx, 'tex> BoundTexture<'ctx, 'tex> {
        pub fn bind_data(&self, image: &image::RgbaImage, active_context: &mut ActiveContext) {
            active_context.switch_to(self.context.number);

            unsafe {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);

                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
                let dim = image.dimensions();
                let pixels = image.as_raw();
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
            }
            get_error().unwrap();
        }
        pub fn bind_data_from_path(
            &self,
            path: &str,
            active_context: &mut ActiveContext,
        ) -> image::error::ImageResult<()> {
            let img = image::ImageReader::open(path)?.decode()?;
            let img = match img {
                image::DynamicImage::ImageRgba8(img) => img,
                image => image.to_rgba8(),
            };
            self.bind_data(&img, active_context);
            Ok(())
        }
    }
}
