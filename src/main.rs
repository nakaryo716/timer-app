use std::sync::atomic::Ordering;
use std::{
    env,
    sync::{atomic::AtomicUsize, mpsc},
    thread,
};
use tracing::debug;
use utilities::utility::{input_stream, timer, select_mode_ui};

mod utilities;

// If you want to check how dose it work inner thread,
// Please execute the following command.
// RUST_LOG=debug cargo run

fn main() {
    // Setting log environment.
    let log_level = env::var("RUST_LOG").unwrap_or("info".to_string());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::channel();
    static TIME: AtomicUsize = AtomicUsize::new(0);

    // This thread is monitoring whether to execute or stop the application.
    let init_thread = thread::spawn(move || {
        select_mode_ui();
        loop {
            let mode = input_stream();

            if mode == 0 {
                tx.send(false).unwrap();
                break;
            }

            println!("How many seconds would you like me to count?");
            let input_time = input_stream();
            println!("-------------------------------------");

            TIME.store(input_time, Ordering::Release);
            tx.send(true).unwrap();
        }
        debug!("exit init_thread loop");
    });

    // This thread represents the logic for the timer.
    let timer_thread = thread::spawn(move || {
        while rx.recv().unwrap() {
            debug!("start timer thread");

            let selected = TIME.load(Ordering::Acquire);
            debug!("timer thread got {} s", selected);

            if selected == 0 {
                select_mode_ui();
                continue;
            } else {
                timer(selected);
                println!("Time up!");
                select_mode_ui();
            }
        }
        debug!("exit timer thread loop");
    });

    init_thread.join().unwrap();
    timer_thread.join().unwrap();
}
