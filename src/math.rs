use std::ops::Mul;

pub mod matrix {

}

pub mod vector {
    use super::*;

    pub struct Vector4<T> {
        pub x : T,
        pub y : T,
        pub z : T,
        pub w : T,
    }
    impl<T : Mul<T, Output=T> + Copy> Mul<T> for Vector4<T> {
        type Output=Self;

        fn mul(self, rhs: T) -> Self::Output {
            Vector4{
                x: self.x * rhs,
                y: self.y * rhs,
                z: self.z * rhs,
                w: self.w * rhs,
            }
        }
    }
}
