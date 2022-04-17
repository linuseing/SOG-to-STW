use crate::Args;
use serialport::SerialPort;

pub fn open(_args: &Args) -> Result<Box<dyn SerialPort>, ()> {
    Err(())
}
