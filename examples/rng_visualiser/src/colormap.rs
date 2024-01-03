const fn convert_to_bytes<const N: usize>(values: &[(f64, f64, f64); N]) -> [u8; N * 8 * 3] {
    let mut ret = std::mem::MaybeUninit::uninit();

    // this looked so pretty too :(
    // ret.iter_mut().skip(i * 8 * 3).take(8).zip(values[i].0.to_le_bytes().iter()).for_each(|(dst, src)| *dst = *src);

    let mut i = 0;
    while i < N {
        let cur = values[i];

        let bytes: [u8; 8 * 3] = unsafe {
            let mut result = std::mem::MaybeUninit::uninit();
            let result_ptr = result.as_mut_ptr() as *mut u8;

            std::ptr::copy_nonoverlapping(cur.0.to_be_bytes().as_ptr(), result_ptr, 8);
            std::ptr::copy_nonoverlapping(cur.1.to_be_bytes().as_ptr(), result_ptr.add(8), 8);
            std::ptr::copy_nonoverlapping(cur.2.to_be_bytes().as_ptr(), result_ptr.add(16), 8);

            result.assume_init()
        };

        unsafe {
            std::ptr::copy_nonoverlapping(bytes.as_ptr(), (ret.as_mut_ptr() as *mut u8).add(i * 8 * 3), bytes.len());
        };

        i += 1;
    }

    unsafe { ret.assume_init() }
}

const fn convert_from_bytes<const N: usize>(values: &[u8; N * 8 * 3]) -> [(f64, f64, f64); N] {
    let mut ret = std::mem::MaybeUninit::uninit();

    let mut i = 0;
    while i < N {
        let cur = unsafe {
            let src_base_ptr = values.as_ptr().add(i * 8 * 3);
            let mut result = ([0; 8], [0; 8], [0; 8]);

            std::ptr::copy_nonoverlapping(src_base_ptr, result.0.as_mut_ptr(), 8);
            std::ptr::copy_nonoverlapping(src_base_ptr.add(8), result.1.as_mut_ptr(), 8);
            std::ptr::copy_nonoverlapping(src_base_ptr.add(16), result.2.as_mut_ptr(), 8);

            result
        };

        let cur = (f64::from_be_bytes(cur.0), f64::from_be_bytes(cur.1), f64::from_be_bytes(cur.2));

        unsafe {
            *(ret.as_ptr() as *mut (f64, f64, f64)).add(i) = cur;
        };

        i += 1;
    }

    unsafe { ret.assume_init() }
}

pub const COLORMAP_GRAYSCALE: [(f64, f64, f64); 2] = convert_from_bytes(include_bytes!("colormaps/grayscale.bin"));
pub const COLORMAP_MAGMA: [(f64, f64, f64); 256] = convert_from_bytes(include_bytes!("colormaps/magma.bin"));
pub const COLORMAP_INFERNO: [(f64, f64, f64); 256] = convert_from_bytes(include_bytes!("colormaps/inferno.bin"));
pub const COLORMAP_PLASMA: [(f64, f64, f64); 256] = convert_from_bytes(include_bytes!("colormaps/plasma.bin"));
pub const COLORMAP_VIRIDIS: [(f64, f64, f64); 256] = convert_from_bytes(include_bytes!("colormaps/viridis.bin"));
pub const COLORMAP_CIVIDIS: [(f64, f64, f64); 256] = convert_from_bytes(include_bytes!("colormaps/cividis.bin"));
pub const COLORMAP_TWILIGHT: [(f64, f64, f64); 510] = convert_from_bytes(include_bytes!("colormaps/twilight.bin"));
pub const COLORMAP_TURBO: [(f64, f64, f64); 256] = convert_from_bytes(include_bytes!("colormaps/turbo.bin"));

pub fn map_to_color(v: f64, colormap: &[(f64, f64, f64)]) -> (f64, f64, f64) {
    let v = v.clamp(0f64, 1f64) * (colormap.len() - 1) as f64;

    let prev = v.floor();
    let next = v.ceil();
    let t = v - prev;

    let prev_color = colormap[prev as usize];
    let next_color = colormap[next as usize];

    (
        prev_color.0 + t * (next_color.0 - prev_color.0),
        prev_color.1 + t * (next_color.1 - prev_color.1),
        prev_color.2 + t * (next_color.2 - prev_color.2),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        // sanity check
        assert_eq!(COLORMAP_GRAYSCALE, [(0f64, 0f64, 0f64), (1f64, 1f64, 1f64)]);

        let foo = [(1f64, 2f64, 3f64), (0.1f64, 0.2f64, 0.3f64)];
        let v1 = convert_to_bytes(&foo);
        let v2 = convert_from_bytes(&v1);

        assert_eq!(foo, v2);
    }
}
