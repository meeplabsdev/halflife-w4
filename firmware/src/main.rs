use iced::{
    Size,
    widget::{Row, button, row},
};
use wiringpi::pin::{
    self, OutputPin,
    Value::{High, Low},
};

#[derive(Debug, Clone, Copy)]
enum Message {
    EnableAmp,
    DisableAmp,
    N6Db,
    P0Db,
    P3Db,
    P6Db,
}

struct Application {
    amp_en: OutputPin<pin::WiringPi>,
    amp_g0: OutputPin<pin::WiringPi>,
    amp_g1: OutputPin<pin::WiringPi>,
}

impl Application {
    fn new() -> Self {
        // http://www.orangepi.org/orangepiwiki/images/thumb/b/b7/Zero3-img172.png/780px-Zero3-img172.png
        let wiring = wiringpi::setup();

        Self {
            amp_en: wiring.output_pin(5),
            amp_g0: wiring.output_pin(7),
            amp_g1: wiring.output_pin(8),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::EnableAmp => self.amp_en.digital_write(High),
            Message::DisableAmp => self.amp_en.digital_write(Low),
            Message::N6Db => {
                self.amp_g0.digital_write(Low);
                self.amp_g1.digital_write(Low);
            }
            Message::P0Db => {
                self.amp_g0.digital_write(High);
                self.amp_g1.digital_write(Low);
            }
            Message::P3Db => {
                self.amp_g0.digital_write(Low);
                self.amp_g1.digital_write(High);
            }
            Message::P6Db => {
                self.amp_g0.digital_write(High);
                self.amp_g1.digital_write(High);
            }
        }
    }

    fn view(&self) -> Row<'_, Message> {
        row![
            button("enable amp").on_press(Message::EnableAmp),
            button("disable amp").on_press(Message::DisableAmp),
            button("-6db").on_press(Message::N6Db),
            button("+0db").on_press(Message::P0Db),
            button("+3db").on_press(Message::P3Db),
            button("+6db").on_press(Message::P6Db),
        ]
    }
}

pub fn main() -> iced::Result {
    iced::application(Application::new, Application::update, Application::view)
        .window_size(Size::new(800.0, 480.0))
        .scale_factor(|_| 1.0)
        .resizable(false)
        .run()
}
