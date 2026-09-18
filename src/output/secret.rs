use core::time;
use std::thread;

// umm yea fixed the imports

use super::sick::play_rick;
use super::storm::render_storm;

use crate::output::type_out;

pub(crate) fn output_secret(output: &String) {
    secret_found();
    type_out(output, true);

    // todo!(
    //     "[RUSTER] replace print with your outputter"
    // )
}

pub(crate) fn output_variable_secret(outputs: &Vec<String>) {
    secret_found();

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).unwrap()
        .as_nanos();
    let smth = (now % outputs.len() as u128) as usize;

    type_out(&outputs[smth], true);
    println!();
}

pub(crate) fn ascii_animation(_input: String) {
    secret_found();

    play_rick();

    // todo!(
    //     "[RUSTER] call ascii output displayer"
    // )
}

pub(crate) fn ascii_art(input: String) {
    secret_found();

    let number: u64 = input.trim().parse().unwrap_or(0);
    render_storm(number);
    println!();

    // todo!(
    //     "[RUSTER] call ascii output displayer"
    // )
}

fn secret_found() {
    // [RUSTER] change print to your outputter
    // oke

    // tells user that they just had discovered a secret, then clearing that.
    type_out("you have found a secret", false);
    thread::sleep(time::Duration::from_millis(500));
    print!("\r\x1B[K")
}
