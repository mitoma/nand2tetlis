use const_value;
use adder::*;

pub fn alu(
    x: [bool; 16],
    y: [bool; 16],
    zx: bool,
    nx: bool,
    zy: bool,
    ny: bool,
    f: bool,
    no: bool,
) -> ([bool; 16], bool, bool) {
    add16(const_value::ZERO, y);
    ([false; 16], false, false)
}
