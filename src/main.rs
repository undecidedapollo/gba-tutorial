#![no_std]
#![no_main]

use gba::prelude::*;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn main() -> ! {
    DISPSTAT.write(DisplayStatus::new().with_irq_vblank(true));
    IE.write(IrqBits::VBLANK);
    IME.write(true);

    DISPCNT.write(
        DisplayControl::new()
            .with_video_mode(VideoMode::_3)
            .with_show_bg2(true),
    );

    const SCREEN_WIDTH: usize = 240;
    const SCREEN_HEIGHT: usize = 160;

    let mut col = 0;
    let mut row = 0;

    loop {
        VBlankIntrWait();
        VIDEO3_VRAM.get(col, row).unwrap().write(Color::BLUE);
        col += 1;
        if col >= SCREEN_WIDTH {
            col = 0;
            row += 1;
        }
        if row >= SCREEN_HEIGHT {
            row = 0;
        }
    }
}
