use std::fs::OpenOptions;
use std::io::{self, prelude, BufRead};
use std::{fs::File, io::Write};

use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitIgnore {
    pub name: String,
    pub source: String,
}

fn main() {
    let mut body: Vec<String> = match ureq::get("https://api.github.com/gitignore/templates").call()
    {
        Ok(res) => res.into_json().unwrap(),
        Err(error) => panic!("Unable to fetch licenses: {}", error),
    };
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your language/template:")
        .default(0)
        .items(&body[..])
        .interact()
        .unwrap();

    println!("Enjoy your {}!", body[selection]);
    let url = format!(
        "https://api.github.com/gitignore/templates/{}",
        body[selection]
    );
    let mut body: GitIgnore = match ureq::get(&url).call() {
        Ok(res) => res.into_json().unwrap(),
        Err(error) => panic!("Unable to fetch licenses: {}", error),
    };
    print!("{}", body.source);
    let filename = "gitignore.demo";
    let file = match std::fs::metadata(&filename) {
        Ok(_) => {
            let mut input = String::new();
            let stdin = io::stdin();
            println!("File already exists. Do you want to (a)ppend, (o)verwrite, or (i)gnore?");
            stdin.lock().read_line(&mut input).unwrap();
            match input.trim().to_lowercase().as_str() {
                "a" => OpenOptions::new().append(true).open(&filename).unwrap(),
                "o" => File::create(&filename).unwrap(),
                _ => return,
            }
        }
        Err(_) => File::create(&filename).unwrap(),
    };
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(body.source.as_bytes()).unwrap();
}
