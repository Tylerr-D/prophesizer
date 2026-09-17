use core::time;
use std::thread;
use rand;

pub(crate) fn output_secret(output: &String) {
    secret_found();
    println!("{}", output);
    todo!(
        "[RUSTER] replace print with your outputter"
    )
}

pub(crate) fn output_variable_secret(outputs: &Vec<String>) {
    secret_found();
    let outputs_available = outputs.len();

    let smth: usize = rand::random_range(0..outputs_available-1);

    println!("{}", outputs[smth]);
    todo!(
        "[RUSTER] replace print with your outputter"
    )
}

pub(crate) fn ascii_animation(input: String) {
    secret_found();

    todo!(
        "[RUSTER] call ascii output displayer"
    )
}

pub(crate) fn ascii_art(input: String) {
    secret_found();

    todo!(
        "[RUSTER] call ascii output displayer"
    )
}

fn secret_found() {
    // [RUSTER] change print to your outputter, storm i think idk
    println!("You have found a secret!");

    thread::sleep(time::Duration::from_secs(1));
}
