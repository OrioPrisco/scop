use gl::types::*;
use std::mem;
use std::ptr;
use std::error::Error;
use std::fmt::{self, Display};
use std::ffi::c_void;
use std::ffi::CStr;
use std::cell::RefCell;


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
    let err : GLenum = unsafe {gl::GetError()};
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

pub mod vao {

    use super::*;
    use vbo::{self, InternalBoundVbo};

    pub struct Vao<'vbo, 'vert> {
        handle : GLuint,
        vbo : Option<VboRef<'vbo, 'vert>>,
    }

    //The currently bound VAO
    pub struct BoundVao<'vao,'vbo, 'vert> {
        vao : &'vao mut Vao<'vbo, 'vert>,
        ctx : Context,
    }

    impl<'vbo, 'vert> Vao<'vbo, 'vert> {
        pub fn new() -> GLResult<Vao<'vbo, 'vert>> {
            let mut vao  = Vao {handle: 0, vbo: None};
            unsafe { gl::GenVertexArrays(1, &mut vao.handle); };
            get_error()?;
            Ok(vao)
        }
        pub unsafe fn raw(&self) -> u32 {
            self.handle
        }
    }

    impl<'vao,'vbo, 'vert> BoundVao<'vao,'vbo, 'vert> {
        pub fn new(vao : &'vao mut Vao<'vbo, 'vert>, ctx : Context) -> BoundVao<'vao,'vbo, 'vert> {
            unsafe { gl::BindVertexArray(vao.handle)};
            get_error().unwrap();
            BoundVao{
                vao,
                ctx,
            }
        }
        pub unsafe fn raw(&self) -> GLuint {
            unsafe { self.vao.raw() }
        }
        pub fn bind_vbo(&mut self, vbo : VboRef<'vbo, 'vert>) {
            self.vao.vbo.replace(vbo);
            unsafe {
                gl::BindBuffer(gl::ARRAY_BUFFER, vbo.borrow().raw());
                gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<f32>() as i32, ptr::null() as *const c_void);
                gl::EnableVertexAttribArray(0);
            };
        }
        pub fn unbind_vbo(&mut self) {
            self.vao.vbo.take();
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0)};
        }
        pub fn get_bind(&self) -> Option<InternalBoundVbo<'vbo, 'vert>> {
            self.vao.vbo.map(|vbo| InternalBoundVbo::new(vbo))
        }
        pub fn unbind(self) -> Context {
            unsafe {gl::BindVertexArray(0)};
            self.ctx
        }
    }
}

pub mod vbo {

    use super::*;

    //Empty struct that needs to be borrwed to bind a vbo
    //Allows the borrow checker to detect lifetime issues with vbo binding

    pub struct Vbo<'vert> {
        pub(crate) handle : GLuint,
        vertices : Option<&'vert[f32]>,
    }

    impl<'vert> Vbo<'vert> {
        pub fn new() -> GLResult<Vbo<'vert>> {
            let mut vbo  = Vbo {
                handle: 0,
                vertices: None,
            };
            unsafe { gl::GenBuffers(1, &mut vbo.handle); };
            get_error()?;
            Ok(vbo)
        }
        pub unsafe fn raw(&self) -> u32 {
            self.handle
        }
    }

    pub struct InternalBoundVbo<'vbo, 'vert> {
        pub(crate) vbo: VboRef<'vbo, 'vert>,
    }

    impl<'vbo, 'vert> InternalBoundVbo<'vbo, 'vert> {
        pub(crate) fn new(vbo : VboRef<'vbo, 'vert>) -> InternalBoundVbo<'vbo, 'vert> {
            InternalBoundVbo{vbo}
        }
        pub unsafe fn raw(&self) -> GLuint {
            unsafe { self.vbo.borrow().raw() }
        }
        pub fn bind_data<'a>(&'a mut self, vertices : &'vert[f32]) {
            self.vbo.borrow_mut().vertices.replace(vertices);
            unsafe { gl::NamedBufferData(
                self.vbo.borrow().handle,
                (vertices.len() * mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            )};
            get_error().unwrap();
        }
    }

    //The currently bound VBO
    pub struct BoundVbo<'vbo, 'vert> {
        vbo : InternalBoundVbo<'vbo, 'vert>,
        ctx: Context,
    }

    impl<'vbo, 'vert> BoundVbo<'vbo, 'vert> {
        pub fn new(vbo : VboRef<'vbo, 'vert>, ctx : Context) -> BoundVbo<'vbo, 'vert> {
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo.borrow().raw())};
            get_error().unwrap();
            BoundVbo{
                vbo: InternalBoundVbo::new(vbo),
                ctx,
            }
        }
        pub fn unbind(self) -> Context {
            unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0)};
            self.ctx
        }
    }
    impl<'vbo,'vert> std::ops::Deref for BoundVbo<'vbo,'vert> {
        type Target = InternalBoundVbo<'vbo,'vert>;

       fn deref(&self) -> &Self::Target { &self.vbo }
    }
    impl<'vbo,'vert> std::ops::DerefMut for BoundVbo<'vbo,'vert> {
       fn deref_mut(&mut self) -> &mut Self::Target { &mut self.vbo }
    }
}

pub mod shader {
    use super::*;

    #[derive(Debug)]
    pub struct Error {
        error: String,
    }

    impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}",  self.error)
        }
    }
    impl super::Error for Error {}

    impl Error {
        pub(self) fn get(shader : GLuint) -> Error {
            let mut error_size = 0;
            unsafe {gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut error_size)};
            get_error().unwrap();
            let mut buffer : Vec<u8> = vec![0; error_size as usize];
            unsafe {gl::GetShaderInfoLog(shader, error_size, ptr::null_mut(), buffer.as_mut_ptr() as *mut i8)};
            get_error().unwrap();
            Error{error : String::from_utf8(buffer).unwrap()}
        }
    }

    pub struct Shader(GLuint);

    impl Shader {
        //TODO shader type enum
        pub fn new(program : &CStr, shader_type : GLuint) -> Result<Shader, Error>{
            let shader = unsafe {gl::CreateShader(shader_type)};
            get_error().unwrap();
            let mut status = 0;
            let source_ptr : *const i8 = program.as_ptr() as *const i8;
            unsafe{ gl::ShaderSource(shader, 1, &raw const source_ptr, ptr::null())};
            get_error().unwrap();
            unsafe {gl::CompileShader(shader)};
            get_error().unwrap();
            unsafe {gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status)};
            get_error().unwrap();
            if status != 1 {
                return Err(Error::get(shader));
            }
            Ok(Shader(shader))
        }
        pub unsafe fn raw(&self) -> GLuint { self.0 }
    }
}
