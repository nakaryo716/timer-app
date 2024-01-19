use rodio::{Decoder, OutputStream, Sink, Source};
use std::error;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::Duration;

pub fn alarm(path: &PathBuf) -> Result<(), Box<dyn error::Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // issue
    // whene I run this application timer-app directory, I should pass path
    // "/Users/user-name/projects/timer-app/assets/Alarm1.mp3"
    // but, Whene I run this application unix executable file directory, I must pass the path
    // "../../assets/Alarm1.mp3" relative path
    let file = BufReader::new(File::open(path)?);

    let source = Decoder::new(file)?;
    let source = source.take_duration(Duration::from_secs(6)).amplify(10.0);

    sink.append(source);

    sink.sleep_until_end();
    Ok(())
}
