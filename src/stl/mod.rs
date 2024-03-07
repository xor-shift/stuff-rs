mod binary;

struct Triangle {
    pub vertices: [[f32; 3]; 3],
    pub attrs: u16,
}

mod util {
    pub(super) fn vec_diff(lhs: [f32; 3], rhs: [f32; 3]) -> [f32; 3] { return [lhs[0] - rhs[0], lhs[1] - rhs[1], lhs[2] - rhs[2]] }

    pub(super) fn vec_cross(lhs: [f32; 3], rhs: [f32; 3]) -> [f32; 3] {
        return [
            lhs[1] * rhs[2] - lhs[2] * rhs[1], //
            lhs[0] * rhs[2] - lhs[2] * rhs[0],
            lhs[0] * rhs[1] - lhs[1] * rhs[0],
        ];
    }
}

impl Triangle {
    pub fn normal(&self) -> [f32; 3] {
        use util::vec_diff;

        return util::vec_cross(vec_diff(self.vertices[1], self.vertices[0]), vec_diff(self.vertices[2], self.vertices[0]));
    }

    fn from_bytes(bytes: [u8; 50]) -> Self {
        let get_vertex = |offset: usize| -> [f32; 3] {
            [
                f32::from_le_bytes((&bytes[offset + 0..offset + 4]).try_into().unwrap()),
                f32::from_le_bytes((&bytes[offset + 4..offset + 8]).try_into().unwrap()),
                f32::from_le_bytes((&bytes[offset + 8..offset + 12]).try_into().unwrap()),
            ]
        };

        let vertices = [get_vertex(12 + 12 * 0), get_vertex(12 + 12 * 1), get_vertex(12 + 12 * 2)];

        let attrs = u16::from_le_bytes((&bytes[48..50]).try_into().unwrap());

        Self { vertices, attrs }
    }
}

enum STLHeader {
    Plaintext(String),
    Binary([u8; 80], usize),
}

struct STLFile<Alloc: std::alloc::Allocator = std::alloc::Global> {
    header: STLHeader,
    triangles: Vec<Triangle, Alloc>,
}

#[cfg(test)]
mod test {
    #[test]
    fn foo() {
        assert!(false);
    }
}
