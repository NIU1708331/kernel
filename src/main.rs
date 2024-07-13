#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use kernel::println;
use bootloader::{BootInfo, entry_point};

// use alloc::{boxed::Box, vec, string, vec::Vec, rc::Rc};
use kernel::task::{keyboard, Task, executor::Executor}; 

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use kernel::allocator;
    use x86_64::VirtAddr;
    use kernel::memory;
    use kernel::memory::BootInfoFrameAllocator;

    println!("Hello world!");
    kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization falied");

    use kernel::vga_video::FRAMEBUFFER;

    {
        let mut fb = FRAMEBUFFER.lock();
        for y in 0..340 {
            for x in 0..200 {
                fb[x][y].write(((x + y) % 255) as u8);
            }
        }

    } // Release the lock

    #[cfg(test)]
    test_main();

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));

    println!("Kernel didn't crash");
    executor.run();
    // kernel::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info);
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
