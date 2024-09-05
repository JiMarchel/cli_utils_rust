use mycat::Config;
use std::{
    fs::File,
    io::{self, Read},
};

pub fn mycat(config: &Config) -> io::Result<()> {
    let mut file = File::open(config.file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    config.process(&contents);
    Ok(())
}
