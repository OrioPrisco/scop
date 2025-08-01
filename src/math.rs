use std::mem;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[macro_use]
mod helper_macros;

pub trait Sqrt {
    fn sqrt(self) -> Self;
}
impl_float_trait!(Sqrt, sqrt, f64 f32);
impl_asf64_trait!(Sqrt, sqrt, i32 i64);

pub trait Cos {
    fn cos(self) -> Self;
}
impl_float_trait!(Cos, cos, f64 f32);

pub trait Sin {
    fn sin(self) -> Self;
}
impl_float_trait!(Sin, sin, f64 f32);
pub trait Tan {
    fn tan(self) -> Self;
}
impl_float_trait!(Tan, tan, f64 f32);
pub trait ToRadians {
    fn to_radians(self) -> Self;
}
impl_float_trait!(ToRadians, to_radians, f64 f32);

pub trait NumberLike:
    Mul<Self, Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Sub<Self, Output = Self>
    + Neg<Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
    + Copy
    + From<i8>
{
}

impl<T> NumberLike for T where
    T: Mul<Self, Output = Self>
        + Add<Self, Output = Self>
        + AddAssign<Self>
        + Sub<Self, Output = Self>
        + Neg<Output = Self>
        + Mul<Self, Output = Self>
        + Div<Self, Output = Self>
        + Copy
        + From<i8>
{
}

pub mod matrix {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct Mat4<T: NumberLike> {
        pub components: [[T; 4]; 4],
    }
    use super::vector::Vector3;
    impl<T: NumberLike> Mat4<T> {
        pub fn empty() -> Self {
            Mat4 {
                components: [[0.into(); 4]; 4],
            }
        }
        pub fn identity() -> Self {
            let mut ret = Mat4 {
                components: [[0.into(); 4]; 4],
            };
            ret.components[0][0] = 1.into();
            ret.components[1][1] = 1.into();
            ret.components[2][2] = 1.into();
            ret.components[3][3] = 1.into();
            ret
        }
        pub fn translate(vec: &Vector3<T>) -> Self {
            let mut ret = Self::identity();
            for i in 0..3 {
                ret[i][3] = vec[i];
            }
            ret
        }
        pub fn scale(vec: &Vector3<T>) -> Self {
            let mut ret = Self::identity();
            for i in 0..3 {
                ret[i][i] = vec[i];
            }
            ret
        }
        pub fn transpose(mut self) -> Self {
            for y in 0..4 {
                let comps = self.components.as_mut_slice();
                let (head, tail) = comps.split_at_mut(y + 1);
                for x in y + 1..4 {
                    mem::swap(&mut head[y][x], &mut tail[x - y - 1][y]);
                }
            }
            self
        }
    }
    impl<T: NumberLike + Cos + Sin> Mat4<T> {
        pub fn rotate(vec: &Vector3<T>, angle: T) -> Self {
            let mut ret = Self::identity();
            let cos_theta = angle.cos();
            let sin_theta = angle.sin();
            for y in 0..3 {
                for x in 0..3 {
                    ret[y][x] = (<i8 as Into<T>>::into(1) - cos_theta) * vec[y] * vec[x];
                    if x == y {
                        ret[y][x] += cos_theta;
                    }
                }
            }
            //There's probably a more clever way of doing this but i am not
            ret[1][0] += vec.z * sin_theta;
            ret[2][0] += -vec.y * sin_theta;
            ret[0][1] += -vec.z * sin_theta;
            ret[2][1] += vec.x * sin_theta;
            ret[0][2] += vec.y * sin_theta;
            ret[1][2] += -vec.x * sin_theta;

            ret
        }
    }
    impl<T: NumberLike + Cos + Sin + Sqrt> Mat4<T> {
        pub fn lookat(position: Vector3<T>, target: Vector3<T>, up: Vector3<T>) -> Self {
            let mut ret = Self::identity();

            let direction = (position - target).normalized();
            let right = up.cross(&direction).normalized();
            let cam_up = direction.cross(&right);

            for i in 0..3 {
                ret[0][i] = right[i];
                ret[1][i] = cam_up[i];
                ret[2][i] = direction[i];
            }
            ret * Mat4::translate(&-position)
        }
    }
    impl<T: NumberLike + Tan + ToRadians> Mat4<T> {
        pub fn perspective(angle: T, aspect_ratio: T, near: T, far: T) -> Self {
            let mut ret = Mat4 {
                components: [[0.into(); 4]; 4],
            };
            let tan_half = (angle / 2.into()).to_radians().tan();
            let scale_x = <i8 as Into<T>>::into(1) / (tan_half * aspect_ratio);
            let scale_y = <i8 as Into<T>>::into(1) / tan_half;
            ret.components[0][0] = scale_x;
            ret.components[1][1] = scale_y;
            ret.components[2][2] = -((far) / (far - near));
            ret.components[3][2] = (-1).into();
            ret.components[2][3] = -((far * near) / (far - near));

            ret
        }
    }
    impl<T: NumberLike> Index<usize> for Mat4<T> {
        type Output = [T; 4];

