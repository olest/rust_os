#![no_std]
#![no_main]

// https://doc.rust-lang.org/unstable-book/language-features/custom-test-frameworks.html
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]

mod vga_buffer;

use core::panic::PanicInfo;

// This function is called on panic.
// This is a diverging function, which does not return
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// 
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    //println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

//static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    //vga_buffer::print_something();

    //let vga_buffer = 0xb8000 as *mut u8;

    //for (i, &byte) in HELLO.iter().enumerate() {
        // this tells the compiler that we are sure about these memory accesses
    //    unsafe {
    //        *vga_buffer.offset(i as isize * 2) = byte;
    //        *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //    }
    //}

    //use core::fmt::Write;
    //vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    //write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();


    println!("Hello World{}", "!");
    panic!("Some panic message");
    loop {}
}

