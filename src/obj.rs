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
    InvalidParameter(usize),
    InvalidParameterNumber,
    InvalidLine,
}

impl Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::IOError(e) => write!(f, "{e}"),
            ErrorType::Unsupported(t) => write!(f, "Unssuported entry type : '{t}'"),
            ErrorType::InvalidEntry(t) => write!(f, "Invalid entry type : '{t}'"),
            ErrorType::IndexOutOfBound(i) => write!(f, "Index {i} is out of bound"),
            ErrorType::InvalidParameter(p) => write!(f, "Parameter #{p} is invalid"),
            ErrorType::InvalidParameterNumber => write!(f, "Invalid number of parameters"),
            ErrorType::InvalidLine => write!(f, "Invalid line"),
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    pub position: Vector3<f32>,
    pub color: Vector3<f32>,
    pub texture_coordinates: (f32, f32),
}

#[derive(Debug)]
pub struct Model {
    pub vertices: Box<[Vertex]>,
    pub indices: Box<[u32]>,
}

struct VertexData {
    position: Vector3<f32>,
    color: Option<Vector3<f32>>,
}

pub fn parse_obj(reader: impl BufRead) -> Result<Model, ParseError> {
    use ErrorType::*;
    let mut positions_color: Vec<VertexData> = Vec::new();
    let mut normals: Vec<Vector3<f32>> = Vec::new();
    let mut texture_coords: Vec<Vector3<f32>> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.map_err(|err| ParseError {
            line: None,
            line_no: index,
            err_type: ErrorType::IOError(err),
        })?;
        macro_rules! error {
            ($err:expr) => {
                ParseError {
                    line: Some(line.clone()),
                    line_no: index,
                    err_type: $err,
                }
            };
        }
        if line.starts_with("#") {
            continue;
        }
        if line.is_empty() {
            continue;
        }
        let (line_type, rest) = line.split_once(' ').ok_or(error!(InvalidLine))?;
        match line_type {
            "v" => {
                let args: Vec<_> = rest
                    .split_whitespace()
                    .map(|s| s.parse::<f32>())
                    .collect();

                if let Some(err) = args.iter().enumerate().find(|r| r.1.is_err()) {
                    return Err(error!(InvalidParameter(err.0)));
                }
                let mut iter = args.iter().map(|r| *r.as_ref().unwrap());
                match args.len() {
                    3 => positions_color.push(VertexData {
                        position: Vector3::from_iterator(&mut iter),
                        color: None,
                    }), // x y z
                    4 => {
                        // x y z w
                        positions_color.push(VertexData {
                            position: Vector3::from_iterator(&mut iter),
                            color: None,
                        }); // x y z w
                        eprintln!("{index}: w component ignored");
                    }
                    6 => positions_color.push(VertexData {
                        position: Vector3::from_iterator(&mut iter),
                        color: Some(Vector3::from_iterator(&mut iter)),
                    }), // x y z r g b
                    _ => return Err(error!(InvalidParameterNumber)),
                }
            }
            "vt" => (), // vt u [v, w]
            "vn" => (), //vn x y z  (may not be unit)
            "f" => {
                let args: Vec<_> = rest
                    .split_whitespace()
                    .map(|s| s.parse::<u32>())
                    .collect();

                if let Some(err) = args.iter().enumerate().find(|r| match r.1 {
                    Err(_) => true,
                    Ok(n) => *n == 0,
                }) {
                    return Err(error!(InvalidParameter(err.0)));
                }

                if args.len() != 3 {
                    return Err(error!(InvalidParameterNumber));
                }//maybe support polygonal faces
                indices.extend(args.iter().map(|r| *r.as_ref().unwrap()-1))
            }
            ,  // f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3
            "g" | "o" | "mtllib" | "usemtl" => {
                eprintln!("{index}:Warning {line_type} is not implemented")
            }
            "p" | "l" | "curv" | "curv2D" | "surf" | "s" | "mg" | "parm" | "trim" | "hole"
            | "scrv" | "sp" | "end" | "con" => return Err(error!(Unsupported(line_type.into()))),
            _ => return Err(error!(InvalidEntry(line_type.into()))),
        }
    }
    let vertices: Vec<_> = positions_color
        .iter()
        .map(|p_c| Vertex {
            position: p_c.position,
            color: Vector3::zero(),
            texture_coordinates: (0.0, 0.0),
        })
        .collect();
    Ok(Model {
        vertices: vertices.into(),
        indices: indices.into(),
    })
}