        fn index(&self, index: usize) -> &Self::Output {
            &self.components[index]
        }
    }
    impl<T: NumberLike> IndexMut<usize> for Mat4<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            &mut self.components[index]
        }
    }
    impl<T: NumberLike> MulAssign<&T> for Mat4<T> {
        fn mul_assign(&mut self, rhs: &T) {
            self.components = self.components.map(|arr| arr.map(|v| v * *rhs));
        }
    }
    forward_move_assignop!([T:NumberLike] impl MulAssign, mul_assign for Mat4<T>, T);
    impl<T: NumberLike> Mul<&T> for &Mat4<T> {
        type Output = Mat4<T>;
        fn mul(self, rhs: &T) -> Self::Output {
            Mat4 {
                components: self.components.map(|arr| arr.map(|v| v * *rhs)),
            }
        }
    }
    forward_move_binop!([T:NumberLike] impl Mul, mul for Mat4<T>, T);
    impl<T: NumberLike> AddAssign<&Self> for Mat4<T> {
        fn add_assign(&mut self, rhs: &Self) {
            for y in 0..4 {
                for x in 0..4 {
                    self.components[y][x] = self.components[y][x] + rhs.components[y][x]
                }
            }
        }
    }
    forward_move_assignop!([T:NumberLike] impl AddAssign, add_assign for Mat4<T>, Self);
    impl<T: NumberLike> Add<Self> for &Mat4<T> {
        type Output = Mat4<T>;
        fn add(self, rhs: Self) -> Self::Output {
            let mut ret = Mat4::<T>::empty();
            for y in 0..4 {
                for x in 0..4 {
                    ret.components[y][x] = self.components[y][x] + rhs.components[y][x]
                }
            }
            ret
        }
    }
    forward_move_binop!([T:NumberLike] impl Add, add for Mat4<T>, Mat4<T>);
    impl<T: NumberLike> Mul<Self> for &Mat4<T> {
        type Output = Mat4<T>;
        fn mul(self, rhs: Self) -> Self::Output {
            let mut ret = Mat4::<T>::empty();
            for y in 0..4 {
                for x in 0..4 {
                    for item in 0..4 {
                        ret.components[y][x] += self.components[y][item] * rhs.components[item][x];
                    }
                }
            }
            ret
        }
    }
    forward_move_binop!([T:NumberLike] impl Mul, mul for Mat4<T>, Mat4<T>);
    impl<T: NumberLike> MulAssign<&Self> for Mat4<T> {
        fn mul_assign(&mut self, rhs: &Self) {
            let mut ret = Self::empty();
            for y in 0..4 {
                for x in 0..4 {
                    for item in 0..4 {
                        ret.components[y][x] += self.components[y][item] * rhs.components[item][x];
                    }
                }
            }
            *self = ret;
        }
    }
    forward_move_assignop!([T:NumberLike] impl MulAssign, mul_assign for Mat4<T>, Self);
    use super::vector::Vector4;
    impl<T: NumberLike> Mul<&Vector4<T>> for &Mat4<T> {
        type Output = Vector4<T>;
        fn mul(self, rhs: &Vector4<T>) -> Self::Output {
            let mut ret = Vector4::<T>::zero();
            for y in 0..4 {
                for item in 0..4 {
                    ret[y] += self.components[y][item] * rhs[item];
                }
            }
            ret
        }
    }
    forward_move_binop!([T:NumberLike] impl Mul, mul for Mat4<T>, Vector4<T>);
    impl<T: NumberLike> Mul<&Mat4<T>> for &Vector4<T> {
        type Output = Vector4<T>;
        fn mul(self, rhs: &Mat4<T>) -> Self::Output {
            rhs * self
        }
    }
    forward_move_binop!([T:NumberLike] impl Mul, mul for Vector4<T>, Mat4<T>);
}

