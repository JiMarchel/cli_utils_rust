use std::{
    env,
    fs::File,
    io::{self, Read},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: my_cat <file_name>");
        std::process::exit(1);
    }

    for file_name in &args[1..] {
        if let Err(e) = cat(file_name) {
            eprintln!("Error reading {} : {}", file_name, e)
        }
    }

    Ok(())
}

fn cat(file_name: &str) -> io::Result<()> {
    //Open file!
    let mut file = File::open(file_name)?;

    //Read file
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    //Display file content to terminal
    println!("{}", contents);

    Ok(())
}
