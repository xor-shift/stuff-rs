pub trait NumericLimits {
    // TODO: following fields are skipped:
    // is_specialised (doesn't make sense for rust)
    // has_denorm
    // has_denorm_loss
    // round_style
    // is_iec559 (isn't ieee754 guaranteed here)
    // is_modulo (doesn't make sense for rust)
    // traps (doesn't make sense for rust)
    // tinyness_before
    // round_error

    const IS_SIGNED: bool = false;
    const IS_INTEGER: bool = false;
    const IS_EXACT: bool = false;
    const HAS_INFINITY: bool = false;
    const HAS_QUIET_NAN: bool = false;
    const HAS_SIGNALING_NAN: bool = false;
    const IS_BOUNDED: bool = false;
    const DIGITS: u32 = 0;
    const DIGITS10: u32 = 0;
    const RADIX: u32 = 0;
    const MIN_EXPONENT: i32 = 0;
    const MIN_EXPONENT10: i32 = 0;
    const MAX_EXPONENT: i32 = 0;
    const MAX_EXPONENT10: i32 = 0;

    fn min() -> Self;
    fn lowest() -> Self;
    fn max() -> Self;
    fn epsilon() -> Self;
    fn infinity() -> Self;
    fn quiet_nan() -> Self;
    fn denorm_min() -> Self;
}

macro_rules! numeric_limits_impl {
    (
        type_name = $type_name_t:ty,

        is_signed = $is_signed_v:literal,
        is_integer = $is_integer_v:literal,
        is_exact = $is_exact_v:literal,
        has_infinity = $has_infinity_v:literal,
        has_quiet_nan = $has_quiet_nan_v:literal,
        has_signaling_nan = $has_signaling_nan_v:literal,
        is_bounded = $is_bounded_v:literal,

        digits = $digits_expr:expr,
        digits10 = $digits10_expr:expr,
        radix = $radix_expr:expr,
        min_exponent = $min_exponent_expr:expr,
        min_exponent10 = $min_exponent10_expr:expr,
        max_exponent = $max_exponent_expr:expr,
        max_exponent10 = $max_exponent10_expr:expr,

        min = $min_expr_v:expr,
        lowest = $lowest_expr:expr,
        max = $max_expr:expr,
        epsilon = $epsilon_expr:expr,
        infinity = $infinity_expr:expr,
        quiet_nan = $quiet_nan_expr:expr,
        denorm_min = $denorm_min_expr:expr,
    ) => {
        impl NumericLimits for $type_name_t {
            const IS_SIGNED: bool = $is_signed_v;
            const IS_INTEGER: bool = $is_integer_v;
            const IS_EXACT: bool = $is_exact_v;
            const HAS_INFINITY: bool = $has_infinity_v;
            const HAS_QUIET_NAN: bool = $has_quiet_nan_v;
            const HAS_SIGNALING_NAN: bool = $has_signaling_nan_v;
            const IS_BOUNDED: bool = $is_bounded_v;

            const DIGITS: u32 = $digits_expr;
            const DIGITS10: u32 = $digits10_expr;
            const RADIX: u32 = $radix_expr;
            const MIN_EXPONENT: i32 = $min_exponent_expr;
            const MIN_EXPONENT10: i32 = $min_exponent10_expr;
            const MAX_EXPONENT: i32 = $max_exponent_expr;
            const MAX_EXPONENT10: i32 = $max_exponent10_expr;

            fn min() -> Self { $min_expr_v }
            fn lowest() -> Self { $lowest_expr }
            fn max() -> Self { $max_expr }
            fn epsilon() -> Self { $epsilon_expr }
            fn infinity() -> Self { $infinity_expr }
            fn quiet_nan() -> Self { $quiet_nan_expr }
            fn denorm_min() -> Self { $denorm_min_expr }
        }
    };
}

macro_rules! unsigned_numeric_limits_impl {
    ($type_name:ty) => {
        numeric_limits_impl!(
            type_name = $type_name,

            is_signed = false,
            is_integer = true,
            is_exact = true,
            has_infinity = false,
            has_quiet_nan = false,
            has_signaling_nan = false,
            is_bounded = true,
            digits = <$type_name>::BITS,
            digits10 = 2,
            radix = 2,
            min_exponent = 0,
            min_exponent10 = 0,
            max_exponent = 0,
            max_exponent10 = 0,

            min = <$type_name>::MIN,
            lowest = 0,
            max = <$type_name>::MAX,
            epsilon = 0,
            infinity = 0,
            quiet_nan = 0,
            denorm_min = 0,
        );
    };
}

unsigned_numeric_limits_impl!(u8);
unsigned_numeric_limits_impl!(u16);
unsigned_numeric_limits_impl!(u32);
unsigned_numeric_limits_impl!(u64);
unsigned_numeric_limits_impl!(u128);
unsigned_numeric_limits_impl!(usize);

macro_rules! signed_numeric_limits_impl {
    ($type_name:ty) => {
        numeric_limits_impl!(
            type_name = $type_name,

            is_signed = true,
            is_integer = true,
            is_exact = true,
            has_infinity = false,
            has_quiet_nan = false,
            has_signaling_nan = false,
            is_bounded = true,
            digits = <$type_name>::BITS - 1,
            digits10 = 2,
            radix = 2,
            min_exponent = 0,
            min_exponent10 = 0,
            max_exponent = 0,
            max_exponent10 = 0,

            min = <$type_name>::MIN,
            lowest = 0,
            max = <$type_name>::MAX,
            epsilon = 0,
            infinity = 0,
            quiet_nan = 0,
            denorm_min = 0,
        );
    };
}

signed_numeric_limits_impl!(i8);
signed_numeric_limits_impl!(i16);
signed_numeric_limits_impl!(i32);
signed_numeric_limits_impl!(i64);
signed_numeric_limits_impl!(i128);
signed_numeric_limits_impl!(isize);

macro_rules! float_numeric_limits_impl {
    ($type_name:ty, $denorm_min:literal) => {
        numeric_limits_impl!(
            type_name = $type_name,

            is_signed = true,
            is_integer = false,
            is_exact = true,
            has_infinity = false,
            has_quiet_nan = false,
            has_signaling_nan = false,
            is_bounded = true,
            digits = <$type_name>::DIGITS,
            digits10 = 2,
            radix = <$type_name>::RADIX,
            min_exponent = <$type_name>::MIN_EXP,
            min_exponent10 = <$type_name>::MIN_10_EXP,
            max_exponent = <$type_name>::MAX_EXP,
            max_exponent10 = <$type_name>::MAX_10_EXP,

            min = <$type_name>::MIN_POSITIVE,
            lowest = <$type_name>::MIN,
            max = <$type_name>::MAX,
            epsilon = <$type_name>::EPSILON,
            infinity = <$type_name>::INFINITY,
            quiet_nan = <$type_name>::NAN,
            denorm_min = $denorm_min,
        );
    };
}

float_numeric_limits_impl!(f32, 1.401298464e-45_f32);
float_numeric_limits_impl!(f64, 4.9406564584124654e-324);
