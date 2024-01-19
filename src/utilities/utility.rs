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
        number: usize,
    }
    number
}

pub fn select_mode_ui() {
    println!(
        "
Select mode.
If you exit this app, you input 0.
If, you want count time, you input any numver key.     
    "
    );
}
