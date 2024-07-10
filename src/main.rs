use clap::{arg, command, value_parser, ArgAction, Command};
use std::{
    env,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Seek, Write},
    path::Path,
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
    println!("{theme}");

    let home = match env::home_dir() {
        Some(path) => path,
        None => panic!("unable to locate home directory!"),
    };
    let mut wal_path = home.clone();
    wal_path.push(".cache/wal/colors.json");
    let file = match File::open(wal_path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening colors.json! {}", e),
    };

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

    let mut writer = match OpenOptions::new().append(true).open(&path) {
        Ok(w) => w,
        Err(e) => panic!("Error opening file! {}", e),
    };

    let line_numb: Vec<usize> = reader
        .lines()
        .enumerate()
        .filter(|(_, l)| {
            if let Ok(s) = l {
                return s.contains("pywal");
            }
            false
        })
        .map(|(i, _)| i)
        .collect();

    let content = format!(
        r#"[pywal]
accent             = #000000 
accent-active      = #000000 
accent-inactive    = #000000 
banner             = #000000 
border-active      = #{foreground} 
border-inactive    = #{foreground} 
header             = #000000 
highlight          = #000000 
main               ={background} 
notification       = #000000
notification-error = #000000
subtext            = #{cursor} 
text               = {cursor}
"#,
        background = config.special.background,
        cursor = config.special.cursor,
        foreground = config.special.foreground,
    );
    println!("{}", content);

    if line_numb.is_empty() {
        let _ = writer.write_all(content.as_bytes());
    } else {
        println!("{}", line_numb[0]);
    }

    //TODO: read data from wal_path, write data to colo.ini starting from line number
}
