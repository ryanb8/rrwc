use crate::WcResult;
use crate::{char_count, naive_basic_line_count_by_ref, read_lines, BUFFER_SIZE};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, Read, Seek, SeekFrom};
use std::path::Path;
use std::str::{from_utf8, from_utf8_unchecked};

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

pub fn wc_low_level_full_file(fp: &String) -> WcResult {
    let mut f = File::open(fp).unwrap();
    let mut text = String::new();
    let mut prior_char_is_ws: bool = false;

    let _ = f.read_to_string(&mut text);

    let lc_wc_bc = char_count(text.chars(), &mut prior_char_is_ws, true);

    WcResult {
        input_path: fp.to_string(),
        linecount: lc_wc_bc[0],
        wordcount: lc_wc_bc[1],
        bytecount: lc_wc_bc[2],
    }
}

pub fn wc_low_level_custom_buffer(fp: &String) -> WcResult {
    let mut f = File::open(fp).unwrap();
    let mut buffer = [0; BUFFER_SIZE];

    let mut lc_wc_bc: [usize; 3] = [0, 0, 0];
    let mut this_lc_wc_bc: [usize; 3];
    let mut prior_char_is_ws: bool = false;
    let mut c: usize = 0;

    loop {
        let n = f.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        let text = &buffer[..n];
        let s = match from_utf8(text) {
            Ok(s) => s,
            Err(e) => {
                // println!("WE HIT THIS!");
                let end = e.valid_up_to();
                // This is safe due to the above check
                let s = unsafe { from_utf8_unchecked(&text[..end]) };
                let offset = (end - n) as i64;
                // we could also just hold onto the bytes at the start of our buffer but this is a
                // bit simpler IMO
                f.seek(SeekFrom::Current(-1 * offset)).unwrap();
                s
            }
        };
        this_lc_wc_bc = char_count(s.chars(), &mut prior_char_is_ws, c == 0);
        lc_wc_bc = [
            lc_wc_bc[0] + this_lc_wc_bc[0],
            lc_wc_bc[1] + this_lc_wc_bc[1],
            lc_wc_bc[2] + this_lc_wc_bc[2],
        ];
        c += 1;
    }
    WcResult {
        input_path: fp.to_string(),
        linecount: lc_wc_bc[0],
        wordcount: lc_wc_bc[1],
        bytecount: lc_wc_bc[2],
    }
}
