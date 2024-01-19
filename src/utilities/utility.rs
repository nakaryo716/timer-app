use std::{
    thread::{self, sleep},
    time,
};

use proconio::input;

pub fn timer(time: usize) {
    let timer = thread::spawn(move || {
        for i in (0..time).rev() {
            sleep(time::Duration::from_secs(1));
            println!("{} s", i + 1);
        }
    });
    timer.join().unwrap();
}

pub fn input_stream() -> usize {
    input! {
        number: usize
    }
    number
}

pub fn select_mode_ui() {
    println!(
        "
Please choose a mode.
If you want to exit this application, please enter 0.
If you'd like to start counting, press any number key.
    "
    );
}
