use std::process::exit;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub enum StatusMsg {
    Alive,
    Dead,
}

pub fn run_forever(func: fn(mpsc::Sender<StatusMsg>), timeout: u64) -> ! {
    let (tx, rx) = mpsc::channel::<StatusMsg>();

    let tx2 = tx.clone();
    thread::spawn(move || {
        func(tx2);
    });

    loop {
        match rx.recv_timeout(Duration::from_secs(timeout)) {
            Ok(msg) => match msg {
                StatusMsg::Alive => {
                    continue;
                }
                StatusMsg::Dead => {
                    println!("Process signaled unrecoverable error. Restarting worker thread!");
                    let tx2 = tx.clone();
                    thread::spawn(move || {
                        func(tx2);
                    });
                }
            },
            Err(_) => {
                println!("Worker not responding. Restarting application!");
                exit(0);
            }
        }
    }
}
