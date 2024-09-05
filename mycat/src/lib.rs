#[derive(Debug)]
pub struct Config<'a> {
    pub file_name: &'a str,
    pub number: bool,
    pub number_nonblank: bool,
    pub show_ends: bool,
    pub squeeze_blank: bool,
}

impl<'a> Config<'a> {
    pub fn process(&self, contents: &str) {
        let mut previous_was_blank = false;
        let mut line_count = 0;

        for line in contents.lines() {
            let is_blank = line.trim().is_empty();

            if self.squeeze_blank && is_blank && previous_was_blank {
                continue;
            }

            if (self.number_nonblank && !is_blank) || self.number {
                line_count += 1;
                print!("{:>6} ", line_count);
            }

            if self.show_ends {
                println!("{}$", line);
            } else {
                println!("{}", line);
            }

            previous_was_blank = is_blank;
        }
    }
}
