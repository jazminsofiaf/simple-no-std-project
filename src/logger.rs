pub struct SerialPortLogger {
    serial: u32
}
impl SerialPortLogger {

    pub fn new() -> SerialPortLogger{
        SerialPortLogger {
            serial: 0
        }
    }
    pub fn delay(&self) -> u32{
        self.serial
    }
}