use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl Sqrt for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl Sqrt for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
impl Sqrt for i32 {
    fn sqrt(self) -> Self {
        (self as f64).sqrt() as i32
    }
}
impl Sqrt for i64 {
    fn sqrt(self) -> Self {
        (self as f64).sqrt() as i64
    }
}

pub trait NumberLike:
    Mul<Self, Output = Self>
    + Add<Self, Output = Self>
    + Div<Self, Output = Self>
    + Copy
    + From<i8>
{
}

impl<T> NumberLike for T where
    T: Mul<Self, Output = Self>
        + Add<Self, Output = Self>
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
    impl<T: NumberLike> Mat4<T> {
        pub fn empty() -> Mat4<T> {
            Mat4 {
                components: [[0.into(); 4]; 4],
            }
        }
        pub fn identity() -> Mat4<T> {
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
    impl<T: NumberLike> MulAssign<T> for Mat4<T> {
        fn mul_assign(&mut self, rhs: T) {
            self.components = self.components.map(|arr| arr.map(|v| v * rhs));
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
}
