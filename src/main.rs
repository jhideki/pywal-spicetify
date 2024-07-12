use clap::{Arg, Command};
use std::{
    env,
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process,
};

fn apply_theme(theme: &str) {
    process::Command::new("spicetify")
        .arg("config")
        .arg("current_theme")
        .arg(&theme);
    process::Command::new("spicetify")
        .arg("config")
        .arg("color_scheme")
        .arg("pywal");
    match process::Command::new("spicetify").arg("apply").output() {
        Ok(stdout) => {
            println!("Running spicetify...");
            if let Ok(output) = String::from_utf8(stdout.stdout) {
                println!("{output}");
            }
        }
        Err(e) => panic!("Error running spicetify apply {}", e),
    }
}

fn wal_config(mut path: PathBuf) {
    path.push(".config/wal/templates/colors-spicetify.ini");
    if !path.exists() {
        let mut file = match OpenOptions::new().create(true).write(true).open(&path) {
            Ok(f) => f,
            Err(e) => panic!("Error opening .config/wal {}", e),
        };
        println!(
            "Generating colors-spicetify.ini file in {}",
            &path.display()
        );
        let content = r#"accent             = {color0.strip} 
accent-active      = {color2.strip} 
accent-inactive    = {color3.strip} 
banner             = {color4.strip} 
border-active      = {foreground.strip} 
border-inactive    = {foreground.strip} 
header             = {foreground.strip} 
highlight          = {color6.strip} 
main               = {background.strip} 
notification       = {color7.strip}
notification-error = {color8.strip} 
subtext            = {cursor.strip} 
text               = {cursor.strip}"#;
        let _ = file.write_all(content.as_bytes());
    }
    let _ = process::Command::new("wal")
        .arg("-w")
        .output()
        .expect("Failed run wal");
}

fn main() {
    let matches = Command::new("pywal-spicetify")
        .version("0.1")
        .about("A simple cli tool for setting spicetify colors from wal")
        .arg(Arg::new("theme").required(true))
        .arg(Arg::new("reset").short('r').long("reset"))
        .arg_required_else_help(true)
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

    wal_config(home.clone());
    let mut wal_path = home.clone();
    wal_path.push(".cache/wal/colors-spicetify.ini");
    let file = match File::open(&wal_path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening colors-spicetify.ini! {}", e),
    };

    println!("wal config path: {}", wal_path.display());

    let mut reader = BufReader::new(file);
    let mut wal_config = String::new();
    let _ = reader.read_to_string(&mut wal_config);

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
    content.push_str("[pywal]");
    content.push_str("\n");
    content.push_str(&wal_config);
    let _ = writer.write_all(content.as_bytes());

    apply_theme(&theme);
}
