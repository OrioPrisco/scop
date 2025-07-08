use std::ops::{Add, Div, Mul};

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl<T> Sqrt for T
where
    T: Into<f64> + From<f64>,
{
    fn sqrt(self) -> Self {
        self.into().sqrt().into()
    }
}

pub trait NumberLike:
    Mul<Self, Output = Self> + Add<Self, Output = Self> + Div<Self, Output = Self> + Sqrt + Copy
{
}

impl<T> NumberLike for T where
    T: Mul<Self, Output = Self> + Add<Self, Output = Self> + Div<Self, Output = Self> + Sqrt + Copy
{
}

pub mod matrix {}

pub mod vector {
    use super::*;

    #[derive(Clone, Copy, Debug)]
    pub struct Vector4<T: NumberLike> {
        pub x: T,
        pub y: T,
        pub z: T,
        pub w: T,
    }
    impl<T> Vector4<T>
    where
        T: NumberLike,
    {
        ///Might act weird on non floating point types
        pub fn norm(&self) -> T {
            (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
        }
        pub fn norm2(&self) -> T {
            self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
        }
        ///Might act weird on non floating point types
        ///Might panic or return a nonsense Vector for 0 vectors
        pub fn normalized(self) -> Self {
            self / self.norm()
        }
    }
    impl<T> Mul<T> for Vector4<T>
    where
        T: NumberLike,
    {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            Vector4 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
                w: self.w * rhs,
            }
        }
    }
    impl<T> Div<T> for Vector4<T>
    where
        T: NumberLike,
    {
        type Output = Self;

        fn div(self, rhs: T) -> Self::Output {
            Vector4 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
                w: self.w / rhs,
            }
        }
    }
    impl<T> Add<Self> for Vector4<T>
    where
        T: NumberLike,
    {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Vector4 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }
    #[derive(Clone, Copy, Debug)]
    pub struct Vector3<T: NumberLike> {
        pub x: T,
        pub y: T,
        pub z: T,
    }
    impl<T> Vector3<T>
    where
        T: NumberLike,
    {
        ///Might act weird on non floating point types
        pub fn norm(&self) -> T {
            (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
        }
        pub fn norm2(&self) -> T {
            self.x * self.x + self.y * self.y + self.z * self.z
        }
        ///Might act weird on non floating point types
        pub fn normalized(self) -> Self {
            self / self.norm()
        }
    }
    impl<T> Mul<T> for Vector3<T>
    where
        T: NumberLike,
    {
        type Output = Self;

        fn mul(self, rhs: T) -> Self::Output {
            Vector3 {
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
            }
        }
    }
    impl<T> Div<T> for Vector3<T>
    where
        T: NumberLike,
    {
        type Output = Self;

        fn div(self, rhs: T) -> Self::Output {
            Vector3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }
    impl<T> Add<Self> for Vector3<T>
    where
        T: NumberLike,
    {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            Vector3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }
}
