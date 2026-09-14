// here we go

use std::thread::sleep;
use std::time::Duration;
use std::io::{self, Write};


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
    ];

    let readings = [
        "cursed",
        "alright",
        "haunted",
        "suspicious",
        "blessed",
    ];

    // let pick = number as usize % prophecies.len();

    // println!("prophecy: {}",prophecies[pick]);

    match number % 4 {
        0 => {
            let pick = number as usize % prophecies.len();
           type_out(&format!("prophecy: {}", prophecies[pick]));
        }

        1 => {
            let pick = number as usize % readings.len();
            type_out(&format!("reading: you are lowk {}",readings[pick]));
        }

        3 => {
            render_sigil(number);
        }

        _ => {
            let pick = number as usize % prophecies.len();
            let color = colors[number as usize % colors.len()];
            type_out(&format!("prophecy:{}{}{}",color, prophecies[pick], reset));
        }
    }

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
fn type_out(text: &str) {
    for ch in text.chars(){
        print!("{}", ch);
        io::stdout().flush().unwrap();
        sleep(Duration::from_millis(30));
    }
    println!();
}

