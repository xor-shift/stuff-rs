pub mod canonical;

pub use canonical::*;
use crate::FloatingPoint;

macro_rules! alias_type {
    ($name:ident, $underlying:ty) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        pub struct $name(pub $underlying);

        impl From<$underlying> for $name { fn from(value: $underlying) -> Self { Self(value) } }
        impl Into<$underlying> for $name { fn into(self) -> $underlying { self.0 } }

        // impl FloatingPoint for $name {}
    };
}

alias_type!(NormallyDistributedF32, f32);
alias_type!(NormallyDistributedF64, f64);

macro_rules! alias_float {
    ($name:ident) => {
        #[derive(Debug, PartialEq, Copy, Clone)]
        pub struct $name<T: FloatingPoint>(T);

        impl<T: FloatingPoint> $name<T> {
            pub fn new(val: T) -> Self { Self(val) }

            pub fn get(self) -> T { self.0 }
        }

        impl<T: FloatingPoint> From<T> for $name<T> { fn from(value: T) -> Self { Self(value) } }
        //impl<T: FloatingPoint> Into<T> for $name<T> { fn into(self) -> T { self.0 } }
    };
}

alias_float!(NormallyDistributedFloat);
