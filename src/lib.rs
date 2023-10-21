use std::fs::File;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::str::{from_utf8, from_utf8_unchecked, Chars};
pub mod misfit_toys;

const BUFFER_SIZE: usize = 8 * 1024;

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

pub fn wc_low_level_buf_reader(fp: &String) -> WcResult {
    let mut buf = [0u8; BUFFER_SIZE];
    // println!("starting");
    let mut lc_wc_bc: [usize; 3] = [0, 0, 0];
    let mut this_lc_wc_bc: [usize; 3];
    let mut c = 0usize;
    let mut prior_char_is_ws: bool = false;
    if let Ok(mut br) = read_buffer(fp) {
        loop {
            let n = br.read(&mut buf).unwrap();
            // println!("Read {} bytes for count {}", n, c);
            if n == 0 {
                break;
            }
            let text: &[u8; BUFFER_SIZE] = &buf;
            let s = match from_utf8(&text[..n]) {
                Ok(s) => s,
                Err(e) => {
                    // println!("WE ARE HERE");
                    let end = e.valid_up_to();
                    // This is safe due to the above check
                    let s = unsafe { from_utf8_unchecked(&text[..end]) };
                    let offset = (n - end) as i64;
                    // we could also just hold onto the bytes at the start of our buffer but this is a
                    // bit simpler IMO
                    br.seek_relative(-1 * offset).unwrap();
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
    }

    WcResult {
        input_path: fp.to_string(),
        linecount: lc_wc_bc[0],
        wordcount: lc_wc_bc[1],
        bytecount: lc_wc_bc[2],
    }
}

fn read_buffer<P>(filename: P) -> io::Result<io::BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file))
}

fn char_count(cs: Chars<'_>, prior_char_is_ws: &mut bool, first_global_batch: bool) -> [usize; 3] {
    let mut lc_wc_bc: [usize; 3] = [0, 0, 0];
    let mut i: usize = 0;

    for c in cs {
        lc_wc_bc[2] += c.len_utf8();
        // let enter_prior_char_is_ws = prior_char_is_ws.clone();

        // checking for ws/non-ws is way more time intensive if it's Non-ascii
        // we handle ascii first
        if c.is_ascii() && !c.is_ascii_whitespace() {
            *prior_char_is_ws = false;
        } else if c.is_ascii_whitespace() {
            // WC only counts \n as new character - I think. There's A lot of code in wc.
            // https://github.com/coreutils/coreutils/blob/master/src/wc.c#L492
            if c == '\u{000A}' {
                lc_wc_bc[0] += 1;
            }
            if !*prior_char_is_ws && !(i == 0 && first_global_batch) {
                lc_wc_bc[1] += 1;
            }
            *prior_char_is_ws = true;
        } else if c.is_whitespace() {
            if !*prior_char_is_ws && !(i == 0 && first_global_batch) {
                lc_wc_bc[1] += 1;
            }
            *prior_char_is_ws = true;
        } else if *prior_char_is_ws {
            *prior_char_is_ws = false;
        }
        i += 1;
        // println!(
        //     "c: `{c}`, current wc: {},  enter_pib: {enter_prior_char_is_ws}",
        //     lc_wc_bc[1]
        // );
    }
    return lc_wc_bc;
}
