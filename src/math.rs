use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign};

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

pub trait NumberLike:
    Mul<Self, Output = Self>
    + Add<Self, Output = Self>
    + AddAssign<Self>
    + Div<Self, Output = Self>
    + Copy
    + From<i8>
{
}

impl<T> NumberLike for T where
    T: Mul<Self, Output = Self>
        + Add<Self, Output = Self>
        + AddAssign<Self>
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
    impl<T: NumberLike> Mul<T> for &Mat4<T> {
        type Output = Mat4<T>;
        fn mul(self, rhs: T) -> Self::Output {
            Mat4 {
                components: self.components.map(|arr| arr.map(|v| v * rhs)),
            }
        }
    }
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
    impl<T: NumberLike> Mul<&Mat4<T>> for &Vector4<T> {
        type Output = Vector4<T>;
        fn mul(self, rhs: &Mat4<T>) -> Self::Output {
            rhs * self
        }
    }
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
    impl<T: NumberLike> Mul<T> for &Vector4<T> {
        type Output = Vector4<T>;

        fn mul(self, rhs: T) -> Self::Output {
            Vector4 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
                w: self.w * rhs,
            }
        }
    }
    impl<T: NumberLike + MulAssign> MulAssign<T> for Vector4<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
            self.w *= rhs;
        }
    }
    impl<T: NumberLike> Div<T> for &Vector4<T> {
        type Output = Vector4<T>;

        fn div(self, rhs: T) -> Self::Output {
            Vector4 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
                w: self.w / rhs,
            }
        }
    }
    impl<T: NumberLike + DivAssign> DivAssign<T> for Vector4<T> {
        fn div_assign(&mut self, rhs: T) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
            self.w /= rhs;
        }
    }
    impl<T: NumberLike> Add<&Self> for Vector4<T> {
        type Output = Self;

        fn add(self, rhs: &Self) -> Self::Output {
            Vector4 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }
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
    impl<T: NumberLike> Mul<T> for &Vector3<T> {
        type Output = Vector3<T>;

        fn mul(self, rhs: T) -> Self::Output {
            Vector3 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }
    impl<T: NumberLike + MulAssign> MulAssign<T> for Vector3<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }
    impl<T: NumberLike> Div<T> for &Vector3<T> {
        type Output = Vector3<T>;

        fn div(self, rhs: T) -> Self::Output {
            Vector3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }
    impl<T: NumberLike + DivAssign> DivAssign<T> for Vector3<T> {
        fn div_assign(&mut self, rhs: T) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
        }
    }
    impl<T: NumberLike> Add<&Self> for Vector3<T> {
        type Output = Self;

        fn add(self, rhs: &Self) -> Self::Output {
            Vector3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }
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
