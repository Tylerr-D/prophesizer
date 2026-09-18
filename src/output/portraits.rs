
use std::io::{self, Write};

// ok so i just convered it to ascii myself lol

const ZACH: &str = r#"
,'''''''''''......''''.'''''''',,,,;;;;:::cccllloo
'''''...............''..'''... .,,,,;;;:::cclllloo
..........................'..   ',,,;;::::cclllloo
..........................'.    .,,,;;;:::cclllloo
...........................     .'',,,;;;::ccllllo
...........................     .........';::cccll
...........................     ...       ';::cccl
...........................     ..       .,;:::ccl
...........................    ..       .';;:::ccl
...........................    '.    ..',,;;:::ccl
............................';lddollc,,,,,;;::::cl
..........................cxxxxxxddk0x;,,,;;:::ccl
'........................oxoc;'..  .;dl',,;;::::cc
'.....................'.,:,.         .l;.,,;::::cc
,'....................'.:..          .:l.',,;;::::
;,'''................',.d;.........,,;ll..,,,;;;;:
;;,,,'................;.lc''..........''..'',,;;;:
;;;,,,,''.............,'...    .. .   ....''',,;;:
;;;;;,,,,'''...........'..        ..   ...'',,;;::
;;;;;;;,,,,,''..........''.        ....'..'',;;::c
;;;;;,,,,,,,''''.........;:..       ..,'.'',;;::cc
;;;;;;,,,,''''''''.......':;;,.    ..;,.'',,;:::cc
;;;;;;;,,,,,'''''''''''.;oo...,,...,;..''',,;;:c:o
,,;;;;;;,,,,,,,,,,,;ccloxxd:    ...,cl;...........
,,,,;;;,,,''''''',:oxOOOOkxo'      .,lxl.         
...............'lxOOOOkkkkkxl.    ..,dxOOl'.      
............'cxOOkkkkkkkkkkkxc. ;doox0dx000Ooc,.  
..........'cxkOkkkkkkkxxxdxkOkd. cXNNK'.xkOOOOkx:.
.........;dOOOkkxxxxxxxkxxxxkkOkc.;ONNc .dxkOkxdx:
........,xOOOkkkkxxxxxxxxO0OO0OOOx..oX0. ,xkxxxxxd
.......'ckOkkkkxkkxddxxxxkOO0000OOk,.lNx..okxxxxxx
'''....,oOOOkkkkkOxddddddddxxxkkOkk0c.cKl 'ddxxxkk
'''...';dOOkOkkOOkkddddooooddodxxxxOX:.:0'.:dxxxkk
"#;

// add more portraits here
// in form of:
//
// const PORTRAIT_NAME: &str = r#"
// (portrait image)
// "
//

pub fn show_portrait(word: &str) -> bool {
    match word {
        "zach" | "zrl" | "goat" | "Goat" | "ceo" | "chill" | "genius" | "thebigleagues" => {
            println!();
            println!("zach latta himself");
            println!();
            print!("{}", ZACH);
            let _ = io::stdout().flush();
            true
        }

        // add more portraits here
        // in this form:
        //
        // "word" => {
        //     print!("{}", PORTRAIT_NAME);
        //     let _ = io::stddout().flush();
        //     true
        // }
        //
        _ => false,
    }
}
