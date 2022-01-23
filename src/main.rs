use std::io::{stdin, BufReader};
use std::fs::File;
use std::time::{Instant};
use rodio::{Decoder, OutputStream, source::Source};

fn beat(bpm: i32) {
    // Set vars
    let mut count: i32 = 0;
    let millis_per_beat = 60000.0/(bpm as f32);
    let mut beat_done = false;
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut beat = Instant::now();
    // Start loop
    loop {
        let elapsed_time = beat.elapsed();
        if count % 2 == 0 && !beat_done {
            println!("Kick");
            let kick_raw = BufReader::new(File::open("kick.ogg").unwrap());
            let kick_source = Decoder::new(kick_raw).unwrap();
            stream_handle.play_raw(kick_source.convert_samples()).expect("Couldn't convert");
            beat_done = true;
        } else if !beat_done {
            println!("Snare");
            let snare_raw = BufReader::new(File::open("snare.ogg").unwrap());
            let snare_source = Decoder::new(snare_raw).unwrap();
            stream_handle.play_raw(snare_source.convert_samples()).expect("Couldn't convert");
            beat_done = true;
        }
        if elapsed_time.as_millis() >= (millis_per_beat as u128) {
            count += 1;
            if count % 2 == 0 {
                count = 0;
            }
            beat_done = false;
            beat = Instant::now();
        }
    }
}

fn main() {
    let mut input = String::new();
    println!("Welcome to BeatBuddy. Enter desired bpm.");
    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let bpm: i32 = input.trim().parse().expect("Invalid bpm");
    beat(bpm);
}
