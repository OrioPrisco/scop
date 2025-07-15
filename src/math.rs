use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

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
                ret[3][i] = vec[i];
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
    }
    impl<T: NumberLike + Cos + Sin> Mat4<T> {
        pub fn rotate(vec: &Vector3<T>, angle: &T) -> Self {
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
    impl<T: NumberLike + Tan + ToRadians> Mat4<T> {
        pub fn perspective(angle : T, aspect_ratio : T, near: T, far : T) -> Self {
            let mut ret = Mat4 {
                components: [[0.into(); 4]; 4],
            };
            let tan_half =  (angle/2.into()).to_radians().tan();
            let scale_x = <i8 as Into<T>>::into(1) / (tan_half * aspect_ratio);
            let scale_y = <i8 as Into<T>>::into(1) / tan_half;
            ret.components[0][0] = scale_x;
            ret.components[1][1] = scale_y;
            ret.components[2][2] = - ((far)/(far-near));
            ret.components[2][3] = (-1).into();
            ret.components[3][2] = - ((far * near)/(far-near));

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
    impl<T: NumberLike> MulAssign<T> for Mat4<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.components = self.components.map(|arr| arr.map(|v| v * rhs));
        }
    }
    impl<T: NumberLike> Mul<&T> for &Mat4<T> {
        type Output = Mat4<T>;
        fn mul(self, rhs: &T) -> Self::Output {
            Mat4 {
                components: self.components.map(|arr| arr.map(|v| v * *rhs)),
            }
        }
    }
    forward_move_binop!([T:NumberLike] impl Mul, mul for Mat4<T>, T);
    impl<T: NumberLike> AddAssign<Self> for Mat4<T> {
        fn add_assign(&mut self, rhs: Self) {
            for y in 0..4 {
                for x in 0..4 {
                    self.components[y][x] = self.components[y][x] + rhs.components[y][x]
                }
            }
        }
    }
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
    }
    impl<T: NumberLike> Mul<&T> for &Vector4<T> {
        type Output = Vector4<T>;

        fn mul(self, rhs: &T) -> Self::Output {
            Vector4 {
                x: self.x * *rhs,
                y: self.y * *rhs,
                z: self.z * *rhs,
                w: self.w * *rhs,
            }
        }
    }
    forward_move_binop!([T:NumberLike] impl Mul, mul for Vector4<T>, T);
    impl<T: NumberLike + MulAssign> MulAssign<T> for Vector4<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
            self.w *= rhs;
        }
    }
    impl<T: NumberLike> Div<&T> for &Vector4<T> {
        type Output = Vector4<T>;

        fn div(self, rhs: &T) -> Self::Output {
            Vector4 {
                x: self.x / *rhs,
                y: self.y / *rhs,
                z: self.z / *rhs,
                w: self.w / *rhs,
            }
        }
    }
    forward_move_binop!([T:NumberLike] impl Div, div for Vector4<T>, T);
    impl<T: NumberLike + DivAssign> DivAssign<T> for Vector4<T> {
        fn div_assign(&mut self, rhs: T) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
            self.w /= rhs;
        }
    }
    impl<T: NumberLike> Add<Self> for &Vector4<T> {
        type Output = Vector4<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Vector4 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }
    forward_move_binop!([T:NumberLike] impl Add, add for Vector4<T>, Vector4<T>);
    impl<T: NumberLike + AddAssign> AddAssign<T> for Vector4<T> {
        fn add_assign(&mut self, rhs: T) {
            self.x += rhs;
            self.y += rhs;
            self.z += rhs;
            self.w += rhs;
        }
    }
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
    }
    impl<T: NumberLike> Mul<&T> for &Vector3<T> {
        type Output = Vector3<T>;

        fn mul(self, rhs: &T) -> Self::Output {
            Vector3 {
                x: self.x * *rhs,
                y: self.y * *rhs,
                z: self.z * *rhs,
            }
        }
    }
    forward_move_binop!([T:NumberLike] impl Mul, mul for Vector3<T>, T);
    impl<T: NumberLike + MulAssign> MulAssign<T> for Vector3<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }
    impl<T: NumberLike> Div<&T> for &Vector3<T> {
        type Output = Vector3<T>;

        fn div(self, rhs: &T) -> Self::Output {
            Vector3 {
                x: self.x / *rhs,
                y: self.y / *rhs,
                z: self.z / *rhs,
            }
        }
    }
    forward_move_binop!([T:NumberLike] impl Div, div for Vector3<T>, T);
    impl<T: NumberLike + DivAssign> DivAssign<T> for Vector3<T> {
        fn div_assign(&mut self, rhs: T) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
        }
    }
    impl<T: NumberLike> Add<Self> for &Vector3<T> {
        type Output = Vector3<T>;

        fn add(self, rhs: Self) -> Self::Output {
            Vector3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }
    forward_move_binop!([T:NumberLike] impl Add, add for Vector3<T>, Vector3<T>);
    impl<T: NumberLike + AddAssign> AddAssign<T> for Vector3<T> {
        fn add_assign(&mut self, rhs: T) {
            self.x += rhs;
            self.y += rhs;
            self.z += rhs;
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
