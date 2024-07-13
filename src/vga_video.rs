use lazy_static::lazy_static;
use volatile::Volatile;
use spin::Mutex;

const BUFFER_WIDTH: usize = 320;
const BUFFER_HEIGHT: usize = 200;

type Framebuffer = [[Volatile<u8>; BUFFER_WIDTH]; BUFFER_HEIGHT];

lazy_static! {
    pub static ref FRAMEBUFFER: Mutex<&'static mut Framebuffer> = Mutex::new(
        unsafe {&mut *(0xa0000 as *mut Framebuffer) }
    );
}

