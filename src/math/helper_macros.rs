macro_rules! impl_float_trait {
    ($imp:ident, $method:ident, $($t:ty)* ) => ($(
        impl $imp for $t {
            fn $method(self) -> Self {
                self.$method()
            }
        }
    )*)
}

macro_rules! impl_asf64_trait {
    ($imp:ident, $method:ident, $($t:ty)* ) => ($(
        impl $imp for $t {
            fn $method(self) -> Self {
                (self as f64).$method() as $t
            }
        }
    )*)
}

//Based on forward_ref and a little bit on forward_ref_generic

// implements binary operators "&T op U", "T op &U", "T op U"
// based on "&T op &U" where &'static T op &'static U has the same output type
// This should always be the case unless specialization is added to rustc
macro_rules! forward_move_binop {
    ([ $($generic:tt)* ] impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<$($generic)*> $imp<$u> for &$t {
            type Output = <&'static $t as $imp<&'static $u>>::Output;
            #[inline]
            fn $method(self, other: $u) -> Self::Output {
                $imp::$method(self, &other)
            }
        }
        impl<$($generic)*> $imp<&$u> for $t {
            type Output = <&'static $t as $imp<&'static $u>>::Output;
            #[inline]
            fn $method(self, other: &$u) -> Self::Output {
                $imp::$method(&self, other)
            }
        }
        impl<$($generic)*> $imp<$u> for $t {
            type Output = <&'static $t as $imp<&'static $u>>::Output;
            #[inline]
            fn $method(self, other: $u) -> Self::Output {
                $imp::$method(&self, &other)
            }
        }
    }
}
