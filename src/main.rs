use std::env;
use std::fs::File;
use std::error::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EntitiyValue {
    id: String,
    value: String,
    synonyms: Vec<String>,
}
struct Entitiy {
    value: Vec<EntitiyValue>
}

fn main() {
    let path = env::args().nth(1).expect("No csv file! The first argument should be a csv file path");
    //let has_header = env::args().nth(2) || false;
    if check_path(&path) {
        let file = File::open(path)
        .expect("Something went wrong reading the csv file");
        if let Err(err) = manage_csv_content(file) {
            println!("Error parsing csv file - {}", err);
        }
    } else {
        println!("No csv file! The first argument should be a csv file path");
    }
}

fn manage_csv_content(file: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .from_reader(file);
    for result in rdr.deserialize() {
        let record: EntitiyValue = result?;
        println!("{:?}", record);
    }
    Ok(())
}


fn is_csv(path: &String) -> bool {
    return path.ends_with(".csv");
}

fn check_path(path: &String) -> bool {
    return is_csv(path);
}
