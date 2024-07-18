use clap::Parser;
use rocket::{build, get, launch, routes, serde::json::Json, State};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::prelude::*;
use rocket::figment::value::Dict;

#[derive(Parser)]
struct Config {
    #[arg(short, long, default_value = "dictionary.yaml")]
    filename: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Example {
    ll: String,
    en: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Classification {
    #[serde(rename = "type")]
    typ: String,
    subtype: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Word {
    head: String,
    classification: Classification,
    definition: String,
    examples: Option<Vec<Example>>,
}

#[derive(Serialize, Deserialize)]
struct Dictionary {
    dictionary: Vec<Word>,
}

fn load_dictionary(config: &Config) -> Vec<Word> {
    let mut file = File::open(&config.filename).expect("failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("failed to read file");
    let dictionary: Dictionary = serde_yaml::from_str(&contents)
        .expect("failed to parse yaml");
    dictionary.dictionary
}


#[get("/word")]
fn get_words(dictionary: &State<Vec<Word>>) -> Json<&Vec<Word>> {
    Json(dictionary)
}

#[get("/word/<word>")]
fn get_word(dictionary: &State<Vec<Word>>, word: &str) -> Option<Json<Word>> {
    for w in dictionary.iter() {
        if w.head == word {
            return Some(Json(w.clone()))
        }
    }
    None
}

#[launch]
fn rocket() -> _ {
    let config = Config::parse();
    build()
        .manage(load_dictionary(&config))
        .mount("/", routes![get_words, get_word])
}
