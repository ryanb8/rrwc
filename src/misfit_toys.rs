use crate::naive_basic_line_count_by_ref;
use crate::read_lines;
use crate::WcResult;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;

pub fn wc_naive_full_file(fp: &String) -> WcResult {
    let mut f = File::open(fp).unwrap();
    let mut text = String::new();

    let _ = f.read_to_string(&mut text);

    let t = naive_basic_line_count_by_ref(&text);

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

pub fn wc_naive_rayon(fp: &String) -> WcResult {
    let mut t: (usize, usize, usize) = (0, 0, 0);
    if let Ok(lines) = read_lines(fp) {
        t = lines
            .par_bridge()
            .map(|l| naive_basic_line_count_by_ref(&l.unwrap()))
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
            .map(|l| naive_basic_line_count_by_ref(&l.unwrap()))
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

fn read_lines_big_buf<P>(filename: P, capacity: usize) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::with_capacity(capacity, file).lines())
}
