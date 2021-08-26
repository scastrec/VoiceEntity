# Voice entity
Create voice entity json file for Alexa and Dialogflow from csv.

## Run
`./lib/release/voice_entitiy test.csv`

## Develop
1. Build: `cargo build`
2. Run: `./target/debug/voice_entitiy test.csv`
3. Build for prod: `cargo build --bin voice_entitiy --release --target-dir lib`

## Dependencies
* csv : use to read CSV
* serde : use to deserialize csn into Rust Struct 
* serde_json : use to serialize struct to json

## TODO
* Add more options as input (activate headers, output file, ...)