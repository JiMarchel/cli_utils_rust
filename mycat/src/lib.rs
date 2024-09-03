use std::{
    fs::File,
    io::{self, Read},
};

pub fn cat(file_name: &str) -> io::Result<()> {
    //Open file!
    let mut file = File::open(file_name)?;

    //Read file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Display file content to terminal
    println!("{}", contents);

    Ok(())
}
