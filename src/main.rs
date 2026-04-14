mod spicetify;
mod wal;

use clap::{Arg, Command};
use std::env;

use spicetify::Spicetify;
use wal::Wal;

fn main() {
    let matches = Command::new("pywal-spicetify")
        .version("0.1")
        .about("A simple cli tool for setting spicetify colors from wal")
        .arg(Arg::new("theme").required(true))
        .arg(
            Arg::new("reset")
                .short('r')
                .long("reset")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("no-restart")
                .short('n')
                .long("no-restart")
                .action(clap::ArgAction::SetTrue)
                .help("Don't restart Spotify if it's not running"),
        )
        .arg_required_else_help(true)
        .get_matches();

    let theme: String = matches
        .get_one::<String>("theme")
        .expect("theme required")
        .to_string();

    println!("You selected the theme: {theme}");

    //Only linux support
    let home = match env::home_dir() {
        Some(path) => path,
        None => panic!("unable to locate home directory!"),
    };

    let wal = Wal::new(home.clone());
    let spicetify = Spicetify::new(home.clone(), &theme);
    let use_no_restart = matches.get_flag("no-restart");

    if matches.get_flag("reset") {
        println!("Resetting configs...");
        wal.reset();
        spicetify.write_config(None);
        spicetify.reload(use_no_restart);
    } else {
        wal.set_config();
        let wal_config = wal.get_config();

        spicetify.write_config(Some(wal_config));
        spicetify.reload(use_no_restart);
    }
}
