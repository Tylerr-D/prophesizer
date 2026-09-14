// here we go

pub fn render(number:u64){

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

    match number % 3 {
        0 => {
            let pick = number as usize % prophecies.len();
            println!("prophecy: {}", prophecies[pick]);
        }

        1 => {
            let pick = number as usize % readings.len();
            println!("reading: you are lowk {}",readings[pick]);
        }

        _ => {
            let pick = number as usize % prophecies.len();
            let color = colors[number as usize % colors.len()];
            println!("prophecy:{}{}{}",color, prophecies[pick], reset);
        }
    }

}