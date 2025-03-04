/// Define entry point
///
///
/// # Example
///
/// ```
/// // define contract entry point
/// entry!(main)
/// ```
#[macro_export]
macro_rules! entry {
    ($main:path) => {
        extern crate alloc;

        #[alloc_error_handler]
        fn oom_handler(_layout: alloc::alloc::Layout) -> ! {
            panic!("Out of memory")
        }

        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            #[cfg(target_arch = "riscv64")]
            unsafe {
                asm!(
                    "lw a0, 0(sp)",
                    "addi a1, sp, 8",
                    "li a2, 0",
                    "call {}",
                    "li a7, 93",
                    "ecall",
                    sym $main,
                    options(noreturn)
                );
            }
            #[cfg(not(target_arch = "riscv64"))]
            panic!("This macro is only valid for riscv64 target");
        }

        #[lang = "eh_personality"]
        extern "C" fn eh_personality() {}

        /// Fix symbol missing
        #[no_mangle]
        pub extern "C" fn abort() {
            panic!("abort!");
        }

        #[panic_handler]
        fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
            #[cfg(debug_assertions)]
            {
                use alloc::format;

                let mut s = alloc::string::String::new();
                if let Some(p) = panic_info.payload().downcast_ref::<&str>() {
                    s.push_str(&format!("panic occurred: {:?}", p));
                } else {
                    s.push_str(&format!("panic occurred:"));
                }
                if let Some(m) = panic_info.message() {
                    s.push_str(&format!(" {:?}", m));
                }
                if let Some(location) = panic_info.location() {
                    s.push_str(&format!(
                        ", in file {}:{}",
                        location.file(),
                        location.line()
                    ));
                } else {
                    s.push_str(&format!(", but can't get location information..."));
                }

                $crate::syscalls::debug(s);
            }
            $crate::syscalls::exit(-1)
        }
    };
}
