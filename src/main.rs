use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Stolen initially from https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

fn main() {
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];

    let mut wordcount: usize = 0;
    let mut linecount: usize = 0;
    let mut bytecount: usize = 0;

    if let Ok(lines) = read_lines(fp) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            linecount = linecount + 1;
            if let Ok(s) = line {
                let this_bytes: usize = s.len();
                let this_words: usize = s
                    .split_terminator(|c: char| c.is_whitespace())
                    .filter(|&x| x.len() >= 1)
                    .count();
                wordcount = this_words + wordcount;
                bytecount = this_bytes + bytecount + 1; // 1 for the new line
            }
        }
    }

    println!("Lines: {}", linecount);
    println!("wordcount: {}", wordcount);
    println!("bytecount: {}", bytecount);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
