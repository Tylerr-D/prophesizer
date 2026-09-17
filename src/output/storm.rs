use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

pub fn render_storm(number: u64) {

    let size = 5 + number % 4;

    let ch = ['*', '#', '+', '@'][((number >> 8) % 4) as usize];

    let junk =  ['?', '/', '~', '$', '%'];


    let mut cells = Vec::new();
    for row in 0..size {
        for col in 0..size {
            let bit = (number >> ((row * col + row + col) % 16)) & 1;

            if bit == 1 {
                cells.push((row as usize, col as usize));
            }
        }
    }

    print!("\x1b[2J\x1b[?25l");
    io::stdout().flush().unwrap();

    for col in 0..size {
        if (number >> (col % 16)) & 1 == 1 {
            print!("\x1b[{};{}H{}", 1, 1 + col * 2, ch);
        }
    }

    for t in 1..= size as usize {
        for &(row, col) in &cells {
            let screen_col = 1 + col * 2;

            if row >= t {
                if t > 1 {
                let bit_above = (number >> (((t - 1) * col + (t-1) + col) % 16)) & 1;
                    if bit_above == 1 {
                        print!("\x1b[{};{}H{}", t - 1, screen_col, ch);
                    } else {
                        print!("\x1b[{};{}H ", t - 1, screen_col)
                    } 
                }
                
                let j = junk[((number >> (row + col)) % 5) as usize];
                print!("\x1b[{};{}H{}", t, screen_col, j); 

                if row == t {
                    print!("\x1b[{};{}H{}", t, screen_col, ch); 
                }
            }
            
            io::stdout().flush().unwrap();
            sleep(Duration::from_millis(40)); 
        }
        print!("\x1b[?25h\n");
        io::stdout().flush().unwrap(); 
    }
}
