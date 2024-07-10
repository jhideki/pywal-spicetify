use clap::{arg, command, value_parser, ArgAction, Command};
use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader},
};

mod data;
use data::Colors;

fn main() {
    let matches = Command::new("pywal-spicetify")
        .version("0.1")
        .about("A simple cli tool for setting spicetify colors from wal")
        .arg(arg!(--theme <value>).required(true))
        .get_matches();
    let theme: String = matches
        .get_one::<String>("theme")
        .expect("theme required")
        .to_string();
    println!("{theme}");

    let wal_path = "~/.cache/wal/colors.json";

    let path = format!("~/.config/spicetify/Themes/{theme}/color.ini");

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("{e}"),
    };

    let reader = BufReader::new(file);

    let mut writer = OpenOptions::new().read(true).write(true).open(&path);
    let line_numb: Vec<usize> = reader
        .lines()
        .enumerate()
        .filter(|(_, l)| {
            if let Ok(s) = l {
                return s.contains("pywal-spicetify");
            }
            false
        })
        .map(|(i, _)| i)
        .collect();

    //TODO: read data from wal_path, write data to colo.ini starting from line number
}
