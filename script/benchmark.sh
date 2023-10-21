#!/usr/bin/env bash

# TODO: Script to accept two different versions (WC or any of the versions in the rust binary)

mkdir benchmark_output
benchmark_run=$(date +%Y%m%d_%H%M%S)
benchmark_folder=benchmark_output/"$benchmark_run"
mkdir "$benchmark_folder"

echo "Moby Dick"
hyperfine --warmup 100 --export-json "$benchmark_folder"/moby_dick.json 'wc benchmark_texts/moby_dick.txt' './target/release/ryan_wc benchmark_texts/moby_dick.txt low_level_full_file'

echo "KJV 1x"
hyperfine --warmup 10 --export-json "$benchmark_folder"/kjv_1x.json 'wc benchmark_texts/kjv_1x.txt' './target/release/ryan_wc benchmark_texts/kjv_1x.txt low_level_full_file'

echo "KJV 10x"
hyperfine --warmup 10 --export-json "$benchmark_folder"/kjv_10x.json 'wc benchmark_texts/kjv_10x.txt' './target/release/ryan_wc benchmark_texts/kjv_10x.txt low_level_full_file'

echo "KJV 100x"
hyperfine --warmup 10 --export-json "$benchmark_folder"/kjv_100x.json 'wc benchmark_texts/kjv_100x.txt' './target/release/ryan_wc benchmark_texts/kjv_100x.txt low_level_full_file'
