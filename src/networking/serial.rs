use serialport::SerialPort;
use crate::Args;

pub fn open(_args: &Args) -> Result<Box<dyn SerialPort>, ()> {
    Err(())
}
