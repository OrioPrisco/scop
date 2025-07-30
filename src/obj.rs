use super::math::vector::Vector3;
use std::fmt::{self, Display};
use std::io::{BufRead, Error as IOError};

//Struct with line info + Error type ?

#[derive(Debug)]
pub struct ParseError {
    pub line: Option<String>,
    pub line_no: usize,
    pub err_type: ErrorType,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.line {
            Some(line) => write!(f, "{}:{} : {}", self.line_no, line, self.err_type),
            None => write!(f, "{}: {}", self.line_no, self.err_type),
        }
    }
}
#[derive(Debug)]
pub enum ErrorType {
    IOError(IOError),
    Unsupported(String),
    InvalidEntry(String),
    IndexOutOfBound(isize),
    InvalidLine,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::IOError(e) => write!(f, "{e}"),
            ErrorType::Unsupported(t) => write!(f, "Unssuported entry type : '{t}'"),
            ErrorType::InvalidEntry(t) => write!(f, "Invalid entry type : '{t}'"),
            ErrorType::IndexOutOfBound(i) => write!(f, "Index {i} is out of bound"),
            ErrorType::InvalidLine => write!(f, "Invalid line"),
        }
    }
}

pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub texture_coordinates: (f32, f32),
}

pub struct Model {
    pub vertices: Box<[Vertex]>,
    pub indices: Box<[u32]>,
}

struct VertexData {
    position: Vector3<f32>,
    color: Option<Vector3<f32>>,
}

pub fn parse_obj(reader: impl BufRead) -> Result<Model, ParseError> {
    let positions_color: Vec<VertexData> = Vec::new();
    let normals: Vec<Vector3<f32>> = Vec::new();
    let texture_coords: Vec<Vector3<f32>> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.map_err(|err| ParseError {
            line: None,
            line_no: index,
            err_type: ErrorType::IOError(err),
        })?;
        macro_rules! error {
            ($err:tt) => {
                ParseError {
                    line: Some(line.clone()),
                    line_no: index,
                    err_type: ErrorType::$err,
                }
            };
        }
        let line_type = line.split_once(' ').ok_or(error!(InvalidLine))?;
        match line_type.0 {
            "v" => (),
            "vt" => (),
            "vn" => (),
            "f" => (),
            _ => (),
        }
    }

    todo!();
}
