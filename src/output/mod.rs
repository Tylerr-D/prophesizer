// here we go

pub fn render(number:u64){
    let prophecies = [
    "wow, wow, wow, wow",
    "tonight is a night",
    "42 is what it means",
    "looks good in the hood",
    "you are going for it",
    ];

    let pick = number as usize % prophecies.len();

    println!("prophecy: {}",prophecies[pick]);

}