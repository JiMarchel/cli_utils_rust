use clap::{Arg, Command};
use std::io;

fn main() -> io::Result<()> {
    let cfg = Command::new("mycat")
        .arg(
            Arg::new("file")
                .help("File to display")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("number all output lines"),
        )
        .get_matches();

    let file_name: &str = cfg.get_one::<String>("file").unwrap();

    if let Err(e) = mycat::cat(file_name) {
        eprintln!("Error reading file : {}", e)
    }

    Ok(())
}
