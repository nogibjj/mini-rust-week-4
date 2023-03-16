use clap::{App, Arg, SubCommand};
use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct UrlMapping {
    code: String,
    url: String,
}

fn main() {
    let matches = App::new("Simple URL Shortener")
        .version("0.1")
        .author("Your Name <your.email@example.com>")
        .about("Shortens URLs and retrieves original URLs using short codes")
        .subcommand(
            SubCommand::with_name("shorten")
                .about("Shortens a URL")
                .arg(
                    Arg::with_name("url")
                        .help("The URL to shorten")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("retrieve")
                .about("Retrieves the original URL using a short code")
                .arg(
                    Arg::with_name("code")
                        .help("The short code")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    let mut url_mappings = read_url_mappings();

    if let Some(matches) = matches.subcommand_matches("shorten") {
        let url = matches.value_of("url").unwrap().to_string();
        let code = generate_unique_code(&url_mappings);

        println!("Short code: {}", code);

        url_mappings.insert(code.clone(), UrlMapping { code, url });
        save_url_mappings(&url_mappings);
    } else if let Some(matches) = matches.subcommand_matches("retrieve") {
        let code = matches.value_of("code").unwrap();

        if let Some(url_mapping) = url_mappings.get(code) {
            println!("Original URL: {}", url_mapping.url);
        } else {
            println!("Short code not found");
        }
    }
}

fn generate_unique_code(url_mappings: &HashMap<String, UrlMapping>) -> String {
    let mut code;
    let mut rng = rand::thread_rng();

    loop {
        code = rng
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect::<String>();

        if !url_mappings.contains_key(&code) {
            break;
        }
    }

    code
}

fn read_url_mappings() -> HashMap<String, UrlMapping> {
    let path = Path::new("url_mappings.json");
    if !path.exists() {
        return HashMap::new();
    }

    let mut file = File::open(path).expect("Unable to open URL mappings file");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Unable to read URL mappings file");

    serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new())
}

fn save_url_mappings(url_mappings: &HashMap<String, UrlMapping>) {
    let data = serde_json::to_string(url_mappings).expect("Unable to serialize URL mappings");
    let mut file = Open
