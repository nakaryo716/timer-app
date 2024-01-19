use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

pub fn alarm() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = BufReader::new(File::open("assets/Alarm1.mp3").unwrap());

    let source = Decoder::new(file).unwrap();
    let source = source
        .take_duration(Duration::from_secs(5))
        .amplify(10.0);

    sink.append(source);

    sink.sleep_until_end();
}
