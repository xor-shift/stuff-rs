use super::*;

// terrible
const fn best_modular_diff(lhs: u32, rhs: u32, modulo: u32) -> i64 {
    // 0 - 254 === 2
    // 254 - 252 === 2
    // 252 - 254 === -2
    // 254 - 0 === -2

    let res_0 = (lhs as i64 - rhs as i64 - modulo as i64) % modulo as i64;
    let res_1 = (lhs as i64 - rhs as i64) % modulo as i64;
    let res_2 = (lhs as i64 - rhs as i64 + modulo as i64) % modulo as i64;

    match (res_0.abs() < res_1.abs(), res_1.abs() < res_2.abs(), res_0.abs() < res_2.abs()) {
        (true, _, true) => res_0,
        (false, true, _) => res_1,
        (_, false, false) => res_2,
        _ => res_1,
    }
}

// disgusting
const _: [(); 0] = [(); 1 - (best_modular_diff(0, 254, 256) == 2) as usize];
const _: [(); 0] = [(); 1 - (best_modular_diff(254, 252, 256) == 2) as usize];
const _: [(); 0] = [(); 1 - (best_modular_diff(252, 254, 256) == -2) as usize];
const _: [(); 0] = [(); 1 - (best_modular_diff(254, 0, 256) == -2) as usize];

// verbose enough?
fn decide_idx_diff_luma_rgb_rgba(hash_table: &ColorHashTable, previous: Color, color: Color) -> RawCommand {
    if hash_table.recall(color.hash()) == color {
        return RawCommand::Index(color.hash());
    }

    if color.a != previous.a {
        return RawCommand::ColorRGBA([color.r, color.g, color.b, color.a]);
    }

    let dr = best_modular_diff(color.r as u32, previous.r as u32, 256);
    let dg = best_modular_diff(color.g as u32, previous.g as u32, 256);
    let db = best_modular_diff(color.b as u32, previous.b as u32, 256);

    match (dr, dg, db) {
        (-2..=1, -2..=1, -2..=1) => RawCommand::Diff([(dr + 2) as u8, (dg + 2) as u8, (db + 2) as u8]),
        _ => match (dg, dr - dg, db - dg) {
            (-32..=31, -8..=7, -8..=7) => RawCommand::Luma([(dg + 32) as u8, (dr - dg + 8) as u8, (db - dg + 8) as u8]),
            _ => RawCommand::ColorRGB([color.r, color.g, color.b]),
        },
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EncoderState {
    Fresh,
    InRun(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EncoderOutput {
    None,
    Single(RawCommand),
    Double(RawCommand, RawCommand),
}

pub struct Encoder<Callback: FnMut(&[u8]) -> std::io::Result<()>> {
    callback: Callback,

    encoded_pixels: usize,

    hash_table: ColorHashTable,
    previous_color: Color,

    state: EncoderState,
}

impl<Callback: FnMut(&[u8]) -> std::io::Result<()>> Encoder<Callback> {
    pub fn new(callback: Callback) -> Self {
        Self {
            callback,
            encoded_pixels: 0,
            hash_table: Default::default(),
            previous_color: Color::from_rgba_bytes([0, 0, 0, 255]),
            state: EncoderState::Fresh,
        }
    }

    fn emit_command(&mut self, command: RawCommand) -> std::io::Result<()> {
        let (bytes, len) = command.to_bytes();
        let slice = &bytes[0..len];
        (self.callback)(slice)
    }

    pub fn feed(&mut self, color: Color) -> std::io::Result<()> {
        let output = self.feed_impl(color);

        self.encoded_pixels += 1;

        match output {
            EncoderOutput::None => Ok(()),
            EncoderOutput::Single(command) => self.emit_command(command),
            EncoderOutput::Double(command_0, command_1) => {
                self.emit_command(command_0)?;
                self.emit_command(command_1)?;
                Ok(())
            }
        }
    }

    pub fn finish(mut self) -> std::io::Result<()> {
        match self.state {
            EncoderState::Fresh => (),
            EncoderState::InRun(run_length) => self.emit_command(RawCommand::Run(run_length - 1))?,
        };
        Ok(()) //
    }

    fn feed_impl(&mut self, color: Color) -> EncoderOutput {
        let ret = match (self.state, color == self.previous_color) {
            (EncoderState::Fresh, false) => EncoderOutput::Single(decide_idx_diff_luma_rgb_rgba(&self.hash_table, self.previous_color, color)),
            (EncoderState::Fresh, true) => EncoderOutput::None,
            (EncoderState::InRun(run_length), continue_run) => match (continue_run, run_length == 62) {
                (true, true) => EncoderOutput::Single(RawCommand::Run(61)),
                (true, false) => EncoderOutput::None,
                (false, true) => EncoderOutput::Double(RawCommand::Run(61), decide_idx_diff_luma_rgb_rgba(&self.hash_table, self.previous_color, color)),
                (false, false) => EncoderOutput::Double(RawCommand::Run(run_length - 1), decide_idx_diff_luma_rgb_rgba(&self.hash_table, self.previous_color, color)),
            },
        };

        let next_state = match (self.state, color == self.previous_color) {
            (_, false) => EncoderState::Fresh,
            (EncoderState::Fresh, true) => EncoderState::InRun(1),
            (EncoderState::InRun(62), true) => EncoderState::InRun(1),
            (EncoderState::InRun(run_length), true) => EncoderState::InRun((run_length + 1) % 63),
        };

        // println!("#{:06} ({:?} -> {:?}) {:?}: {:?} -> {:?}", self.encoded_pixels, self.previous_color, color, ret, self.state, next_state);

        self.state = next_state;
        self.previous_color = color;
        self.hash_table.remember(color);

        ret
    }
}
