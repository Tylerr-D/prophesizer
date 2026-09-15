use std::io;
use std::io::Write;

// basically checks if input is a secret word,
// if no, then sends it back normally
// if yes, then go to secret outputs

// unfinished

pub(crate) fn parse(input: String) {
    if input.is_empty() {
        let gotten = get();
    }


}

// if input is empty, then asks user for input
fn get() -> String {
    print!("Input: ");

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_string()
}
