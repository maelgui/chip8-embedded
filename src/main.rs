#![no_std]
#![no_main]

extern crate xiao_m0 as hal;

use panic_halt as _;

use chip8core;

use embedded_graphics::{pixelcolor::BinaryColor, prelude::*};
use embedded_graphics::geometry::Point;
use hal::{i2c_master, sercom::{I2CMaster2, Pad, Pad0, Pad1}};
use hal::clock::GenericClockController;
use hal::entry;
use hal::pac::Peripherals;
use hal::prelude::*;
use ssd1306::{Builder, I2CDIBuilder, displaysize::DisplaySize128x32, mode::GraphicsMode, prelude::I2CInterface};

type I2C = I2CMaster2<Pad<hal::sercom::Sercom2, Pad0, hal::gpio::Pin<hal::gpio::v2::PA08, hal::gpio::v2::Alternate<hal::gpio::v2::D>>>, Pad<hal::sercom::Sercom2, Pad1, hal::gpio::Pin<hal::gpio::v2::PA09, hal::gpio::v2::Alternate<hal::gpio::v2::D>>>>;

struct MyWindow(GraphicsMode<I2CInterface<I2C>,DisplaySize128x32>);

impl MyWindow {
    fn new(i2c: I2C) -> Self {
        let interface = I2CDIBuilder::new().init(i2c);
        let mut disp: GraphicsMode<_, _> = Builder::new().size(DisplaySize128x32).connect(interface).into();
    
        disp.init().unwrap();
        disp.flush().unwrap();
        
        MyWindow(disp)
    }
}

impl chip8core::Window for MyWindow {
    fn update_with_buffer(self: &mut Self, buffer: &[u8], width: usize, height: usize) {
        for y in 0..height {
            for x in 0..width {
                let color = if buffer[y*width + x] == 1 { BinaryColor::On } else { BinaryColor::Off };
                Pixel(Point::new(x as i32, y as i32), color).draw(&mut self.0).unwrap();
            }
        }
        self.0.flush().unwrap();

    }

    fn is_running(self: &mut Self) -> bool {
        true
    }
}


impl chip8core::Keyboard for MyWindow {
    fn is_key_down(self: &Self, key: chip8core::Key) -> bool {
        todo!()
    }

    fn wait_key_down(self: &Self) -> chip8core::Key {
        todo!()
    }
}

#[entry]
fn main() -> ! {

    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);

    let i2c = i2c_master(
        &mut clocks,
        400.khz(),
        peripherals.SERCOM2,
        &mut peripherals.PM,
        // Metro M0 express has I2C on pins PA22, PA23
        pins.a4.into_floating_input(&mut pins.port),
        pins.a5.into_floating_input(&mut pins.port),
        &mut pins.port
    );


    let rom_file = include_bytes!("../../Chip8Emu/chip8-roms/programs/Chip8 Picture.ch8");
    
    let mut my_window = MyWindow::new(i2c);
    let mut my_chip8 = chip8core::Chip8::new(&mut my_window);

    my_chip8.init(rom_file);

    my_chip8.start();

    loop {}
}
