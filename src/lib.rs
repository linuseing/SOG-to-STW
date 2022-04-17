use structopt::StructOpt;

pub mod networking;
pub mod nmea;
pub mod watchdog;

#[derive(Debug, StructOpt)]
#[structopt(name = "SOG to STW", about = "Transform SOG sentencces to STW.")]
pub struct Args {
    #[structopt(name = "NMEA server")]
    pub server: String,
    #[structopt(name = "Timeout for watchdog", about = "to be implemented")]
    pub time_out: Option<u64>,
    pub serial_port: Option<String>,
    pub serial_baud: Option<u32>,
}
