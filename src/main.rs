extern crate skim;

use std::{fs, io::Cursor};
use ini::Ini;
use home::home_dir;
use skim::prelude::*;

fn main() {
    println!("AWS Profile Switcher");
    let home = home_dir().expect("Unable to get your home directory");

    let file_path = format!("{}/.aws/config", home.display());

    let profiles = Ini::load_from_file(file_path).unwrap();

    let mut profile_names = Vec::new();
    for (section, _) in profiles.iter() {
        match section {
            Some(section) => {
                let profile_name = section.split(" ").collect::<Vec<&str>>();
                profile_names.push(profile_name[1]);
            },
            None => {}
        }
    }

    let options = SkimOptionsBuilder::default()
        .prompt(Some("Choose a profile: "))
        .header(Some("Move up and down to reveal more options\n"))
        .height(Some("0%"))
        .multi(false)
        .build()
        .unwrap();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(profile_names.join("\n")));

    let selected_items = Skim::run_with(&options, Some(items))
            .map(|out| out.selected_items)
            .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        fs::write(format!("{}/.awsp", home.display()), item.output().to_string()).expect("Unable to write file");
    }
}
