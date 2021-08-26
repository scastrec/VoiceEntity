# Voice entity
Create voice entity json file for Alexa and Dialogflow from csv.

## Run
`./lib/release/voice_entitiy test.csv`

## Develop
1. Build: `cargo build`
2. Run: `./target/debug/voice_entitiy test.csv`
3. Build for prod: `cargo build --bin voice_entitiy --release --target-dir lib`
