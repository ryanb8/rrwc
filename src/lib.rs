use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
pub mod misfit_toys;

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
                let t = naive_basic_line_count_by_ref(&s);
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

pub fn wc_low_level_full_file(fp: &String) -> WcResult {
    let mut f = File::open(fp).unwrap();
    let mut text = String::new();

    let _ = f.read_to_string(&mut text);

    let mut prior_char_is_ws: bool = false;
    let mut lc: usize = 0;
    let mut wc: usize = 0;
    let mut bc: usize = 0;

    for c in text.chars() {
        bc += c.len_utf8();

        // checking for ws/non-ws is way more time intensive if it's Non-ascii
        // we handle ascii first
        if c.is_ascii() && !c.is_ascii_whitespace() {
            prior_char_is_ws = false;
        } else if c.is_ascii_whitespace() {
            // WC only counts \n as new character - I think. There's A lot of code in wc.
            // https://github.com/coreutils/coreutils/blob/master/src/wc.c#L492
            if c == '\u{000A}' {
                lc += 1;
            }
            if !prior_char_is_ws && !(bc == 1) {
                wc += 1;
            }
            prior_char_is_ws = true;
        } else if c.is_whitespace() {
            if !prior_char_is_ws && !(bc == 1) {
                wc += 1;
            }
            prior_char_is_ws = true;
        } else if prior_char_is_ws {
            prior_char_is_ws = false;
        }
    }

    WcResult {
        input_path: fp.to_string(),
        linecount: lc,
        wordcount: wc,
        bytecount: bc,
    }
}
