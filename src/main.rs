use clap::{arg, command, value_parser, ArgAction, Command};
use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

mod data;
use data::Config;

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

    //Only linux support
    let home = match env::home_dir() {
        Some(path) => path,
        None => panic!("unable to locate home directory!"),
    };

    let mut wal_path = home.clone();
    wal_path.push(".cache/wal/colors.json");
    let file = match File::open(&wal_path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening colors.json! {}", e),
    };

    println!("wal config path: {}", wal_path.display());

    let reader = BufReader::new(file);

    let config: Config = match serde_json::from_reader(reader) {
        Ok(c) => c,
        Err(e) => panic!("Error deserializing colors.json! {}", e),
    };

    let mut path = home.clone();
    path.push(format!(".config/spicetify/Themes/{theme}/color.ini"));

    println!("spicetify config path: {}", path.display());

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => panic!("Invalid path: {e}"),
    };

    let reader = BufReader::new(file);

    let lines: Vec<String> = match reader.lines().collect() {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading lines! {}", e),
    };

    let mut buf: Vec<String> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        //Remove exisitng config
        if lines[i].contains("pywal") {
            buf.pop(); //pop the last \n
            i += 14;
            continue;
        }
        buf.push(lines[i].clone());
        i += 1;
    }
    let mut writer = match OpenOptions::new().write(true).truncate(true).open(&path) {
        Ok(w) => w,
        Err(e) => panic!("Error opening file! {}", e),
    };

    let mut content = buf.join("\n");
    content.push_str("\n\n");
    content.push_str(&format!(
        r#"[pywal]
accent             = 000000 
accent-active      = 000000 
accent-inactive    = 000000 
banner             = 000000 
border-active      = {foreground} 
border-inactive    = {foreground} 
header             = 000000 
highlight          = 000000 
main               = {background} 
notification       = 000000
notification-error = 000000
subtext            = {cursor} 
text               = {cursor}"#,
        background = &config.special.background[1..],
        cursor = &config.special.cursor[1..],
        foreground = &config.special.foreground[1..],
    ));

    let _ = writer.write_all(content.as_bytes());
}
