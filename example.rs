#[no_std];

extern mod core;

use core::vec::Vec;

#[start]
fn main(_: int, _: **u8) -> int {
    let mut xs = Vec::new();
    let mut i = 0;
    while i < 100 {
        xs.push(i);
        i += 1;
    }
    0
}
