use std::{
    fs::File,
    io::{self, Read},
};

pub struct Config {
    pub file_name: String,
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
}
//TODO:squeeze-blank, -s
impl Config {
    pub fn number(contents: &str) {
        for (i, line) in contents.lines().enumerate() {
            println!("{:>6}  {}", i + 1, line)
        }
    }

    pub fn number_nonblank(contents: &str) {
        for (i, line) in contents.lines().enumerate() {
            if line.trim().is_empty() {
                println!("{}", line)
            } else {
                println!("{:>6} {}", i + 1, line)
            }
        }
    }

    pub fn show_ends(contents: &str) {
        for line in contents.lines() {
            println!("{}$", line)
        }
    }
}

pub fn cat(config: Config) -> io::Result<()> {
    let mut file = File::open(config.file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    match (config.number, config.number_nonblank, config.show_ends) {
        (true, false, false) => Config::number(&contents),
        (false, true, false) => Config::number_nonblank(&contents),
        (false, false, true) => Config::show_ends(&contents),
        (_, _, _) => println!("{}", contents),
    }

    Ok(())
}
