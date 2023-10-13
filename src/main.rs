use ryan_wc::wc_naive;
use ryan_wc::wc_naive_rayon;
use ryan_wc::WcResult;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let fp = &args[1];
    let ver: &String = &args[2];

    let wc_result: WcResult = match ver.as_str() {
        "naive" => wc_naive(fp),
        "naive_rayon" => wc_naive_rayon(fp),
        _ => panic!("Must use value in ['naive', 'naive_rayon']"),
    };

    println!("Using wc version: {}", ver);
    println!("{:#?}", wc_result)
}
