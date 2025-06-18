use gl::types::*;
use std::mem;
use std::error::Error;
use std::fmt::{self, Display};


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

pub mod vao {

    use super::*;
    use vbo::{self, Vbo, InternalBoundVbo};

    pub struct Vao<'a> {
        handle : GLuint,
        vbo : Option<&'a Vbo>,
    }

    //The currently bound VAO
    pub struct BoundVao<'a,'b> {
        vao : &'a mut Vao<'b>,
        ctx : Context,
    }

    impl<'a> Vao<'a> {
        pub fn new() -> GLResult<Vao<'a>> {
            let mut vao  = Vao {handle: 0, vbo: None};
            unsafe { gl::GenVertexArrays(1, &mut vao.handle); };
            get_error()?;
            Ok(vao)
        }
        pub unsafe fn raw(&self) -> u32 {
            self.handle
        }
    }

    impl<'a,'b> BoundVao<'a,'b> {
        pub fn new(vao : &'a mut Vao<'b>, ctx : Context) -> BoundVao<'a,'b> {
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
        pub fn bind_vbo(&mut self, vbo : &'b Vbo) {
            self.vao.vbo.replace(vbo);
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo.0)};
        }
        pub fn unbind_vbo(&mut self) {
            self.vao.vbo.take();
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, 0)};
        }
        pub fn get_bind(&self) -> Option<InternalBoundVbo<'a>> {
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

    #[repr(transparent)]
    pub struct Vbo(pub(crate) GLuint);

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

    pub struct InternalBoundVbo<'a> {
        pub(crate) vbo: &'a Vbo,
    }

    impl<'a> InternalBoundVbo<'a> {
        pub(crate) fn new(vbo : &'a Vbo) -> InternalBoundVbo<'a> {
            InternalBoundVbo{vbo}
        }
        pub unsafe fn raw(&self) -> GLuint {
            unsafe { self.vbo.raw() }
        }
    }

    //The currently bound VBO
    pub struct BoundVbo<'a> {
        vbo : InternalBoundVbo<'a>,
        ctx: Context,
    }

    impl<'a> BoundVbo<'a> {
        pub fn new(vbo : &'a Vbo, ctx : Context) -> BoundVbo<'a> {
            unsafe { gl::BindBuffer(gl::ARRAY_BUFFER, vbo.0)};
            get_error().unwrap();
            BoundVbo{
                vbo: InternalBoundVbo::new(vbo),
                ctx,
            }
        }
        pub unsafe fn raw(&self) -> u32 {
            unsafe { self.vbo.raw() }
        }
        pub fn unbind(self) -> Context {
            unsafe {gl::BindBuffer(gl::ARRAY_BUFFER, 0)};
            self.ctx
        }
    }
}
