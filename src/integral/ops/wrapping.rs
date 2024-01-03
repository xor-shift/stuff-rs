pub trait WrappingAdd<Rhs = Self> {
    type Output;

    fn wrapping_add(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingSub<Rhs = Self> {
    type Output;

    fn wrapping_sub(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingMul<Rhs = Self> {
    type Output;

    fn wrapping_mul(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingDiv<Rhs = Self> {
    type Output;

    fn wrapping_div(self, rhs: Rhs) -> Self::Output;
}

pub trait WrappingShr<Rhs = Self> {
    type Output;

    fn wrapping_shr(self, rhs: u32) -> Self::Output;
}

pub trait WrappingShl<Rhs = Self> {
    type Output;

    fn wrapping_shl(self, rhs: u32) -> Self::Output;
}
