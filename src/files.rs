use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn save(map: &HashMap<String, String>) {
    //Save
    let json = match serde_json::to_string(&map) {
        Ok(j) => j,
        Err(e) => {
            panic!("Failed to Serialize: {}", e)
        }
    };

    if let Err(e) = std::fs::write("db.json", json) {
        panic!("Failed to write: {}", e)
    }
}

pub fn load() -> HashMap<String, String> {
    let file = match File::open("db.json") {
        Ok(f) => f,
        Err(e) => {
            panic!("File doesn't exist: {}", e)
        }
    };
    let reader = BufReader::new(file);

    let data : HashMap<String, String> = match serde_json::from_reader(reader){
        Ok(map) => map,
        Err(_) => HashMap::new(),
    };

    return data
}