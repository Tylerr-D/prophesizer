use core::time;
use std::thread;

// umm yea fixed the imports

use super::sick::play_rick;
use super::storm::render_storm;
use super::type_out;

pub(crate) fn output_secret(output: &String) {
    secret_found();
    type_out(output);
    // todo!(
    //     "[RUSTER] replace print with your outputter"
    // )
}

pub(crate) fn output_variable_secret(outputs: &Vec<String>) {
    secret_found();
    // let outputs_available = outputs.len();

    // let smth: usize = rand::random_range(0..outputs_available-1);

    if outputs.len() == 1 {
        type_out(&outputs[0]);
        println!();
        return;
    }

    // println!("{}", outputs[smth]);
    // todo!(
    //     "[RUSTER] replace print with your outputter"
    // )

    let smth: usize = rand::random_range(0..outputs.len());
    type_out(&outputs[smth]);
    println!();
}

pub(crate) fn ascii_animation(input: String) {
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
    // [RUSTER] change print to your outputter, storm i think idk
    // oke

    type_out("you have found a secret");

    thread::sleep(time::Duration::from_secs(1));
}
