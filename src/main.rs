#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[repr(C)]
struct EfiSimpleTextOutputProtocol {
    _buf: u64,
    OutputString: fn(*const EfiSimpleTextOutputProtocol, *const u16) -> u64, 
    _buf2: [u64; 4],
    ClearScreen: fn(*const EfiSimpleTextOutputProtocol) -> u64,
}

#[repr(C)]
struct EfiSystemTable {
    buf: [i8; 60],
    ConOut: *const EfiSimpleTextOutputProtocol,
}

#[repr(usize)]
enum EfiStatus {
    SUCCESS = 0,
}

#[no_mangle]
fn efi_main(ImageHandle: u64, System: EfiSystemTable) -> EfiStatus
{
    let stdout: &EfiSimpleTextOutputProtocol = unsafe { &*System.ConOut };

    (stdout.ClearScreen)(System.ConOut);

    let string = "Hello UEFI!\n".as_bytes();
    let mut buf = [0u16; 32];
    for i in 0..string.len() {
        buf[i] = string[i] as u16;
    }

    (stdout.OutputString)(System.ConOut, buf.as_ptr());

    loop {}

    EfiStatus::SUCCESS;
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}