pub mod vector {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    pub struct Vector4<T: NumberLike> {
        pub x: T,
        pub y: T,
        pub z: T,
        pub w: T,
    }
    impl<T: NumberLike + Sqrt> Vector4<T> {
        ///Might act weird on non floating point types
        pub fn norm(&self) -> T {
            (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
        }
        ///Might act weird on non floating point types
        ///Might panic or return a nonsense Vector for 0 vectors
        pub fn normalized(&self) -> Self {
            self / self.norm()
        }
    }
    impl<T: NumberLike> Vector4<T> {
        pub fn norm2(&self) -> T {
            self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
        }
        pub fn zero() -> Vector4<T> {
            Self {
                x: 0.into(),
                y: 0.into(),
                z: 0.into(),
                w: 0.into(),
            }
        }
        pub fn from_iterator(it: &mut impl Iterator<Item = T>) -> Self {
            Self {
                x: it.next().unwrap(),
                y: it.next().unwrap(),
                z: it.next().unwrap(),
                w: it.next().unwrap(),
            }
        }
    }
    macro_rules! vector4_op {
        ($($imp:ident, $method:ident, $imp_assign:ident, $method_assign:ident, $op:tt, $op_assign:tt)*) => {$(
            impl <T: NumberLike> $imp<&T> for &Vector4<T> {
                type Output = Vector4<T>;

                fn $method(self, rhs: &T) -> Self::Output {
                    Vector4 {
                        x: self.x $op *rhs,
                        y: self.y $op *rhs,
                        z: self.z $op *rhs,
                        w: self.w $op *rhs,
                    }
                }
            }
            forward_move_binop!([T:NumberLike] impl $imp, $method for Vector4<T>, T);
            impl<T: NumberLike + $imp_assign> $imp_assign<&T> for Vector4<T> {
                fn $method_assign(&mut self, rhs: &T) {
                    self.x $op_assign *rhs;
                    self.y $op_assign *rhs;
                    self.z $op_assign *rhs;
                    self.w $op_assign *rhs;
                }
            }
            forward_move_assignop!([T:NumberLike + $imp_assign] impl $imp_assign, $method_assign for Vector4<T>, T);
        )*}
    }
    vector4_op!(
        Add, add, AddAssign, add_assign, +, +=
        Sub, sub, SubAssign, sub_assign, -, -=
        Mul, mul, MulAssign, mul_assign, *, *=
        Div, div, DivAssign, div_assign, /, /=
    );
    macro_rules! vector4_self_op {
        ($($imp:ident, $method:ident, $imp_assign:ident, $method_assign:ident, $op:tt, $op_assign:tt)*) => {$(
            impl <T: NumberLike> $imp<Self> for &Vector4<T> {
                type Output = Vector4<T>;

                fn $method(self, rhs: Self) -> Self::Output {
                    Vector4 {
                        x: self.x $op rhs.x,
                        y: self.y $op rhs.y,
                        z: self.z $op rhs.z,
                        w: self.w $op rhs.w,
                    }
                }
            }
            forward_move_binop!([T:NumberLike] impl $imp, $method for Vector4<T>, Vector4<T>);
            impl<T: NumberLike + $imp_assign> $imp_assign<&Self> for Vector4<T> {
                fn $method_assign(&mut self, rhs: &Self) {
                    self.x $op_assign rhs.x;
                    self.y $op_assign rhs.y;
                    self.z $op_assign rhs.z;
                    self.w $op_assign rhs.w;
                }
            }
            forward_move_assignop!([T:NumberLike + $imp_assign] impl $imp_assign, $method_assign for Vector4<T>, Vector4<T>);
        )*}
    }
    vector4_self_op!(
        Add, add, AddAssign, add_assign, +, +=
        Sub, sub, SubAssign, sub_assign, -, -=
    );
    impl<T: NumberLike> Index<usize> for Vector4<T> {
        type Output = T;
        fn index(&self, index: usize) -> &Self::Output {
            match index {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                3 => &self.w,
                x => panic!("Non existant index {x}"),
            }
        }
    }
    impl<T: NumberLike> IndexMut<usize> for Vector4<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                0 => &mut self.x,
                1 => &mut self.y,
                2 => &mut self.z,
                3 => &mut self.w,
                x => panic!("Non existant index {x}"),
            }
        }
    }
    #[derive(Clone, Copy, Debug)]
    #[repr(C)]
    pub struct Vector3<T: NumberLike> {
        pub x: T,
        pub y: T,
        pub z: T,
    }
    impl<T: NumberLike + Sqrt> Vector3<T> {
        ///Might act weird on non floating point types
        pub fn norm(&self) -> T {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }
        ///Might act weird on non floating point types
        ///Might panic or return a nonsense Vector for 0 vectors
        pub fn normalized(&self) -> Self {
            self / self.norm()
        }
    }
    impl<T: NumberLike> Vector3<T> {
        pub fn norm2(&self) -> T {
            self.x * self.x + self.y * self.y + self.z * self.z
        }
        pub fn zero() -> Vector3<T> {
            Self {
                x: 0.into(),
                y: 0.into(),
                z: 0.into(),
            }
        }
        pub fn cross(&self, rhs: &Self) -> Self {
            Vector3 {
                x: self.y * rhs.z - self.z * rhs.y,
                y: self.z * rhs.x - self.x * rhs.z,
                z: self.x * rhs.y - self.y * rhs.x,
            }
        }
        pub fn from_iterator(it: &mut impl Iterator<Item = T>) -> Self {
            Self {
                x: it.next().unwrap(),
                y: it.next().unwrap(),
                z: it.next().unwrap(),
            }
        }
    }
    macro_rules! vector3_op {
        ($($imp:ident, $method:ident, $imp_assign:ident, $method_assign:ident, $op:tt, $op_assign:tt)*) => {$(
            impl <T: NumberLike> $imp<&T> for &Vector3<T> {
                type Output = Vector3<T>;

                fn $method(self, rhs: &T) -> Self::Output {
                    Vector3 {
                        x: self.x $op *rhs,
                        y: self.y $op *rhs,
                        z: self.z $op *rhs,
                    }
                }
            }
            forward_move_binop!([T:NumberLike] impl $imp, $method for Vector3<T>, T);
            impl<T: NumberLike + $imp_assign> $imp_assign<&T> for Vector3<T> {
                fn $method_assign(&mut self, rhs: &T) {
                    self.x $op_assign *rhs;
                    self.y $op_assign *rhs;
                    self.z $op_assign *rhs;
                }
            }
            forward_move_assignop!([T:NumberLike + $imp_assign] impl $imp_assign, $method_assign for Vector3<T>, T);
        )*}
    }
    vector3_op!(
        Add, add, AddAssign, add_assign, +, +=
        Sub, sub, SubAssign, sub_assign, -, -=
        Mul, mul, MulAssign, mul_assign, *, *=
        Div, div, DivAssign, div_assign, /, /=
    );
    macro_rules! vector3_self_op {
        ($($imp:ident, $method:ident, $imp_assign:ident, $method_assign:ident, $op:tt, $op_assign:tt)*) => {$(
            impl <T: NumberLike> $imp<Self> for &Vector3<T> {
                type Output = Vector3<T>;

                fn $method(self, rhs: Self) -> Self::Output {
                    Vector3 {
                        x: self.x $op rhs.x,
                        y: self.y $op rhs.y,
                        z: self.z $op rhs.z,
                    }
                }
            }
            forward_move_binop!([T:NumberLike] impl $imp, $method for Vector3<T>, Vector3<T>);
            impl<T: NumberLike + $imp_assign> $imp_assign<&Self> for Vector3<T> {
                fn $method_assign(&mut self, rhs: &Self) {
                    self.x $op_assign rhs.x;
                    self.y $op_assign rhs.y;
                    self.z $op_assign rhs.z;
                }
            }
            forward_move_assignop!([T:NumberLike + $imp_assign] impl $imp_assign, $method_assign for Vector3<T>, Vector3<T>);
        )*}
    }
    vector3_self_op!(
        Add, add, AddAssign, add_assign, +, +=
        Sub, sub, SubAssign, sub_assign, -, -=
    );

    impl<T: NumberLike + Neg> Neg for Vector3<T> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Vector3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }
    impl<T: NumberLike> Index<usize> for Vector3<T> {
        type Output = T;
        fn index(&self, index: usize) -> &Self::Output {
            match index {
                0 => &self.x,
                1 => &self.y,
                2 => &self.z,
                x => panic!("Non existant index {x}"),
            }
        }
    }
    impl<T: NumberLike> IndexMut<usize> for Vector3<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            match index {
                0 => &mut self.x,
                1 => &mut self.y,
                2 => &mut self.z,
                x => panic!("Non existant index {x}"),
            }
        }
    }
}
