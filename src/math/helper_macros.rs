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
