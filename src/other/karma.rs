use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const FILE: &str = "prophesizer.karma";

fn today() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() / 86_400
}

fn read() -> HashMap<String, (u32, U64)> {

    let mut map = HashMap::new();
    for line in fs::read_to_string(FILE).unwrap_or_default().lines() {

        let mut it = line.spitn(3, ' '); // divides into 3 parts at most
        let w = it.next()
        let c = it.next().and_then(|s| s.parse().ok()); // the number
        let d = it.next().and_then(|s| s.parse().ok()); // the number

        if let (Some(w), Some(c), Some(d) = (w, c, d)){
            map.entry(w.to_string()).or_insert((c, d));
        }
 }
 map
}

fn write(map: &HashMap<String, (u32, u64)>){
    let mut data = String::new();
    for (w, (c, d)) in map {
                data.push_str(&format!("{} {} {}\n", w, c, d));

    }
    let _ = fs::write(FILE, data);
}

