use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::path::PathBuf;
use std::process;

pub struct Wal {
    config_path: PathBuf,
    cache_path: PathBuf,
}
impl Wal {
    pub fn new(home: PathBuf) -> Self {
        let mut config_path = home.clone();
        let mut cache_path = home.clone();
        config_path.push(".config/wal/templates/colors-spicetify.ini");
        cache_path.push(".cache/wal/colors-spicetify.ini");
        Wal {
            config_path,
            cache_path,
        }
    }

    pub fn reload(&self) {
        let _ = process::Command::new("wal")
            .arg("-w")
            .output()
            .expect("Failed run wal");
    }

    pub fn reset(&self) {
        if !self.config_path.exists() || !self.cache_path.exists() {
            println!(
                "Files {} and {} have been removed",
                self.config_path.display(),
                self.cache_path.display()
            );
            return;
        }
        println!("Removing file {}", &self.config_path.display());
        let output = process::Command::new("rm").arg(&self.config_path).output();
        match output {
            Ok(output) => {
                if !output.status.success() {
                    println!(
                        "Failed to remove config: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => println!("Failed to run rm {}", e),
        }

        println!("Removing file {}", &self.cache_path.display());
        let output = process::Command::new("rm").arg(&self.cache_path).output();
        match output {
            Ok(output) => {
                if !output.status.success() {
                    println!(
                        "Failed to remove config: {}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => println!("Failed to run rm {}", e),
        }
        self.reload();
    }

    pub fn set_config(&self) {
        let path = &self.config_path;
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
        self.reload();
    }

    pub fn get_config(&self) -> String {
        let path = &self.config_path;
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => panic!("Error opening colors-spicetify.ini! {}", e),
        };

        println!("wal config path: {}", path.display());

        let mut reader = BufReader::new(file);
        let mut wal_config = String::new();
        let _ = reader.read_to_string(&mut wal_config);
        wal_config
    }
}
