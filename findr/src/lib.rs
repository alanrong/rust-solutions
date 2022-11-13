use crate::EntryType::*;
use clap::{App, Arg};
use regex::Regex;
use std::error::Error;
use walkdir::WalkDir;
use walkdir::DirEntry;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("findr")
        .version("0.1.0")
        .author("alanrong <alanrong@gmail.com>")
        .about("Rust find")
        .arg(
            Arg::with_name("path")
                .value_name("PATH")
                .help("Search paths")
                .multiple(true)
                .default_value(".")
        )
        .arg(
            Arg::with_name("names")
                .short("n")
                .long("name")
                .value_name("NAME")
                .help("Name")
                .takes_value(true)
                .multiple(true)
        )
        .arg(
            Arg::with_name("types")
                .short("t")
                .long("type")
                .value_name("TYPE")
                .possible_values(&["d", "f", "l"])
                .help("Entry type")
                .takes_value(true)
                .multiple(true)
        )
        .get_matches();

    let names = matches
        .values_of_lossy("names")
        .map(|vals| {
            vals.into_iter()
                .map(|name| {
                    Regex::new(&name)
                        .map_err(|_| format!("Invalid --name \"{}\"", name))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?
        .unwrap_or_default();

    let entry_types = matches
        .values_of_lossy("types")
        .map(|vals| {
            vals.iter()
                .map(|val| match val.as_str() {
                    "d" => Dir,
                    "f" => File,
                    "l" => Link,
                    _ => unreachable!("Invalid type"),
                })
                .collect()
        })
        .unwrap_or_default();
    

    Ok(Config {
        paths: matches.values_of_lossy("path").unwrap(),
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    Link => entry.path().is_symlink(),
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file(),
                })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        
        println!("{}", entries.join("\n"));
    }
    Ok(())
}