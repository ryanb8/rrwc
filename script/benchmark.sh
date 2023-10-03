#!/usr/bin/env bash

mkdir benchmark_output
benchmark_run=$(date +%Y%m%d_%H%M%S)
benchmark_folder=benchmark_output/"$benchmark_run"
mkdir "$benchmark_folder"

echo "Moby Dick"
hyperfine --warmup 100  -N --export-json "$benchmark_folder"/moby_dick.json 'wc benchmark_texts/moby_dick.txt' './target/release/ryan_wc benchmark_texts/moby_dick.txt'

echo "KJV 1x"
hyperfine --warmup 10  -N --export-json "$benchmark_folder"/kjv_1x.json 'wc benchmark_texts/kjv_1x.txt' './target/release/ryan_wc benchmark_texts/kjv_1x.txt'

echo "KJV 10x"
hyperfine --warmup 10  -N --export-json "$benchmark_folder"/kjv_10x.json 'wc benchmark_texts/kjv_10x.txt' './target/release/ryan_wc benchmark_texts/kjv_10x.txt'

echo "KJV 1x"
hyperfine --warmup 10  -N --export-json "$benchmark_folder"/kjv_100x.json 'wc benchmark_texts/kjv_100x.txt' './target/release/ryan_wc benchmark_texts/kjv_100x.txt'
