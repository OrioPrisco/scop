use super::math::vector::{Vector2, Vector3};
use std::collections::HashMap;
use std::error::Error;
use std::f32::consts::PI;
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

impl Error for ParseError {}

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

/// returns the 0 based index into an array from a 1 based index
/// or a negative index from the end of the list
fn get_index(array_len: usize, index: isize) -> Result<u32, ErrorType> {
    let array_len: u32 = array_len
        .try_into()
        .map_err(|_| ErrorType::IndexOutOfBound(index))?;
    if index > 0 {
        let index = index as u32;
        if index > array_len {
            return Err(ErrorType::IndexOutOfBound(index as isize));
        }
        return Ok(index - 1);
    } else if index < 0 {
        let from_end: u32 = (-index).try_into().unwrap();
        return array_len
            .checked_sub(from_end)
            .ok_or(ErrorType::IndexOutOfBound(index));
    }
    Err(ErrorType::IndexOutOfBound(index))
}

/// Triangulates a polygonal face by using the fan method
/// Fast and easy but might fail on Concave shapes
fn fan_triangulation(indices: Vec<FaceInfo>) -> Vec<FaceInfo> {
    let mut ret: Vec<FaceInfo> = Vec::new();
    let (first, rest) = indices.as_slice().split_first().unwrap();
    for indices in rest.windows(2) {
        ret.push(*first);
        ret.push(indices[0]);
        ret.push(indices[1]);
    }
    ret
}

#[derive(Debug, Clone, Copy)]
struct FaceInfoRaw {
    pub vertex: isize,
    pub texture: Option<isize>,
    pub normal: Option<isize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FaceInfo {
    pub vertex: u32,
    pub texture: Option<u32>,
    pub normal: Option<u32>,
}

impl FaceInfoRaw {
    pub fn same_shape(&self, other: &Self) -> bool {
        (self.texture.is_some() == other.texture.is_some())
            && (self.normal.is_some() == other.normal.is_some())
    }
    pub fn parse(s: &str) -> Option<Self> {
        let mut nums = s.split("/");
        let vertex = nums.next()?.parse::<isize>().ok()?;
        let texture = match nums.next() {
            None => None,
            Some("") => None,
            Some(number) => Some(number.parse::<isize>().ok()?),
        };
        let normal = match nums.next() {
            None => None,
            Some("") => None,
            Some(number) => Some(number.parse::<isize>().ok()?),
        };
        Some(Self {
            vertex,
            texture,
            normal,
        })
    }
    pub fn get_indices(
        self,
        vertices_len: usize,
        textures_size: usize,
        normals_size: usize,
    ) -> Result<FaceInfo, ErrorType> {
        Ok(FaceInfo {
            vertex: get_index(vertices_len, self.vertex)?,
            texture: self
                .texture
                .map(|i| get_index(textures_size, i))
                .transpose()?,
            normal: self
                .normal
                .map(|i| get_index(normals_size, i))
                .transpose()?,
        })
    }
}

pub fn parse_obj(reader: impl BufRead) -> Result<Model, ParseError> {
    use ErrorType::*;
    let mut positions_color: Vec<VertexData> = Vec::new();
    let mut normals: Vec<Vector3<f32>> = Vec::new();
    let mut texture_coords: Vec<(f32, f32)> = Vec::new();
    let mut indices: Vec<FaceInfo> = Vec::new();

    //file parsing
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
                let args : Vec<_> = rest
                    .split_whitespace()
                    .enumerate().map (
                    |(i,s)|
                        FaceInfoRaw::parse(s)
                        .ok_or(error!(InvalidParameter(i)))
                    ).collect::<Result<_,_>>()?;
                if let Some(bad) = args.iter().enumerate().find(|(_i,f)| !f.same_shape(&args[0])) {
                    return Err(error!(InvalidParameter(bad.0)));
                }

                let mut args: Vec<_> = args.iter().map(|f|f.get_indices(positions_color.len(), texture_coords.len(), normals.len()))
                .collect::<Result<_,_>>().map_err(|e| error!(e))?;
                if args.len() < 3 {
                    return Err(error!(InvalidParameterNumber));
                }
                if args.len() > 3 {
                    args = fan_triangulation(args);
                }
                indices.extend(args);
            }
            ,  // f v1/vt1/vn1 v2/vt2/vn2 v3/vt3/vn3
            "g" | "o" | "mtllib" | "usemtl" => {
                eprintln!("{index}:Warning {line_type} is not implemented")
            }
            "s" => {
                let args: Vec<_> = rest.split_whitespace().collect();
                if args.len() > 1 {
                    return Err(error!(InvalidParameterNumber));
                }
                match args[0] {
                    "off" => (),
                    "on" => return Err(error!(Unsupported("s on".into()))),
                    _ => return Err(error!(InvalidEntry(line))),
                }

            }
            "p" | "l" | "curv" | "curv2D" | "surf" | "mg" | "parm" | "trim" | "hole"
            | "scrv" | "sp" | "end" | "con" => return Err(error!(Unsupported(line_type.into()))),
            _ => return Err(error!(InvalidEntry(line_type.into()))),
        }
    }
    //normalization
    let min_coord = positions_color
        .iter()
        .map(|p_c| p_c.position)
        .reduce(|a, b| Vector3 {
            x: a.x.min(b.x),
            y: a.y.min(b.y),
            z: a.z.min(b.z),
        })
        .unwrap_or(Vector3::zero());
    let max_coord = positions_color
        .iter()
        .map(|p_c| p_c.position)
        .reduce(|a, b| Vector3 {
            x: a.x.max(b.x),
            y: a.y.max(b.y),
            z: a.z.max(b.z),
        })
        .unwrap_or(Vector3::zero());
    let middle_coord = (min_coord + max_coord) / 2.0;
    let mid_2d = Vector2 {
        x: middle_coord.x,
        y: middle_coord.z,
    };
    let mut verts_index: HashMap<FaceInfo, u32> = HashMap::with_capacity(indices.len());
    let mut fixed_indices: Vec<u32> = Vec::with_capacity(indices.len());
    let mut fixed_verts: Vec<Vertex> = Vec::with_capacity(positions_color.len());
    for indices in indices {
        if let Some(vert_index) = verts_index.get(&indices) {
            fixed_indices.push(*vert_index);
        } else {
            verts_index.insert(indices, fixed_verts.len() as u32);
            fixed_indices.push(fixed_verts.len() as u32);
            let pos_index = indices.vertex;
            let text_index = indices.texture;
            let pos_color = &positions_color[pos_index as usize];
            let position = pos_color.position;
            let pos_2d = Vector2 {
                x: position.x,
                y: position.z,
            };
            let k = Vector2 { x: 0.0, y: 1.0 };

            let angle = (pos_2d.dot(k) / pos_2d.norm()).acos();
            let distance_2d = (pos_2d - mid_2d).norm();
            fixed_verts.push(Vertex {
                position: position - middle_coord,
                color: pos_color.color.unwrap_or(Vector3::zero()),
                texture_coordinates: text_index.map(|i| texture_coords[i as usize]).unwrap_or(
                    if texture_coords.is_empty() {
                        (angle + distance_2d, position.y - min_coord.y)
                    } else {
                        (0.0, 0.0)
                    },
                ),
            });
        }
    }
    Ok(Model {
        vertices: fixed_verts.into(),
        indices: fixed_indices.into(),
    })
}
