use std::io;
use std::time::Instant;

fn beat(bpm: i32) {
    // Set vars
    let mut count: i32 = 0;
    let millis_per_beat = 60000/bpm;
    let mut beat_done = false;
    let mut beat = Instant::now();
    // Start loop
    loop {
        let mut elapsed_time = beat.elapsed();
        if count % 2 == 0 && !beat_done {
            println!("Kick");
            beat_done = true;
        } else if !beat_done {
            println!("Snare");
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
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let bpm: i32 = input.trim().parse().expect("Invalid bpm");
    beat(bpm);
}
