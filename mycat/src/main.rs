use clap::{Arg, ArgAction, Command};
use mycat::Config;
use std::io;

fn main() -> io::Result<()> {
    let cfg = Command::new("mycat")
        .arg(Arg::new("file").help("File to display").required(true))
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("number all output lines")
                .conflicts_with_all(["number-nonblank", "show-ends"])
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n")
                .conflicts_with("show-ends")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-ends")
                .short('E')
                .long("show-ends")
                .help("display $ at end of each line")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let config = Config {
        file_name: cfg.get_one::<String>("file").unwrap().clone(),
        number: cfg.get_flag("number"),
        number_nonblank: cfg.get_flag("number-nonblank"),
        show_ends: cfg.get_flag("show-ends"),
    };

    if let Err(e) = mycat::cat(config) {
        eprintln!("Error reading file : {}", e)
    }

    Ok(())
}
