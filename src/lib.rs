use gl::types::*;
use std::mem;
use std::error::Error;
use std::fmt::{self, Display};



pub fn make_contexts() -> (vao::Context, vbo::Context) {
    (
        vao::Context::new(),
        vbo::Context::new(),
    )
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

pub mod vao {

    use super::*;

    //Empty struct that needs to be borrwed to bind a vao
    //Allows the borrow checker to detect lifetime issues with vao binding
    pub struct Context;

    impl Context {
        pub(crate) fn new() -> Context { Context }
    }

    #[repr(transparent)]
    pub struct Vao(GLuint);

    //The currently bound VAO
    pub struct BoundVao<'a> {
        vao : &'a Vao,
        ctx : Context,
    }

    impl Vao {
        pub fn new() -> GLResult<Vao> {
            let mut vao  = Vao (0);
            unsafe { gl::GenVertexArrays(1, &mut vao.0); };
            get_error()?;
            Ok(vao)
        }
        pub unsafe fn raw(&self) -> u32 {
            self.0
        }
    }

    impl<'a> BoundVao<'a> {
        pub fn new(vao : &'a Vao, ctx : Context) -> BoundVao<'a> {
            unsafe { gl::BindVertexArray(vao.0)};
            get_error().unwrap();
            BoundVao{
                vao,
                ctx,
            }
        }
        pub unsafe fn raw(&self) -> u32 {
            self.vao.0
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
    pub struct Context;

    impl Context {
        pub(crate) fn new() -> Context { Context }
    }

    #[repr(transparent)]
    pub struct Vbo(GLuint);

    //The currently bound VBO
    pub struct BoundVbo<'a> {
        vbo: &'a Vbo,
        ctx: Context,
    }

    impl Vbo {
        pub fn new() -> GLResult<Vbo> {
            let mut vbo  = Vbo (0);
            unsafe { gl::GenBuffers(1, &mut vbo.0); };
            get_error()?;
            Ok(vbo)
        }
        pub unsafe fn raw(&self) -> u32 {
            self.0
        }
    }

    impl<'a> BoundVbo<'a> {
        pub fn new(vbo : &'a Vbo, ctx : Context) -> BoundVbo<'a> {
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo.0)};
            get_error().unwrap();
            BoundVbo{
                vbo,
                ctx,
            }
        }
        pub unsafe fn raw(&self) -> u32 {
            self.vbo.0
        }
        pub fn unbind(self) -> Context {
            unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0)};
            self.ctx
        }
    }
}
