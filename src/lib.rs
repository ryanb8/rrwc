use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, Read};
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

pub fn wc_naive_full_file(fp: &String) -> WcResult {
    let mut f = File::open(fp).unwrap();
    let mut text = String::new();

    let _ = f.read_to_string(&mut text);

    let t = naive_basic_line_count(text);

    WcResult {
        input_path: fp.to_string(),
        linecount: t.0,
        wordcount: t.1,
        bytecount: t.2,
    }
}

pub fn wc_naive_full_file_via_buf(fp: &String) -> WcResult {
    let lines: Vec<String> = read_lines(fp).unwrap().map(|l| l.unwrap()).collect();
    let t: (usize, usize, usize) = lines
        .iter()
        .map(|l| naive_basic_line_count_by_ref(l))
        .reduce(|a: (usize, usize, usize), b: (usize, usize, usize)| {
            (a.0 + b.0, a.1 + b.1, a.2 + b.2)
        })
        .unwrap();

    WcResult {
        input_path: fp.to_string(),
        linecount: t.0,
        wordcount: t.1,
        bytecount: t.2,
    }
}

// pub fn wc_full_file_rayon(fp: &String) -> WcResult {
//     let mut f = File::open(fp).unwrap();
//     let mut text = String::new();

//     let _ = f.read_to_string(&mut text);

//     t = text.chars().

// }

pub fn wc_naive_rayon(fp: &String) -> WcResult {
    let mut t: (usize, usize, usize) = (0, 0, 0);
    if let Ok(lines) = read_lines(fp) {
        t = lines
            .par_bridge()
            .map(|l| naive_basic_line_count(l.unwrap()))
            // .into_par_iter()
            .reduce(
                || (0usize, 0usize, 0usize),
                |a: (usize, usize, usize), b: (usize, usize, usize)| {
                    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
                },
            );
    };

    WcResult {
        input_path: fp.to_string(),
        linecount: t.0,
        wordcount: t.1,
        bytecount: t.2,
    }
}

pub fn wc_naive_rayon_big_buf(fp: &String) -> WcResult {
    let mut t: (usize, usize, usize) = (0, 0, 0);
    if let Ok(lines) = read_lines_big_buf(fp, 10000000) {
        t = lines
            .par_bridge()
            .map(|l| naive_basic_line_count(l.unwrap()))
            // .into_par_iter()
            .reduce(
                || (0usize, 0usize, 0usize),
                |a: (usize, usize, usize), b: (usize, usize, usize)| {
                    (a.0 + b.0, a.1 + b.1, a.2 + b.2)
                },
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

fn naive_basic_line_count_by_ref(l: &String) -> (usize, usize, usize) {
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

fn read_lines_big_buf<P>(filename: P, capacity: usize) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::with_capacity(capacity, file).lines())
}
