#![no_main]
#![no_std]
mod graphics;
mod hardware;

use graphics::framebuffer::{Color, Framebuffer};
use graphics::font::draw_text;
use hardware::screen::DIMS;
use core::panic::PanicInfo;

use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn write_usize(mut value: usize, buffer: &mut [u8; 20]) -> &str {
    let mut index = buffer.len();
    if value == 0 {
        index -= 1;
        buffer[index] = b'0';
    } else {
        while value > 0 {
            index -= 1;
            buffer[index] = b'0' + (value % 10) as u8;
            value /= 10;
        }
    }

    core::str::from_utf8(&buffer[index..]).unwrap()
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let gop_handle = boot::get_handle_for_protocol::<GraphicsOutput>().unwrap();
    let mut gop = boot::open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

    let mode_info = gop.current_mode_info();
    let resolution = mode_info.resolution();

    let mut frame_buffer = gop.frame_buffer();

let mut fb = unsafe {
    Framebuffer::new(
        frame_buffer.as_mut_ptr(),
        frame_buffer.size(),
        resolution.0,
        resolution.1,
        mode_info.stride(),
    )
};

fb.fill(Color::BLACK);

fb.fill_rect(
    0,
    0,
    fb.width() / 2,
    fb.height(),
    Color::RED,
);
draw_text(
    &mut fb,
    100,
    100,
    "MAC2MONITOR",
    Color::WHITE,
    8,
);

draw_text(
    &mut fb,
    100,
    180,
    "BOOT OK",
    Color::WHITE,
    6,
);

let mut buffer = [0u8; 20];
draw_text(
    &mut fb,
    100,
    240,
    write_usize(DIMS.height, &mut buffer),
    Color::WHITE,
    5,
);

draw_text(
    &mut fb,
    100,
    300,
    write_usize(DIMS.width, &mut buffer),
    Color::WHITE,
    8,
);

    loop {}
}