use std::env;
use std::fs::File;
use std::io::Write;
use std::error::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct EntityValue {
    id: String,
    value: String,
    synonyms: Vec<String>,
}
struct Entitiy {
    values: Vec<EntityValue>
}

#[derive(Debug, Serialize, Deserialize)]
struct DialogflowEntityValue {
    value: String,
    synonyms: Vec<String>,
}

#[derive(Serialize)]
struct AlexaValues {
    value: String,
    synonyms: Vec<String>,

}

#[derive(Serialize)]
struct AlexaEntityValue {
    id: String,
    name: AlexaValues,
}

fn main() {
    let path = env::args().nth(1).expect("No csv file! The first argument should be a csv file path");
    //let has_header = env::args().nth(2) || false;
    if check_path(&path) {
        if let Err(err) = manage_csv_content(&path) {
            println!("Error parsing csv file - {}", err);
        }
    } else {
        println!("No csv file! The first argument should be a csv file path");
    }
}

fn manage_csv_content(path: &String) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)
    .expect("Something went wrong reading the csv file");

    // FIRST decode the file
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b',')
        .double_quote(false)
        .escape(Some(b'\\'))
        .flexible(true)
        .from_reader(file); //TODO use from_path
    let mut values: Vec<EntityValue>= Vec::new();
    // deserialize it in structure
    for result in rdr.deserialize() {
        let record: EntityValue = result?;
        values.push(record);
    }
    let entity: Entitiy = Entitiy {
        values: values
    };

    // map to adequat struct
    let df_entity = create_dialogflow_json(&entity);
    println!("df_entity - {:?}", df_entity);
    let alexa_entity = create_alexa_json(&entity);

    // parse json from struct
    let df_json = serde_json::to_string(&df_entity)?;
    let alexa_json = serde_json::to_string(&alexa_entity)?;
    
    // save as files
    // Open a file in write-only mode, returns `io::Result<File>`
    let df_path = path.clone().replace(".csv", &"_df.json");
    let alexa_path = path.clone().replace(".csv", &"_alexa.json");

    write_file(df_path, df_json);
    write_file(alexa_path, alexa_json);

    Ok(())
}

fn write_file(path: String, content: String) {
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path, why),
        Ok(file) => file,
    };
    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write(content.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path, why),
        Ok(_) => println!("successfully wrote to {}", path),
    }
}

fn create_dialogflow_json(entity: &Entitiy) -> Vec<DialogflowEntityValue> {
    let mut df_entities: Vec<DialogflowEntityValue> = Vec::new();
    for line in entity.values.iter() {
        let mut synonyms = line.synonyms.to_vec();
        synonyms.push(line.value.clone());
        df_entities.push(DialogflowEntityValue{
            synonyms: synonyms,
            value: line.id.clone(),     
        })
    } 
    return df_entities;    
}


fn create_alexa_json(entity: &Entitiy) -> Vec<AlexaEntityValue> {
    let mut alexa_entities: Vec<AlexaEntityValue> = Vec::new();
    for line in entity.values.iter() {
        let mut synonyms = line.synonyms.to_vec();
        synonyms.push(line.value.clone());
        alexa_entities.push(AlexaEntityValue{
            id: line.id.clone(),     
            name: AlexaValues{
                synonyms: synonyms,
                value: line.value.clone()
            }
        })
    } 
    return alexa_entities;    
   
}


fn is_csv(path: &String) -> bool {
    return path.ends_with(".csv");
}

fn check_path(path: &String) -> bool {
    return is_csv(path);
}
