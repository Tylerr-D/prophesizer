use std::io::{self, Write, Cursor};
use std::thread::sleep;
use std::time::Duration;

use include_dir::{include_dir, Dir};
use rodio::{Decoder, DeviceSinkBuilder, Player};

pub static RICK_AUDIO: &[u8] = include_bytes!("../../assets/rickroll.mp3");

static RICK_FRAMES: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets/rick_frames");

pub fn play_rick() {
    let fps = 10;
    let frame_ms = 1000 / fps;

    print!("\x1b[?1049h"); 
    io::stdout().flush().unwrap();


    let rickrolling = DeviceSinkBuilder::open_default_sink()
        .expect("Failed to open audio device");

    let cursor = Cursor::new(RICK_AUDIO);
    let source = Decoder::new(cursor).expect("Failed to decode");

    let player = Player::connect_new(rickrolling.mixer());
    player.append(source);

    for i in 1..=300 {
        let path = format!("frame_{:04}.txt", i);
        let text = match RICK_FRAMES.get_file(&path){
            Some(file) => match file.contents_utf8() {
                Some(info) => info,
                None => break,
            }
            None => break,
        };


        print!("\x1b[2J\x1b[H");
        print!("{}", text);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(frame_ms));
    }

    print!("\x1b[?1049l");
    io::stdout().flush().unwrap();
    println!("you just got prophesized. never gonna give you up.");
}
