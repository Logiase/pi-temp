#![deny(warnings)]
#![deny(unsafe_code)]

use embedded_graphics::{
    fonts::{Font8x16, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    style::TextStyleBuilder,
};
use rppal::{
    gpio::Gpio,
    hal::Delay,
    spi::{Bus, Mode, SlaveSelect, Spi},
};
use ssd1306::{prelude::*, Builder};
use std::{fs, thread::sleep, time::Duration};

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    let spi =
        Spi::new(Bus::Spi0, SlaveSelect::Ss0, 16_000_000, Mode::Mode0).expect("failed to open spi");

    let mut res = Gpio::new().unwrap().get(5).unwrap().into_output();
    let dc = Gpio::new().unwrap().get(6).unwrap().into_output();
    let cs = Gpio::new().unwrap().get(13).unwrap().into_output();

    let interface = SPIInterface::new(spi, dc, cs);

    let mut disp: GraphicsMode<_, _> = Builder::new().connect(interface).into();
    let mut delay = Delay::new();
    disp.reset(&mut res, &mut delay).unwrap();
    disp.init().unwrap();
    disp.flush().unwrap();

    let text_style = TextStyleBuilder::new(Font8x16)
        .text_color(BinaryColor::On)
        .build();

    loop {
        disp.clear();
        let temp: u32 = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        let temp: f32 = temp as f32 / 1000_f32;
        info!("current temp: {}", temp);
        Text::new("Current Temp", Point::zero())
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        Text::new(&format!("{}", temp), Point::new(0, 32))
            .into_styled(text_style)
            .draw(&mut disp)
            .unwrap();
        disp.flush().unwrap();
        sleep(Duration::from_secs(1));
    }
}
