# Timer application on the console.  
## Discription
This is timer application runnning on command line.  
The development language for this application is Rust.Due to this, you can run the application without the need to install a compiler or interpreter.
## Run
cd timer-app root directory.
```
cd timer-app
```
 Then, please execute the following command.
```
 ./target/debug/timer-app 
```
Alternatively, if rustc and cargo are available on your computer,please execute the following command.
```
cargo run
```

## Project Dependcies
I used this crate.
- proconio &emsp; https://github.com/statiolake/proconio-rs
- rodio &emsp; https://github.com/RustAudio/rodio
- tracing &emsp; https://github.com/tokio-rs/tracing
- tracing-subscriber
