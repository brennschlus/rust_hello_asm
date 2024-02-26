#![no_std]
#![no_main]

use core::arch::asm;

#[no_mangle]
pub extern "C" fn _start() {
    unsafe {
        asm!(
            "mov r9, {msg}",
            "mov rax, 1",
            "mov rdi, 1",
            "mov rsi, r9",
            "mov rdx, 14",
            "syscall",
            "mov rax, 60",
            "xor rdi, rdi",
            "syscall",
            msg = in(reg) b"Hello, world!\n".as_ptr(),
        );
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
