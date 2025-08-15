#![no_std]
#![no_main]
extern crate alloc;

use core::{ffi::{c_char, c_int}, panic::PanicInfo, time::Duration};
use alloc::boxed::Box;

use embedded_graphics::prelude::*;
use mousefood::{EmbeddedBackend, EmbeddedBackendConfig, TerminalAlignment};
use ratatui::{widgets::Paragraph, Frame, Terminal};
use why2025_badge::{println, malloc::Malloc};
use why2025_badge_embedded_graphics::Why2025BadgeWindow;

#[unsafe(no_mangle)]
pub extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> i32 {
    println!("Hello World from Rust!");

    let mut window = Why2025BadgeWindow::new_floating(Size::new(640, 480), "Console");

    let config = EmbeddedBackendConfig {
        flush_callback: Box::new(|d: &mut Why2025BadgeWindow| d.flush()),
        font_regular: ibm437::IBM437_8X8_REGULAR,
        font_bold: Some(ibm437::IBM437_8X8_BOLD),
        font_italic: None,
        vertical_alignment: TerminalAlignment::Center,
        horizontal_alignment: TerminalAlignment::Center,
    };
    let events = window.events();
    let backend = EmbeddedBackend::new(&mut window, config);
    let mut terminal = Terminal::new(backend).unwrap();
    loop {
        if let Some(e) = events.poll(true, Duration::from_millis(100)) {
            println!("new event: {e:#?}");
        }
        terminal.draw(draw).unwrap();
    }
}

fn draw(frame: &mut Frame) {
    let text = Paragraph::new("Hello World with ratatui!");
    frame.render_widget(text, frame.area());
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC: {info}");
    unsafe { why2025_badge::ffi::abort() };
}

#[global_allocator]
static ALLOC: Malloc = Malloc;
