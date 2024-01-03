pub trait CheckedAdd<Rhs = Self> {
    type Output;

    fn checked_add(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedSub<Rhs = Self> {
    type Output;

    fn checked_sub(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedMul<Rhs = Self> {
    type Output;

    fn checked_mul(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedDiv<Rhs = Self> {
    type Output;

    fn checked_div(self, rhs: Rhs) -> Option<Self::Output>;
}

pub trait CheckedShr<Rhs = Self> {
    type Output;

    fn checked_shr(self, rhs: u32) -> Option<Self::Output>;
}

pub trait CheckedShl<Rhs = Self> {
    type Output;

    fn checked_shl(self, rhs: u32) -> Option<Self::Output>;
}
