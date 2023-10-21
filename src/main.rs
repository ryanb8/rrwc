use ryan_wc::misfit_toys::{
    wc_low_level_custom_buffer, wc_low_level_full_file, wc_naive_full_file,
    wc_naive_full_file_via_buf, wc_naive_rayon, wc_naive_rayon_big_buf,
};
use ryan_wc::{wc_low_level_buf_reader, wc_naive};

use ryan_wc::WcResult;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];
    let ver: &String = &args[2];

    let wc_result: WcResult = match ver.as_str() {
        "naive" => wc_naive(fp),
        "low_level_buf_reader" => wc_low_level_buf_reader(fp),
        // misfit toys
        "low_level_custom_buffer" => wc_low_level_custom_buffer(fp),
        "low_level_full_file" => wc_low_level_full_file(fp),
        "naive_rayon" => wc_naive_rayon(fp),
        "naive_rayon_big_buf" => wc_naive_rayon_big_buf(fp),
        "naive_full_file" => wc_naive_full_file(fp),
        "full_file_via_buf" => wc_naive_full_file_via_buf(fp),
        _ => panic!(
            "Must use value in ['naive', 'low_level_buf_reader', 'low_level_full_file', 'low_level_custom_buffer', 'naive_rayon', 'naive_rayon_big_buf', 'naive_full_file', 'full_file_via_buf']"
        ),
    };

    println!("Using wc version: {}", ver);
    println!("{:#?}", wc_result)
}
