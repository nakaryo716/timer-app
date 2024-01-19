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
            let mode = input_stream();

            if mode == 0{
                tx.send(false).unwrap();
                break;
            }

           
            let input_time = input_stream();
            

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
                timer2(selected);
                println!("Time up!");
            }
        }
        debug!("finish timer thread");
    });

    init_thread.join().unwrap();
    timer_thread.join().unwrap();
}



fn timer2(time: usize) {
    let timer = thread::spawn(move || {
        for i in 0..time {
            sleep(time::Duration::from_secs(1));
            println!("{} s", i + 1);
        }
    });
    timer.join().unwrap();
}

fn input_stream() -> usize {
    input! {
        number: usize,
    }
    number
}