use super::*;

pub struct Decoder<'data> {
    pub data: &'data mut Box<[Color]>,

    write_head: usize,
    hash_table: ColorHashTable,
    previous: Color,
}

impl<'data> Decoder<'data> {
    pub fn new(out: &'data mut Box<[Color]>) -> Self {
        Self {
            data: out,

            write_head: 0,
            hash_table: Default::default(),
            previous: Color::from_rgba_bytes([0, 0, 0, 255]),
        }
    }

    pub fn process(&mut self, command: RawCommand) -> Result<()> {
        // print!("{:?}: ", command);

        let (to_emit, times_to_emit) = match command {
            RawCommand::Index(index) => (self.hash_table.recall(index), 1),
            RawCommand::Diff([dr, dg, db]) => (
                Color {
                    r: self.previous.r.wrapping_add(dr).wrapping_sub(2),
                    g: self.previous.g.wrapping_add(dg).wrapping_sub(2),
                    b: self.previous.b.wrapping_add(db).wrapping_sub(2),
                    a: self.previous.a,
                },
                1,
            ),
            RawCommand::Luma([dg, drdg, dbdg]) => (
                Color {
                    r: self.previous.r.wrapping_add(drdg.wrapping_sub(8).wrapping_add(dg.wrapping_sub(32))),
                    g: self.previous.g.wrapping_add(dg).wrapping_sub(32),
                    b: self.previous.b.wrapping_add(dbdg.wrapping_sub(8).wrapping_add(dg.wrapping_sub(32))),
                    a: self.previous.a,
                },
                1,
            ),
            RawCommand::Run(run_length) => (self.previous, run_length as usize + 1),
            RawCommand::ColorRGB([r, g, b]) => (Color { r, g, b, a: self.previous.a }, 1),
            RawCommand::ColorRGBA(rgba) => (Color::from_rgba_bytes(rgba), 1),
        };

        if self.write_head + times_to_emit > self.data.len() {
            return Err(QoIError::new(ErrorType::TooMuchData));
        }

        self.previous = to_emit;
        self.hash_table.remember(to_emit);

        for i in 0..times_to_emit {
            self.data[self.write_head + i] = to_emit;
        }

        self.write_head += times_to_emit;

        Ok(())
    }
}
