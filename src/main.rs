use ryan_wc::wc_naive;
use ryan_wc::wc_naive_rayon;
use ryan_wc::wc_naive_rayon_big_buf;
use ryan_wc::WcResult;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];
    let ver: &String = &args[2];

    let wc_result: WcResult = match ver.as_str() {
        "naive" => wc_naive(fp),
        "naive_rayon" => wc_naive_rayon(fp),
        "naive_rayon_big_buf" => wc_naive_rayon_big_buf(fp),
        _ => panic!("Must use value in ['naive', 'naive_rayon', 'naive_rayon_big_buf']"),
    };

    println!("Using wc version: {}", ver);
    println!("{:#?}", wc_result)
}
