use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use std::process::Command;

pub fn play_rick() {
    let fps = 10;

    let frame_ms = 1000 / fps;

    print!("\x1b[?1049h"); 
    io::stdout().flush().unwrap();


let _ = Command::new("mpv")
    .arg("--no-video")
    .arg("assets/rickroll.mp3")
    .spawn();

    for i in 1..=300 {
        
        let path = format!("assets/ascii_frames/frame_{:04}.txt",i);
        let text = match std::fs::read_to_string(&path){
            Ok(t) => t,
            Err(_) => break,
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
