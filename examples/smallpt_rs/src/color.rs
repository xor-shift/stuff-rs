use stuff::{smallvec::Vector, FloatingPoint, ZeroAndOne};

#[derive(Clone, Copy, PartialEq)]
pub struct Color<T: FloatingPoint>(pub Vector<T, 3>);

fn srgb_oetf<T: FloatingPoint>(x: T) -> T {
    if x <= T::as_from(0.00031308) {
        T::as_from(12.92) * x
    } else {
        T::as_from(1.055) * x.powf(<T as ZeroAndOne>::one() / T::as_from(2.4)) - T::as_from(0.055)
    }
}

impl<T: FloatingPoint> Color<T> {
    pub const fn new(r: T, g: T, b: T) -> Self { Self(Vector([r, g, b])) }

    pub fn to_qoi_color(self) -> stuff::qoi::Color {
        stuff::qoi::Color::from_rgba_bytes([
            (srgb_oetf(self.0[0].clamp(<T as ZeroAndOne>::zero(), <T as ZeroAndOne>::one())) * T::as_from(255)).trunc().to_scalar().unwrap(),
            (srgb_oetf(self.0[1].clamp(<T as ZeroAndOne>::zero(), <T as ZeroAndOne>::one())) * T::as_from(255)).trunc().to_scalar().unwrap(),
            (srgb_oetf(self.0[2].clamp(<T as ZeroAndOne>::zero(), <T as ZeroAndOne>::one())) * T::as_from(255)).trunc().to_scalar().unwrap(),
            255,
        ])
    }
}
