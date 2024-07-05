//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

use core::arch::global_asm;
use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

#[path = "boards/qemu.rs"]
mod board;

global_asm!(include_str!("entry.asm"));

/// clear BSS segment
pub fn clear_bss() {
    extern "C" {
        static sbss: u8;
        static ebss: u8;
    }
    unsafe {
        // 获取BSS段的起始地址和结束地址
        let sbss_ptr = &sbss as *const u8 as usize;
        let ebss_ptr = &ebss as *const u8 as usize;

        // 创建一个范围迭代器
        (sbss_ptr..ebss_ptr).for_each(|addr| {
            // 将地址转换为可变指针，并写入0
            (addr as *mut u8).write_volatile(0);
        });
    }
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        static stext: u64; // begin addr of text segment
        static etext: u64; // end addr of text segment
        static srodata: u64; // start addr of Read-Only data segment
        static erodata: u64; // end addr of Read-Only data ssegment
        static sdata: u64; // start addr of data segment
        static edata: u64; // end addr of data segment
        static sbss: u64; // start addr of BSS segment
        static ebss: u64; // end addr of BSS segment
        static boot_stack_lower_bound: u64; // stack lower bound
        static boot_stack_top: u64; // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] \x1b[31mhello world!\x1b[0m");
    unsafe {
        trace!(
            "[kernel] .text [{:#x}, {:#x})",
            stext as usize,
            etext as usize
        );
        debug!(
            "[kernel] .rodata [{:#x}, {:#x})",
            srodata as usize, erodata as usize
        );
        info!(
            "[kernel] .data [{:#x}, {:#x})",
            sdata as usize, edata as usize
        );
        warn!(
            "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
            boot_stack_top as usize, boot_stack_lower_bound as usize
        );
        error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    }
    use crate::board::QEMUExit;
    board::QEMU_EXIT_HANDLE.exit_success(); // CI autotest success
                                            // board::QEMU_EXIT_HANDLE.exit_failure(); // CI autoest failed
}
