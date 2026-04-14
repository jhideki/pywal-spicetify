use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process;
pub struct Spicetify {
    config_path: PathBuf,
    theme: String,
}

fn is_spotify_running() -> bool {
    match process::Command::new("pgrep")
        .arg("-x")
        .arg("spotify")
        .output()
    {
        Ok(output) => output.status.success(),
        Err(_) => {
            match process::Command::new("ps")
                .arg("-C")
                .arg("spotify")
                .output()
            {
                Ok(output) => output.status.success(),
                Err(_) => false,
            }
        }
    }
}
impl Spicetify {
    pub fn new(home: PathBuf, theme: &str) -> Self {
        let mut config_path = home;
        config_path.push(format!(".config/spicetify/Themes/{theme}/color.ini"));
        if let Err(e) = fs::metadata(&config_path) {
            panic!("Error reading file {} {}", config_path.display(), e);
        }
        Self {
            config_path,
            theme: String::from(theme),
        }
    }

    pub fn reload(&self, use_no_restart_flag: bool) {
        process::Command::new("spicetify")
            .arg("config")
            .arg("current_theme")
            .arg(&self.theme)
            .status()
            .unwrap();
        process::Command::new("spicetify")
            .arg("config")
            .arg("color_scheme")
            .arg("pywal")
            .status()
            .unwrap();
        
        let mut apply_cmd = process::Command::new("spicetify");
        apply_cmd.arg("apply");
        
        if use_no_restart_flag && !is_spotify_running() {
            apply_cmd.arg("-n");
        }
        
        match apply_cmd.output() {
            Ok(stdout) => {
                println!("Running spicetify...");
                if let Ok(output) = String::from_utf8(stdout.stdout) {
                    println!("{output}");
                }
            }
            Err(e) => panic!("Error running spicetify apply {}", e),
        }
    }

    pub fn write_config(&self, wal_config: Option<String>) {
        let file = match File::open(&self.config_path) {
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
        let mut writer = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&self.config_path)
        {
            Ok(w) => w,
            Err(e) => panic!("Error opening file! {}", e),
        };

        let mut content = buf.join("\n");
        if let Some(wal_config) = wal_config {
            content.push_str("\n\n");
            content.push_str("[pywal]");
            content.push_str("\n");
            content.push_str(&wal_config);
        }

        let _ = writer.write_all(content.as_bytes());
    }
}
