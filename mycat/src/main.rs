use clap::{Arg, ArgAction, Command};
use mycat::Config;
pub mod cat;
use cat::mycat;

fn main() -> std::io::Result<()> {
    let matches = Command::new("mycat")
        .arg(Arg::new("file").help("File to display").required(true))
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .help("number all output lines")
                .conflicts_with("number-nonblank")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number-nonblank")
                .short('b')
                .long("number-nonblank")
                .help("number nonempty output lines, overrides -n")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("show-ends")
                .short('E')
                .long("show-ends")
                .help("display $ at end of each line")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("squeeze-blank")
                .short('s')
                .long("squeeze-blank")
                .help("suppress repeated empty output lines")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let config = Config {
        file_name: matches.get_one::<String>("file").unwrap(),
        number: matches.get_flag("number"),
        number_nonblank: matches.get_flag("number-nonblank"),
        show_ends: matches.get_flag("show-ends"),
        squeeze_blank: matches.get_flag("squeeze-blank"),
    };

    if let Err(e) = mycat(&config) {
        eprintln!("Error reading file: {}", e);
    }

    Ok(())
}
