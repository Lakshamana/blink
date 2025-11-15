#[derive(Debug)]
pub struct Led<PIN>
where
    PIN: embedded_hal::digital::v2::OutputPin,
{
    pin: PIN,
}

impl<PIN: embedded_hal::digital::v2::OutputPin> Led<PIN> {
    pub fn new(pin: PIN) -> Self {
        Self { pin }
    }

    pub fn on(&mut self) -> Result<(), PIN::Error> {
        self.pin.set_low()
    }

    pub fn off(&mut self) -> Result<(), PIN::Error> {
        self.pin.set_high()
    }
}
