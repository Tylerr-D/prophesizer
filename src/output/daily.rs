use std::time::SystemTime;
use std::time::UNIX_EPOCH;


pub fn show_daily(input: Option<&str>) {
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let days = secs / 86400;

    let word = input.unwrap_or("");

    let weekday = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"][(days % 7) as usize];

    let total = crate::algs::process_word(&*word) + days;

    // aa i lowk got no good things to add lol
    // im lowk adding anything atp

    let luck = ["high", "medium", "low", "cursed", "none"];

    let danger = ["mild", "moderate", "run", "?"];

    let advice = ["do not trust the terminal", "eat something", "no", "stand up for once"];

    println!("daily reading for {} ({}):", word, weekday);
    println!("luck: {}", luck[(total % luck.len() as u64) as usize]);
    println!("danger: {}", danger[(total % danger.len() as u64) as usize]);
    println!("advice: {}", advice[(total % advice.len() as u64) as usize]);

}
