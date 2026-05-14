#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    DISPCNT.write(
        DisplayControl::new()
            .with_video_mode(VideoMode::_3)
            .with_show_bg2(true),
    );

    let start_col = 10;
    let start_row = 20;
    let end_col = start_col + 120;
    let end_row = start_row + 30;
    for row in start_row..end_row {
        for col in start_col..end_col {
            VIDEO3_VRAM.get(col, row).unwrap().write(Color::BLUE);
        }
    }

    loop {}
}
