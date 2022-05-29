#![no_std]
#![feature(core_intrinsics)]
#![feature(default_alloc_error_handler)]

use wee_alloc;
use core::slice;
use core::mem;
extern crate alloc;
use alloc::vec::Vec;

// Use `wee_alloc` as the global allocator to decrease binary size.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[no_mangle]
fn read_wasm_memory<'a>(offset: *const u8, len: usize) -> &'a[u8] {
    unsafe { slice::from_raw_parts(offset, len) }
}

// Need to provide a tiny `panic` implementation for `#![no_std]`.
// This translates into an `unreachable` instruction that will
// raise a `trap` the WebAssembly execution if we panic at runtime.
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &::core::panic::PanicInfo) -> ! {
    ::core::intrinsics::abort();
}

#[no_mangle]
pub extern "C" fn alloc_wasm_memory(size: usize) -> *mut u8 {
    if size > isize::max_value() as usize {
        unsafe { core::hint::unreachable_unchecked() }
    }
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut u8;
}

#[no_mangle]
pub extern "C" fn answer(board_ptr: *const u8, board_size: usize, valid_move_ptr: *const u8, valid_move_count: usize, player_id: i32) -> i32 {
    if valid_move_count == 0 {
        unsafe { core::hint::unreachable_unchecked() }
    }
    let valid_move_list = read_wasm_memory(valid_move_ptr, valid_move_count);
    valid_move_list[0] as i32
}

