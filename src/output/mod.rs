// here we go


mod storm;
pub use storm::render_storm;
pub mod sick;
pub mod secret;
pub mod portraits;
pub mod daily;
pub mod history;


use std::f64::consts::PI;
use std::fs::File;
use std::io::{self, Write};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;


pub fn render(number:u64){

    flicker();

    // umm these are js colors lol
    let colors = ["\x1b[31m", "\x1b[32m", "\x1b[33m", "\x1b[34m", "\x1b[35m"];
    let reset = "\x1b[0m";

    let prophecies = [
    "wow, wow, wow, wow",
    "tonight is the night",
    "42 is what it means",
    "looks good in the hood",
    "you are going for it",
    "think about the moon",
    "what if it didnt happen",
    
    ];

    let readings = [
        "cursed",
        "alright",
        "haunted",
        "suspicious",
        "blessed",
        "cool",
        "cold",
    ];

    // let pick = number as usize % prophecies.len();

    // println!("prophecy: {}",prophecies[pick]);

    match number % 4 {
        0 => {
            let pick = number as usize % prophecies.len();
           type_out(&format!("prophecy: {}", prophecies[pick]), true);
        }

        1 => {
            let pick = number as usize % readings.len();
            type_out(&format!("reading: you are lowk {}", readings[pick]), true);
        }

        3 => {
            render_storm(number);
        }

        _ => {
            let pick = number as usize % prophecies.len();
            let color = colors[number as usize % colors.len()];
            type_out(&format!("prophecy:{}{}{}", color, prophecies[pick], reset), true);
        }
    }

    play_melody(number);

}

pub fn render_sigil(number: u64){
    let size = 5 + number % 4;
let ch = ['*', '#', '+', '@'][((number >> 8) % 4) as usize];

    for row in 0..size {
        let mut line = String::new();
        for col in 0..size {
            let bit = (number >> ((row * col + row + col) % 16)) & 1;

            if bit == 1 {
                line.push(ch);

            }

            else {
                line.push(' ');

            }

            line.push(' ');

        }

        println!("{}", line);
    }
}

fn rand_small() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().subsec_nanos() as u64
}

fn flicker() {
    // holy symbols
    let junk = ['%', '#', '&', '@', '$', '?', '/', '\\', '*', '~'];

    for _ in 0..8 {
        let mut line = String::new();
        for _ in 0..14 {
            line.push(junk[(rand_small() % 10) as usize]);
            line.push(' ');
        }

        print!("{}\r", line);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(60));
    }

    print!("                              \r");
    io::stdout().flush().unwrap();
    println!();
}


// i hope this looks fine lol
fn type_out(text: &str, new_line: bool) {
    for ch in text.chars(){
        print!("{}", ch);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(30));
    }
    
    if new_line {
        println!();
    }
}

pub fn play_melody(number:u64) {
    // these are js c major scales
    let scale: [f64; 8] = [261.63, 293.66, 329.63, 349.23, 392.00, 440.00, 493.88, 523.25];

    let digits: Vec<usize> = number
    .to_string()
    .chars()
    .map(|c| (c as u8 - b'0') as usize)
    .collect();

    let sample_rate = 44100u32;
    
    let note_len = 0.15;

    let mut samples: Vec<f64> = Vec::new();

    for &d in &digits {
        let freq = scale[d % 8];

        let n = (sample_rate as f64 * note_len) as usize;

        for i in 0..n {
            let t = i as f64 / sample_rate as f64;
            let fade = 1.0 - (i as f64 / n as f64);

            samples.push((freq * 2.0 * PI * t).sin() * 0.3 * fade);
            // this is the actual music lol
        }
    }

    write_wav(&samples, sample_rate);
    play_wav();

}

fn write_wav(samples: &[f64], sample_rate: u32) {

    let mut wav: Vec<u8> = Vec::new();


    let data_len = (samples.len() * 2 + 36) as u32;

    // some tuff shi?
    // lowk had to search how to covert it 
        wav.extend_from_slice(b"RIFF");
        wav.extend_from_slice(&data_len.to_le_bytes());
        wav.extend_from_slice(b"WAVEfmt ");
    wav.extend_from_slice(&16u32.to_le_bytes());
        wav.extend_from_slice(&1u16.to_le_bytes()); 
    wav.extend_from_slice(&1u16.to_le_bytes());
        wav.extend_from_slice(&sample_rate.to_le_bytes());
        wav.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes());
    wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data");
    wav.extend_from_slice(&((samples.len() * 2) as u32).to_le_bytes());

    for s in samples {
wav.extend_from_slice(&((*s * 32767.0) as i16).to_le_bytes());
    }

    let mut f = File::create("/tmp/prophesizer_tune.wav").unwrap();
    f.write_all(&wav).unwrap();

}

fn play_wav(){
    let player = if cfg!(target_os = "macos") {
        "afplay"
    }

    else {
        "aplay"
    };

 let _ = Command::new(player).arg("/tmp/prophesizer_tune.wav").output();
}


