use std::io::{BufRead, BufReader, Write};
use std::time::Duration;
use structopt::StructOpt;
use sog_to_stw::Args;
use sog_to_stw::nmea::{Sentence, VesselInfo};
use sog_to_stw::watchdog::{StatusMsg, run_forever};

const WATCHDOG_TIMER: u64 = 5;
const CONNECTION_TIMEOUT: u64 = 5;

fn main() {
    let timeout: u64 = match Args::from_args().time_out {
        None => {WATCHDOG_TIMER}
        Some(secs) => {secs}
    };

    run_forever(|tx| {
        let args: Args = Args::from_args();

        /*println!("Connecting to NMEA server at: {}!", args.server.as_str());

        let server = SocketAddr::from_str(args.server.as_str())
            .expect("Invalid server address format!");

        let stream = TcpStream::connect_timeout(&server, Duration::from_secs(2));
        let mut stream = match stream {
            Ok(stream) => {stream}
            Err(err) => {
                println!("encountered error ({}) on opening socket! Restarting worker!", err);
                tx.send(StatusMsg::Dead);
                return;
            }
        };
        */

        /*match stream.set_read_timeout(Some(Duration::from_secs(10))) {
            Ok(_) => {
                println!("set read timeout to: {}", READ_TIMEOUT);
            }
            Err(err) => {
                println!("encountered error while setting read timeout {}! Restarting worker!", err);
                tx.send(StatusMsg::Dead);
                return;
            }
        }
        match stream.set_write_timeout(Some(Duration::from_secs(1))) {
            Ok(_) => {
                println!("set write timeout to: {}", WRITE_TIMEOUT);
            }
            Err(err) => {
                println!("encountered error while setting read timeout {}! Restarting worker!", err);
                tx.send(StatusMsg::Dead);
                return;
            }
        }
        tx.send(StatusMsg::Alive);
        */

        let port = args.serial_port.clone().expect("Please specify a serial port!");
        let baud = args.serial_baud.clone().expect("Please specify the serial baud rate.");
        let time_out = match args.time_out.clone() {
            None => {CONNECTION_TIMEOUT}
            Some(timeout) => {timeout}
        };

        let mut port= match serialport::new(port, baud).open() {
            Ok(port) => { port }
            Err(_) => {
                println!("Unable to open serial port! Restarting worker!");
                tx.send(StatusMsg::Dead).expect("An unexpected error occurred. Restating Application");
                return;
            }
        };

        match port.set_timeout(Duration::from_secs(time_out)) {
            Ok(_) => {}
            Err(_) => {
                println!("Unable to set port timeout! Restarting worker!");
                tx.send(StatusMsg::Dead).expect("An unexpected error occurred. Restating Application");
                return;
            }
        }

        let mut reader = BufReader::new(&mut port);

        tx.send(StatusMsg::Alive).expect("An unexpected error occurred. Restating Application");
        let mut vessel_info = VesselInfo::default();
        loop {
            tx.send(StatusMsg::Alive).expect("An unexpected error occurred. Restating Application");
            let mut buff = String::new();
            let res = reader.read_line(&mut buff);
            match res {
                Ok(bytes) => {
                    if bytes == 0 {
                        println!("Connection closed! Restarting worker!");
                        tx.send(StatusMsg::Dead).expect("An unexpected error occurred. Restating Application");
                        return;
                    }
                }
                Err(err) => {
                    println!("{:?}", err);
                    tx.send(StatusMsg::Dead).expect("An unexpected error occurred. Restating Application");
                    return;
                }
            }
            match Sentence::try_from(buff.as_str()) {
                Ok(msg) => {
                    match msg {
                        Sentence::VHW(vhw) => {
                            vessel_info = vhw;
                        }
                        Sentence::SOG(sog) => {
                            match reader
                                .get_mut()
                                .write_all(vessel_info.to_modified(sog, "xx").as_bytes()) {
                                Ok(_) => {}
                                Err(err) => {
                                    println!("Writing to connection failed {}! Restarting worker!", err);
                                    return;
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                    continue;
                }
            };
        }
    }, 2*timeout);
}
