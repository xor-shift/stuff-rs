use super::Triangle;
use super::{STLFile, STLHeader};

fn read_stl_header_binary<R: std::io::Read>(reader: &mut R) -> Option<STLHeader> {
    let mut header_bytes = [0; 80];
    reader.read_exact(&mut header_bytes).unwrap();

    let mut num_tris = [0; 4];
    reader.read_exact(&mut num_tris).unwrap();

    Some(STLHeader::Binary(header_bytes, u32::from_le_bytes(num_tris) as usize))
}

pub fn read_stl_binary<R: std::io::Read>(reader: &mut R) -> Option<STLFile> {
    let header = read_stl_header_binary(reader).unwrap();

    let mut triangles = Vec::new();

    if let STLHeader::Binary(_header, tri_count) = header {
        triangles.reserve(tri_count);
        for _ in 0..tri_count {
            let mut bytes = [0; 50];
            reader.read_exact(&mut bytes).unwrap();

            let tri = Triangle::from_bytes(bytes);
            triangles.push(tri);
        }
    } else {
        unreachable!();
    }

    Some(STLFile { header, triangles })
}
