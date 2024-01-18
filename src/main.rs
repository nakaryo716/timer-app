use core::time;
use std::{sync::{mpsc, atomic::AtomicUsize}, thread::{self, sleep}, env};
use std::sync::atomic::Ordering;
use proconio::input;
use tracing::debug;

// RUST_LOG=debug cargo run

fn main() {
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();
    

    let (tx, rx) = mpsc::channel();
    static TIME: AtomicUsize = AtomicUsize::new(0);


    let init_thread = thread::spawn(move || {
        loop {
            input! {
                mode: usize,
            }

            if mode == 0{
                tx.send(false).unwrap();
                break;
            }

            input! {
                input_time: usize,
            }

            TIME.store(input_time, Ordering::Release);
            tx.send(true).unwrap();

            debug!("inputed {}", input_time);
        }
    });

    let timer_thread = thread::spawn(move || {
        while rx.recv().unwrap() {
            debug!("start timer thread");
            let selected = TIME.load(Ordering::Acquire);
            
            debug!("timer thread got {} s", selected);

            if selected == 0 {
                continue;
            } else {
                debug!("do count");
                // write timer logic and sound
                sleep(time::Duration::from_secs(selected as u64));
                debug!("finish count");
            }
        }
        debug!("finish timer thread");
    });

    init_thread.join().unwrap();
    timer_thread.join().unwrap();
}