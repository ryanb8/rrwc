use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WcResult {
    input_path: String,
    linecount: usize,
    wordcount: usize,
    bytecount: usize,
}

pub fn wc_naive(fp: &String) -> WcResult {
    let mut wordcount: usize = 0;
    let mut linecount: usize = 0;
    let mut bytecount: usize = 0;

    if let Ok(lines) = read_lines(fp) {
        for line in lines {
            if let Ok(s) = line {
                let t = naive_basic_line_count(s);
                linecount = t.0 + linecount;
                wordcount = t.1 + wordcount;
                bytecount = t.2 + bytecount;
            }
        }
    }

    WcResult {
        input_path: fp.to_string(),
        linecount,
        wordcount,
        bytecount,
    }
}

pub fn wc_naive_rayon(fp: &String) -> WcResult {
    let mut t: (usize, usize, usize) = (0, 0, 0);
    if let Ok(lines) = read_lines(fp) {
        let ts: Vec<(usize, usize, usize)> = lines
            .par_bridge()
            .map(|l| naive_basic_line_count(l.unwrap()))
            .collect();

        t = ts.into_par_iter().reduce(
            || (0usize, 0usize, 0usize),
            |a: (usize, usize, usize), b: (usize, usize, usize)| (a.0 + b.0, a.1 + b.1, a.2 + b.2),
        );
    };

    WcResult {
        input_path: fp.to_string(),
        linecount: t.0,
        wordcount: t.1,
        bytecount: t.2,
    }
}

fn naive_basic_line_count(l: String) -> (usize, usize, usize) {
    let this_bytes: usize = l.len() + 1;
    let this_words: usize = l
        .split_terminator(|c: char| c.is_whitespace())
        .filter(|&x| x.len() >= 1)
        .count();
    return (1, this_words, this_bytes);
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
